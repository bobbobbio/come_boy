// Copyright 2017 Remi Bernotavicius

pub use self::game_pak::GamePak;
use self::joypad::{ControllerJoyPad, JoyPad, PlaybackJoyPad, RecordingJoyPad};
use self::lcd_controller::{LcdController, OAM_DATA};
use self::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyMemoryMap, GameBoyMemoryMapMut, GameBoyRegister, MemoryChunk,
    MemoryMappedHardware,
};
use self::sound_controller::SoundController;
use crate::emulator_common::disassembler::MemoryAccessor;
use crate::game_boy_emulator::joypad::KeyEvent;
use crate::lr35902_emulator::{Intel8080Register, LR35902Emulator, LR35902Flag};
use crate::rendering::{Keycode, Renderer};
use crate::util::{super_fast_hash, Scheduler};
use enum_iterator::IntoEnumIterator;
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Range, RangeFrom};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fmt, mem};

pub use self::debugger::run_debugger;
pub use self::disassembler::disassemble_game_boy_rom;

#[macro_use]
mod memory_controller;

mod debugger;

mod disassembler;
mod game_pak;
mod joypad;
mod lcd_controller;
mod sound_controller;
mod tandem;

/// This is how many ticks of the emulator we do before under-clocking and checking for input
const SLEEP_INPUT_TICKS: u64 = 10_000;

struct ModuloCounter {
    counter: u64,
    every: u64,
}

impl ModuloCounter {
    fn new(every: u64) -> Self {
        Self { counter: 0, every }
    }

    fn incr(&mut self) -> bool {
        self.counter = self.counter.wrapping_add(1);
        self.counter % self.every == 0
    }
}

struct Underclocker {
    last_cycles: u64,
    last_instant: Instant,
}

impl Underclocker {
    fn new(now: u64) -> Self {
        Self {
            last_cycles: now,
            last_instant: Instant::now(),
        }
    }

    fn underclock(&mut self, now: u64, speed: u32) {
        let elapsed_cycles = now - self.last_cycles;

        let delay = Duration::from_secs(1) / speed;
        let expected_time = (elapsed_cycles as u32) * delay;

        // If we didn't take long enough, sleep the difference.
        if let Some(sleep_time) = expected_time.checked_sub(self.last_instant.elapsed()) {
            std::thread::sleep(sleep_time);
        }

        self.last_cycles = now;
        self.last_instant = std::time::Instant::now();
    }
}

#[derive(Debug, Copy, Clone)]
enum UserControl {
    SaveStateLoaded,
    ScreenClosed,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Replay(joypad::ReplayError),
    Serde(bincode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<joypad::ReplayError> for Error {
    fn from(e: joypad::ReplayError) -> Self {
        Self::Replay(e)
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::Serde(e)
    }
}

/*   ____                      ____              _____                 _       _
 *  / ___| __ _ _ __ ___   ___| __ )  ___  _   _| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * | |  _ / _` | '_ ` _ \ / _ \  _ \ / _ \| | | |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |_| | (_| | | | | | |  __/ |_) | (_) | |_| | |___| | | | | | |_| | | (_| | || (_) | |
 *  \____|\__,_|_| |_| |_|\___|____/ \___/ \__, |_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 *                                         |___/
 */

const ALL_INTERRUPTS: [(InterruptFlag, u16); 5] = [
    (InterruptFlag::VerticalBlanking, 0x0040),
    (InterruptFlag::LCDSTAT, 0x0048),
    (InterruptFlag::Timer, 0x0050),
    (InterruptFlag::Serial, 0x0058),
    (InterruptFlag::Joypad, 0x0060),
];

/// This mask represents the various interrupts the LcdController handles.
#[derive(Debug, Clone, Copy, PartialEq, ReprFrom, IntoEnumIterator)]
#[repr(u8)]
pub enum InterruptFlag {
    VerticalBlanking = 0b00000001,
    LCDSTAT = 0b00000010,
    Timer = 0b00000100,
    Serial = 0b00001000,
    Joypad = 0b00010000,
}

impl FlagMask for InterruptFlag {
    fn read_mask() -> u8 {
        InterruptFlag::VerticalBlanking as u8
            | InterruptFlag::LCDSTAT as u8
            | InterruptFlag::Timer as u8
            | InterruptFlag::Serial as u8
            | InterruptFlag::Joypad as u8
    }

    fn write_mask() -> u8 {
        InterruptFlag::VerticalBlanking as u8
            | InterruptFlag::LCDSTAT as u8
            | InterruptFlag::Timer as u8
            | InterruptFlag::Serial as u8
            | InterruptFlag::Joypad as u8
    }
}

impl From<InterruptEnableFlag> for InterruptFlag {
    fn from(f: InterruptEnableFlag) -> Self {
        match f {
            InterruptEnableFlag::VerticalBlanking => InterruptFlag::VerticalBlanking,
            InterruptEnableFlag::LCDSTAT => InterruptFlag::LCDSTAT,
            InterruptEnableFlag::Timer => InterruptFlag::Timer,
            InterruptEnableFlag::Serial => InterruptFlag::Serial,
            InterruptEnableFlag::Joypad => InterruptFlag::Joypad,
        }
    }
}

/// This mask represent the various interrupts that the program can enable.
#[derive(Debug, Clone, Copy, PartialEq, ReprFrom, IntoEnumIterator)]
#[repr(u8)]
pub enum InterruptEnableFlag {
    VerticalBlanking = 0b00000001,
    LCDSTAT = 0b00000010,
    Timer = 0b00000100,
    Serial = 0b00001000,
    Joypad = 0b00010000,
}

impl FlagMask for InterruptEnableFlag {
    fn read_mask() -> u8 {
        0xFF
    }

    fn write_mask() -> u8 {
        0xFF
    }
}

impl From<InterruptFlag> for InterruptEnableFlag {
    fn from(f: InterruptFlag) -> Self {
        match f {
            InterruptFlag::VerticalBlanking => InterruptEnableFlag::VerticalBlanking,
            InterruptFlag::LCDSTAT => InterruptEnableFlag::LCDSTAT,
            InterruptFlag::Timer => InterruptEnableFlag::Timer,
            InterruptFlag::Serial => InterruptEnableFlag::Serial,
            InterruptFlag::Joypad => InterruptEnableFlag::Joypad,
        }
    }
}

const HIGH_RAM: Range<u16> = Range {
    start: 0xFF80,
    end: 0xFFFF,
};

const INTERNAL_RAM_A: Range<u16> = Range {
    start: 0xC000,
    end: 0xDE00,
};

const INTERNAL_RAM_B: Range<u16> = Range {
    start: 0xDE00,
    end: 0xE000,
};

#[derive(Serialize, Deserialize, Default)]
struct Divider(GameBoyRegister);

impl fmt::Debug for Divider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Divider {
    fn set_state_post_bios(&mut self) {
        self.0.set_value(0xab);
    }
    fn increment(&mut self) {
        self.0.add(1)
    }
}

impl MemoryMappedHardware for Divider {
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.0.read_value()
    }

    fn set_value(&mut self, address: u16, _value: u8) {
        assert_eq!(address, 0);
        self.0.set_value(0);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct GameBoyRegisters {
    interrupt_flag: GameBoyFlags<InterruptFlag>,
    interrupt_enable: GameBoyFlags<InterruptEnableFlag>,

    serial_transfer_data: GameBoyRegister,
    serial_transfer_control: GameBoyRegister,
    divider: Divider,
}

#[derive(Default, Serialize, Deserialize)]
struct TimerControl {
    value: GameBoyFlags<TimerFlags>,
    timer_restart_requested: bool,
}

impl fmt::Debug for TimerControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl TimerControl {
    fn read_flag(&self, flag: TimerFlags) -> bool {
        self.value.read_flag(flag)
    }

    fn read_flag_value(&self, flag: TimerFlags) -> u8 {
        self.value.read_flag_value(flag)
    }

    fn set_value(&mut self, value: u8) {
        self.value.set_value(value);
    }
}

impl MemoryMappedHardware for TimerControl {
    fn read_value(&self, address: u16) -> u8 {
        MemoryMappedHardware::read_value(&self.value, address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.timer_restart_requested = true;
        MemoryMappedHardware::set_value(&mut self.value, address, value)
    }
}

#[derive(Default, Serialize, Deserialize)]
struct GameBoyTimer {
    counter: GameBoyRegister,
    modulo: GameBoyRegister,
    control: TimerControl,
    scheduler: Scheduler<()>,
    interrupt_requested: bool,
    running: bool,
}

impl fmt::Debug for GameBoyTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoyTimer")
            .field("counter", &self.counter)
            .field("modulo", &self.modulo)
            .field("control", &self.control)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, ReprFrom, IntoEnumIterator)]
#[repr(u8)]
enum TimerFlags {
    Enabled = 0b00000100,
    Speed = 0b00000011,
}

impl FlagMask for TimerFlags {
    fn read_mask() -> u8 {
        TimerFlags::Enabled as u8 | TimerFlags::Speed as u8
    }

    fn write_mask() -> u8 {
        TimerFlags::Enabled as u8 | TimerFlags::Speed as u8
    }
}

impl GameBoyTimer {
    fn enabled(&self) -> bool {
        self.control.read_flag(TimerFlags::Enabled)
    }

    fn timer_speed(&self) -> u64 {
        match self.control.read_flag_value(TimerFlags::Speed) {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => panic!(),
        }
    }

    fn set_state_post_bios(&mut self) {
        self.counter.set_value(0x0);
        self.modulo.set_value(0x0);
        self.control.set_value(0xf8);
    }

    fn schedule_initial_events(&mut self, now: u64) {
        let speed = self.timer_speed();
        self.scheduler.schedule(now + speed, ());
    }

    fn fire(&mut self, now: u64) {
        let counter = self.counter.read_value().wrapping_add(1);
        if counter == 0 {
            self.interrupt_requested = true;
            let modulo_value = self.modulo.read_value();
            self.counter.set_value(modulo_value);
        } else {
            self.counter.set_value(counter);
        }
        let speed = self.timer_speed();
        self.scheduler.schedule(now + speed, ());
    }

    fn tick(&mut self, now: u64) {
        if self.control.timer_restart_requested {
            self.scheduler.drop_events();
            if self.enabled() {
                let speed = self.timer_speed();
                self.scheduler.schedule(now + speed, ());
            }
            self.control.timer_restart_requested = false;
        }
    }

    fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.interrupt_requested {
            interrupt_flag.set_flag(InterruptFlag::Timer, true);
            self.interrupt_requested = false;
        }
    }

    fn deliver_events(&mut self, now: u64) {
        while let Some((time, ())) = self.scheduler.poll(now) {
            self.fire(time);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum GameBoyEmulatorEvent {
    DividerTick,
    DriveJoypad,
}

impl GameBoyEmulatorEvent {
    fn deliver(self, emulator: &mut GameBoyEmulator, time: u64) {
        match self {
            Self::DividerTick => emulator.divider_tick(time),
            Self::DriveJoypad => emulator.drive_joypad(time),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DmaTransfer {
    src_current: u16,
    dst_end: u16,
    dst_current: u16,
}

impl DmaTransfer {
    fn new(dst: Range<u16>, src: RangeFrom<u16>) -> Self {
        Self {
            src_current: src.start,
            dst_end: dst.end,
            dst_current: dst.start,
        }
    }

    fn tick(&mut self, memory_map: &mut GameBoyMemoryMapMut, cpu: &mut LR35902Emulator) {
        let value = memory_map.read_memory(self.src_current);
        memory_map.set_memory(self.dst_current, value);
        cpu.add_cycles(1);

        self.src_current = self.src_current.wrapping_add(1);
        self.dst_current = self.dst_current.wrapping_add(1);
    }

    fn is_done(&self) -> bool {
        self.dst_current == self.dst_end
    }
}

fn default_clock_speed_hz() -> u32 {
    // GameBoy clock speed is about 4.19Mhz
    4_194_304
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GameBoyEmulator {
    cpu: LR35902Emulator,
    sound_controller: SoundController,
    lcd_controller: LcdController,
    high_ram: MemoryChunk,
    internal_ram_a: MemoryChunk,
    internal_ram_b: MemoryChunk,

    registers: GameBoyRegisters,
    scheduler: Scheduler<GameBoyEmulatorEvent>,
    timer: GameBoyTimer,

    dma_transfer: Option<DmaTransfer>,

    #[serde(skip, default = "default_clock_speed_hz")]
    clock_speed_hz: u32,

    #[serde(skip)]
    game_pak: Option<GamePak>,

    joypad_key_events: Vec<KeyEvent>,

    #[serde(skip)]
    joypad: Option<Box<dyn JoyPad>>,
}

impl GameBoyEmulator {
    fn new() -> Self {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(),
            lcd_controller: LcdController::new(),
            sound_controller: Default::default(),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            internal_ram_a: MemoryChunk::from_range(INTERNAL_RAM_A),
            internal_ram_b: MemoryChunk::from_range(INTERNAL_RAM_B),
            registers: Default::default(),
            scheduler: Scheduler::new(),
            timer: Default::default(),
            dma_transfer: None,
            clock_speed_hz: default_clock_speed_hz(),
            game_pak: None,
            joypad_key_events: vec![],
            joypad: None,
        };
        e.set_state_post_bios();
        e.schedule_initial_events();
        e
    }

    fn plug_in_joy_pad<J: JoyPad + 'static>(&mut self, joypad: J) {
        self.joypad = Some(Box::new(joypad) as Box<dyn JoyPad>)
    }

    fn load_game_pak(&mut self, game_pak: GamePak) {
        println!("Loading {:?}", &game_pak);
        self.game_pak = Some(game_pak);
    }

    fn crashed(&self) -> Option<&String> {
        if self.cpu.crashed() {
            self.cpu.crash_message.as_ref()
        } else {
            None
        }
    }

    fn divider_tick(&mut self, time: u64) {
        self.registers.divider.increment();
        self.scheduler
            .schedule(time + 256, GameBoyEmulatorEvent::DividerTick);
    }

    fn deliver_events<R: Renderer>(&mut self, renderer: &mut R, now: u64) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            event.deliver(self, time);
        }

        self.lcd_controller.deliver_events(renderer, now);
        self.timer.deliver_events(now);
    }

    fn tick<R: Renderer>(&mut self, renderer: &mut R) {
        self.cpu
            .load_instruction(&mut game_boy_memory_map_mut!(self));

        let now = self.cpu.elapsed_cycles;
        self.deliver_events(renderer, now);

        self.cpu
            .execute_instruction(&mut game_boy_memory_map_mut!(self));

        self.timer.tick(now);
        self.lcd_controller.tick(now);
        self.execute_dma();

        let now = self.cpu.elapsed_cycles;
        self.deliver_events(renderer, now);

        self.lcd_controller
            .schedule_interrupts(&mut self.registers.interrupt_flag);
        self.timer
            .schedule_interrupts(&mut self.registers.interrupt_flag);

        self.handle_interrupts();
    }

    fn read_key_events<R: Renderer>(
        &mut self,
        renderer: &mut R,
    ) -> std::result::Result<(), UserControl> {
        use crate::rendering::Event;

        for event in renderer.poll_events() {
            match event {
                Event::Quit { .. } => {
                    return Err(UserControl::ScreenClosed);
                }
                Event::KeyDown(Keycode::F2) => {
                    if let Err(e) = self.save_state_to_file() {
                        println!("Failed to create save state {:?}", e);
                    }
                }
                Event::KeyDown(Keycode::F3) => {
                    return Err(UserControl::SaveStateLoaded);
                }
                Event::KeyDown(Keycode::F4) => {
                    if self.clock_speed_hz == default_clock_speed_hz() {
                        self.clock_speed_hz = u32::MAX;
                    } else {
                        self.clock_speed_hz = default_clock_speed_hz();
                    }
                }
                Event::KeyDown(code) => self.joypad_key_events.push(KeyEvent::Down(code)),
                Event::KeyUp(code) => self.joypad_key_events.push(KeyEvent::Up(code)),
            }
        }
        Ok(())
    }

    fn drive_joypad(&mut self, time: u64) {
        if let Some(joypad) = &mut self.joypad {
            joypad.tick(time, mem::take(&mut self.joypad_key_events));
        }

        self.scheduler
            .schedule(time + 456, GameBoyEmulatorEvent::DriveJoypad);
    }

    fn execute_dma(&mut self) {
        if let Some(mut address) = self.lcd_controller.registers.dma.take_request() {
            // 0xE000 to 0xFFFF is mapped differently for DMA. It ends up just being the internal
            // ram repeated again. To account for this we just adjust the source address.
            if address >= INTERNAL_RAM_B.end {
                address -= 0x2000;
            }
            self.dma_transfer = Some(DmaTransfer::new(OAM_DATA, address..));
        }

        if let Some(transfer) = self.dma_transfer.as_mut() {
            transfer.tick(&mut game_boy_memory_map_mut!(self), &mut self.cpu);

            if transfer.is_done() {
                self.dma_transfer = None;
            }
        }
    }

    /// If the stack happens to overflow into the IO registers, it can cause weird behavior when
    /// handling interrupts.
    ///
    /// Before handling an interrupt we push the return address to the stack. If the high byte when
    /// writing that address overwrites the IE register (0xFFFF) the interrupt handling will
    /// short-circuit and jump to address 0x0000.
    fn maybe_handle_ie_push_bug(&mut self, flag: InterruptFlag) -> bool {
        let sp = self.cpu.read_register_pair(Intel8080Register::SP);

        if sp == 0xFFFE && !self.registers.interrupt_enable.read_flag(flag.into()) {
            self.cpu.jump(&mut game_boy_memory_map_mut!(self), 0x0000);
            true
        } else {
            false
        }
    }

    fn deliver_interrupt(&mut self, flag: InterruptFlag, address: u16) {
        let pc = self.cpu.read_program_counter();
        self.cpu
            .push_u16_onto_stack(&mut game_boy_memory_map_mut!(self), pc);

        if self.maybe_handle_ie_push_bug(flag) {
            return;
        }

        self.cpu.jump(&mut game_boy_memory_map_mut!(self), address);
        self.cpu.push_frame(address);

        self.registers.interrupt_flag.set_flag(flag, false);
    }

    fn maybe_deliver_interrupt(&mut self, flag: InterruptFlag, address: u16) -> bool {
        let interrupt_flag_value = self.registers.interrupt_flag.read_flag(flag);
        let interrupt_enable_value = self.registers.interrupt_enable.read_flag(flag.into());

        if interrupt_flag_value && interrupt_enable_value {
            self.cpu.resume();
            if self.cpu.get_interrupts_enabled() {
                self.deliver_interrupt(flag, address);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_interrupts(&mut self) {
        let mut interrupted = false;

        for &(flag, address) in &ALL_INTERRUPTS {
            interrupted |= self.maybe_deliver_interrupt(flag, address);
        }

        if interrupted {
            self.cpu.set_interrupts_enabled(false);
        }
    }

    fn set_state_post_bios(&mut self) {
        self.sound_controller.set_state_post_bios();
        self.lcd_controller.set_state_post_bios();

        /*
         * After running the BIOS (the part of the gameboy that shows the logo) the cpu is left in
         * a very certain state. Since this is always the case, certain games may rely on this fact
         * (and indeed often times do.)
         */
        self.cpu.set_register(Intel8080Register::A, 0x1);
        self.cpu.set_register(Intel8080Register::B, 0x0);
        self.cpu.set_register(Intel8080Register::C, 0x13);
        self.cpu.set_register(Intel8080Register::D, 0x0);
        self.cpu.set_register(Intel8080Register::E, 0xD8);
        self.cpu.set_register(Intel8080Register::H, 0x01);
        self.cpu.set_register(Intel8080Register::L, 0x4D);
        self.cpu.set_flag(LR35902Flag::Carry, true);
        self.cpu.set_flag(LR35902Flag::HalfCarry, true);
        self.cpu.set_flag(LR35902Flag::Subtract, false);
        self.cpu.set_flag(LR35902Flag::Zero, true);

        self.registers.serial_transfer_data.set_value(0x0);
        self.registers.serial_transfer_control.set_value(0x7e);

        self.registers.divider.set_state_post_bios();
        self.timer.set_state_post_bios();

        self.registers.interrupt_flag.set_value(0xe1);

        /* 10 - 26 Sound Controller */

        /* 40 - 4B LCD Controller */

        let high_ram = include_bytes!("assets/high_ram.bin");
        self.high_ram.clone_from_slice(&high_ram[..]);

        let internal_ram = include_bytes!("assets/internal_ram.bin");

        let split = (INTERNAL_RAM_A.end - INTERNAL_RAM_A.start) as usize;
        self.internal_ram_a
            .clone_from_slice(&internal_ram[0..split]);
        self.internal_ram_b.clone_from_slice(&internal_ram[split..]);

        self.registers.interrupt_enable.set_value(0x0);
    }

    fn schedule_initial_events(&mut self) {
        let now = self.cpu.elapsed_cycles;
        self.scheduler
            .schedule(now + 52, GameBoyEmulatorEvent::DividerTick);
        self.scheduler
            .schedule(now + 456, GameBoyEmulatorEvent::DriveJoypad);

        self.lcd_controller.schedule_initial_events(now);
        self.timer.schedule_initial_events(now);
    }

    fn run_inner<R: Renderer>(&mut self, renderer: &mut R) -> std::result::Result<(), UserControl> {
        let mut underclocker = Underclocker::new(self.cpu.elapsed_cycles);
        let mut sometimes = ModuloCounter::new(SLEEP_INPUT_TICKS);

        while self.crashed().is_none() {
            self.tick(renderer);

            // We can't do this every tick because it is too slow. So instead so only every so
            // often.
            if sometimes.incr() {
                underclocker.underclock(self.cpu.elapsed_cycles, self.clock_speed_hz);
                self.read_key_events(renderer)?;
            }
        }

        if self.cpu.crashed() {
            println!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
        Ok(())
    }

    fn run<R: Renderer, J: JoyPad + 'static>(&mut self, renderer: &mut R, joypad: J) {
        self.plug_in_joy_pad(joypad);
        while std::matches!(self.run_inner(renderer), Err(UserControl::SaveStateLoaded)) {
            if let Err(e) = self.load_state_from_file() {
                println!("Failed to load state {:?}", e);
            }
        }
    }

    fn write_memory(&self, w: &mut dyn Write) -> Result<()> {
        let memory_map = game_boy_memory_map!(self);
        let mut mem = [0u8; 0x10000];
        for i in 0..0x10000 {
            mem[i] = memory_map.read_memory(i as u16);
        }

        w.write(&mem)?;
        Ok(())
    }

    fn hash(&self) -> u32 {
        let memory_map = game_boy_memory_map!(self);
        let mut mem = [0u8; 0x10000 + 15];
        for i in 0..0x10000 {
            mem[i] = memory_map.read_memory(i as u16);
        }

        mem[0x10000 + 0] = self.cpu.read_register(Intel8080Register::A);
        mem[0x10000 + 1] = self.cpu.read_register(Intel8080Register::B);
        mem[0x10000 + 2] = self.cpu.read_register(Intel8080Register::C);
        mem[0x10000 + 3] = self.cpu.read_register(Intel8080Register::D);
        mem[0x10000 + 4] = self.cpu.read_register(Intel8080Register::E);
        mem[0x10000 + 5] = self.cpu.read_register(Intel8080Register::H);
        mem[0x10000 + 6] = self.cpu.read_register(Intel8080Register::L);
        mem[0x10000 + 7] = (self.cpu.read_program_counter() >> 8) as u8;
        mem[0x10000 + 8] = (self.cpu.read_program_counter() & 0xFF) as u8;
        mem[0x10000 + 9] = (self.cpu.read_register_pair(Intel8080Register::SP) >> 8) as u8;
        mem[0x10000 + 10] = (self.cpu.read_register_pair(Intel8080Register::SP) & 0xFF) as u8;
        mem[0x10000 + 11] = if self.cpu.read_flag(LR35902Flag::Zero) {
            1
        } else {
            0
        };
        mem[0x10000 + 12] = if self.cpu.read_flag(LR35902Flag::Subtract) {
            1
        } else {
            0
        };
        mem[0x10000 + 13] = if self.cpu.read_flag(LR35902Flag::HalfCarry) {
            1
        } else {
            0
        };
        mem[0x10000 + 14] = if self.cpu.read_flag(LR35902Flag::Carry) {
            1
        } else {
            0
        };
        return super_fast_hash(&mem[..]);
    }

    fn save_state_to_file(&self) -> Result<()> {
        let mut file = File::create("save_state.bin")?;
        self.save_state(&mut file)?;
        Ok(())
    }

    fn load_state_from_file(&mut self) -> Result<()> {
        let mut file = File::open("save_state.bin")?;
        self.load_state(&mut file)?;
        Ok(())
    }

    fn load_state<R: Read>(&mut self, mut input: R) -> Result<()> {
        println!("Loading save state");
        let emulator: Self = bincode::deserialize_from(&mut input)?;
        let old_emulator = std::mem::replace(self, emulator);
        let mut game_pak = old_emulator.game_pak.unwrap();
        game_pak.load_state(&mut input)?;
        self.game_pak = Some(game_pak);
        self.joypad = old_emulator.joypad;
        Ok(())
    }

    fn save_state<W: Write>(&self, mut writer: W) -> Result<()> {
        println!("Saving state");
        bincode::serialize_into(&mut writer, self)?;
        self.game_pak.as_ref().unwrap().save_state(writer)?;
        Ok(())
    }
}

#[test]
fn initial_state_test() {
    let mut e = GameBoyEmulator::new();
    e.plug_in_joy_pad(joypad::PlainJoyPad::new());

    // Lock down the initial state.
    assert_eq!(e.hash(), 1497694477);
}

pub fn run_emulator<R: Renderer>(
    renderer: &mut R,
    game_pak: GamePak,
    save_state: Option<Vec<u8>>,
) -> Result<()> {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);

    if let Some(save_state) = save_state {
        e.load_state(&save_state[..])?;
    }

    e.run(renderer, ControllerJoyPad::new());
    Ok(())
}

pub fn run_in_tandem_with<P: AsRef<Path> + Debug>(
    other_emulator_path: P,
    game_pak: GamePak,
    pc_only: bool,
) -> Result<()> {
    println!("loading {:?}", &other_emulator_path);

    tandem::run(other_emulator_path, game_pak, pc_only)
}

fn run_emulator_until<R: Renderer>(e: &mut GameBoyEmulator, renderer: &mut R, ticks: u64) {
    while e.cpu.elapsed_cycles < ticks {
        if let Some(c) = e.crashed() {
            panic!("Emulator crashed: {}", c);
        }

        e.tick(renderer);
    }
}

fn run_emulator_until_and_take_screenshot<R: Renderer, J: JoyPad + 'static, P: AsRef<Path>>(
    mut e: GameBoyEmulator,
    renderer: &mut R,
    joypad: Option<J>,
    ticks: u64,
    output_path: P,
) {
    if let Some(joypad) = joypad {
        e.plug_in_joy_pad(joypad);
    }
    let ticks = e.cpu.elapsed_cycles + ticks;
    run_emulator_until(&mut e, renderer, ticks);
    renderer.save_buffer(output_path).unwrap();
}

pub fn run_until_and_take_screenshot<R: Renderer, P1: AsRef<Path>, P2: AsRef<Path>>(
    renderer: &mut R,
    game_pak: GamePak,
    ticks: u64,
    replay_path: Option<P1>,
    output_path: P2,
) -> Result<()> {
    let joypad = if let Some(replay_path) = replay_path {
        Some(PlaybackJoyPad::new(game_pak.hash(), replay_path)?)
    } else {
        None
    };

    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);
    run_emulator_until_and_take_screenshot(e, renderer, joypad, ticks, output_path);
    Ok(())
}

pub fn run_and_record_replay<R: Renderer>(
    renderer: &mut R,
    game_pak: GamePak,
    output: &Path,
) -> Result<()> {
    let joypad = RecordingJoyPad::new(game_pak.title(), game_pak.hash(), output)?;
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);
    e.run(renderer, joypad);
    Ok(())
}

pub fn playback_replay<R: Renderer>(
    renderer: &mut R,
    game_pak: GamePak,
    input: &Path,
) -> Result<()> {
    let joypad = PlaybackJoyPad::new(game_pak.hash(), input)?;
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);
    e.run(renderer, joypad);
    Ok(())
}

pub fn print_replay(input: &Path) -> Result<()> {
    joypad::print_replay(input)?;
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests;
