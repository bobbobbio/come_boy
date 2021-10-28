// Copyright 2017 Remi Bernotavicius

pub use self::game_pak::GamePak;
pub use self::joypad::ControllerJoyPad;
use self::joypad::{JoyPad, KeyEvent};
use self::lcd_controller::{LcdController, OAM_DATA};
use self::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyMemoryMap, GameBoyMemoryMapMut, GameBoyRegister, MemoryAccessor,
    MemoryChunk, MemoryMappedHardware,
};
use self::sound_controller::SoundController;
use crate::io;
use crate::lr35902_emulator::{Intel8080Register, LR35902Emulator, LR35902Flag};
use crate::rendering::{Keycode, NullRenderer, Renderer};
use crate::sound::{NullSoundStream, SoundStream};
use crate::storage::{OpenMode, PanicStorage, PersistentStorage};
use crate::util::{super_fast_hash, Scheduler};
use core::fmt::Debug;
use core::ops::{Range, RangeFrom};
use core::time::Duration;
use core::{fmt, mem};
use enum_iterator::IntoEnumIterator;
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};
use std::time::Instant;

pub use self::debugger::run_debugger;
pub use self::disassembler::disassemble_game_boy_rom;
pub use self::trampolines::*;

mod debugger;
mod disassembler;
mod game_pak;
mod joypad;
mod lcd_controller;
mod memory_controller;
mod sound_controller;
mod tandem;

pub mod trampolines;

/// This is how many ticks of the emulator we do before under-clocking and checking for input
pub const SLEEP_INPUT_TICKS: u64 = 10_000;

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
    start_cycles: u64,
    start_instant: Instant,
    speed: u32,
}

impl Underclocker {
    fn new(now: u64, speed: u32) -> Self {
        Self {
            start_cycles: now,
            start_instant: Instant::now(),
            speed,
        }
    }

    fn underclock(&mut self, now: u64) {
        let elapsed_cycles = now - self.start_cycles;

        let delay = Duration::from_secs(1) / self.speed;
        let expected_elapsed = (elapsed_cycles as u32) * delay;

        if let Some(sleep_time) = expected_elapsed.checked_sub(self.start_instant.elapsed()) {
            std::thread::sleep(sleep_time);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UserControl {
    SaveStateLoaded,
    ScreenClosed,
    SpeedChange,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Replay(joypad::ReplayError),
    Serde(bincode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
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
    fn deliver(
        self,
        emulator: &mut GameBoyEmulator,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        time: u64,
    ) {
        match self {
            Self::DividerTick => emulator.divider_tick(time),
            Self::DriveJoypad => emulator.drive_joypad(ops, time),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct OamDmaTransfer {
    src_current: u16,
    dst_current: u16,
    value: u8,
    now: u64,
}

impl OamDmaTransfer {
    fn new(src: RangeFrom<u16>, now: u64) -> Self {
        Self {
            src_current: src.start,
            dst_current: 0,
            value: 0,
            now,
        }
    }

    fn read(&mut self, memory_map: &GameBoyMemoryMap<'_, impl PersistentStorage>) {
        self.value = memory_map.read_memory(self.src_current);
        self.src_current = self.src_current.wrapping_add(1);
    }

    fn write(&mut self, oam: &mut MemoryChunk) {
        oam.as_mut_slice()[self.dst_current as usize] = self.value;
        self.dst_current += 1;
    }

    fn bytes_to_transfer(&mut self, now: u64) -> u64 {
        let result = now - self.now;
        self.now = now;
        result
    }

    fn is_done(&self) -> bool {
        self.dst_current == (OAM_DATA.end - OAM_DATA.start)
    }
}

const fn default_clock_speed_hz() -> u32 {
    // GameBoy clock speed is about 4.19Mhz
    4_194_304
}

pub struct GameBoyOps<Renderer, SoundStream, Storage: PersistentStorage> {
    pub renderer: Renderer,
    pub sound_stream: SoundStream,
    pub storage: Storage,
    joypad: Option<Box<dyn JoyPad + 'static>>,
    game_pak: Option<GamePak<Storage>>,
    pub clock_speed_hz: u32,
}

pub type NullGameBoyOps = GameBoyOps<NullRenderer, NullSoundStream, PanicStorage>;

impl NullGameBoyOps {
    fn null() -> Self {
        Self::new(NullRenderer, NullSoundStream, PanicStorage)
    }
}

impl<Renderer, SoundStream, Storage: PersistentStorage> GameBoyOps<Renderer, SoundStream, Storage> {
    pub fn new(renderer: Renderer, sound_stream: SoundStream, storage: Storage) -> Self {
        Self {
            renderer,
            sound_stream,
            storage,
            joypad: None,
            game_pak: None,
            clock_speed_hz: default_clock_speed_hz(),
        }
    }

    fn memory_map<'a>(&'a self, bridge: &'a Bridge) -> GameBoyMemoryMap<'a, Storage> {
        GameBoyMemoryMap {
            game_pak: self.game_pak.as_ref(),
            joypad: self.joypad.as_ref().map(|j| &**j as &dyn JoyPad),
            bridge,
        }
    }

    fn memory_map_mut<'a>(
        &'a mut self,
        bridge: &'a mut Bridge,
    ) -> GameBoyMemoryMapMut<'a, Storage> {
        GameBoyMemoryMapMut {
            game_pak: self.game_pak.as_mut(),
            joypad: self.joypad.as_mut().map(|j| &mut **j as &mut dyn JoyPad),
            bridge,
        }
    }

    pub fn plug_in_joy_pad(&mut self, joypad: impl JoyPad + 'static) {
        self.joypad = Some(Box::new(joypad));
    }

    pub fn load_game_pak(&mut self, game_pak: GamePak<Storage>) {
        println!("Loading {:?}", &game_pak);
        self.game_pak = Some(game_pak);
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Bridge {
    sound_controller: SoundController,
    lcd_controller: LcdController,
    high_ram: MemoryChunk,
    internal_ram_a: MemoryChunk,
    internal_ram_b: MemoryChunk,
    registers: GameBoyRegisters,
    timer: GameBoyTimer,
}

impl Bridge {
    fn new() -> Self {
        Self {
            lcd_controller: LcdController::new(),
            sound_controller: Default::default(),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            internal_ram_a: MemoryChunk::from_range(INTERNAL_RAM_A),
            internal_ram_b: MemoryChunk::from_range(INTERNAL_RAM_B),
            registers: Default::default(),
            timer: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameBoyEmulator {
    cpu: LR35902Emulator,
    bridge: Bridge,
    scheduler: Scheduler<GameBoyEmulatorEvent>,
    dma_transfer: Option<OamDmaTransfer>,
    joypad_key_events: Vec<KeyEvent>,
}

impl GameBoyEmulator {
    pub fn new() -> Self {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(),
            bridge: Bridge::new(),

            scheduler: Scheduler::new(),
            dma_transfer: None,
            joypad_key_events: vec![],
        };
        e.set_state_post_bios();
        e.schedule_initial_events();
        e
    }

    fn crashed(&self) -> Option<&String> {
        if self.cpu.crashed() {
            self.cpu.crash_message.as_ref()
        } else {
            None
        }
    }

    fn divider_tick(&mut self, time: u64) {
        self.bridge.registers.divider.increment();
        self.scheduler
            .schedule(time + 256, GameBoyEmulatorEvent::DividerTick);
    }

    fn deliver_events(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        now: u64,
    ) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            event.deliver(self, ops, time);
        }

        self.bridge
            .lcd_controller
            .deliver_events(&mut ops.renderer, now);
        self.bridge.timer.deliver_events(now);
        self.bridge
            .sound_controller
            .deliver_events(now, &mut ops.sound_stream);
    }

    pub fn tick(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        self.cpu
            .load_instruction(&mut ops.memory_map_mut(&mut self.bridge));

        let now = self.cpu.elapsed_cycles;
        self.deliver_events(ops, now);

        self.cpu
            .execute_instruction(&mut ops.memory_map_mut(&mut self.bridge));

        let now = self.cpu.elapsed_cycles;
        self.bridge.timer.tick(now);
        self.bridge.lcd_controller.tick(now);
        self.execute_dma(ops, now);
        self.deliver_events(ops, now);

        self.bridge
            .lcd_controller
            .schedule_interrupts(&mut self.bridge.registers.interrupt_flag);
        self.bridge
            .timer
            .schedule_interrupts(&mut self.bridge.registers.interrupt_flag);

        self.handle_interrupts(ops);
    }

    pub fn read_key_events(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> std::result::Result<(), UserControl> {
        use crate::rendering::Event;

        for event in ops.renderer.poll_events() {
            match event {
                Event::Quit { .. } => {
                    return Err(UserControl::ScreenClosed);
                }
                Event::KeyDown(Keycode::F2) => {
                    if let Err(e) = self.save_state_to_storage(ops) {
                        println!("Failed to create save state {:?}", e);
                    }
                }
                Event::KeyDown(Keycode::F3) => {
                    return Err(UserControl::SaveStateLoaded);
                }
                Event::KeyDown(Keycode::F4) => {
                    if ops.clock_speed_hz == default_clock_speed_hz() {
                        ops.clock_speed_hz = u32::MAX;
                    } else {
                        ops.clock_speed_hz = default_clock_speed_hz();
                    }
                    return Err(UserControl::SpeedChange);
                }
                Event::KeyDown(code) => self.joypad_key_events.push(KeyEvent::Down(code)),
                Event::KeyUp(code) => self.joypad_key_events.push(KeyEvent::Up(code)),
            }
        }
        Ok(())
    }

    fn drive_joypad(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        time: u64,
    ) {
        if let Some(joypad) = &mut ops.joypad {
            joypad.tick(time, mem::take(&mut self.joypad_key_events));
        }

        self.scheduler
            .schedule(time + 456, GameBoyEmulatorEvent::DriveJoypad);
    }

    fn execute_dma(
        &mut self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        now: u64,
    ) {
        if let Some(mut address) = self.bridge.lcd_controller.registers.dma.take_request() {
            // 0xE000 to 0xFFFF is mapped differently for DMA. It ends up just being the internal
            // ram repeated again. To account for this we just adjust the source address.
            if address >= INTERNAL_RAM_B.end {
                address -= 0x2000;
            }
            self.dma_transfer = Some(OamDmaTransfer::new(address.., now));
            self.bridge.lcd_controller.oam_data.borrow();
        }

        if let Some(transfer) = self.dma_transfer.as_mut() {
            for _ in 0..transfer.bytes_to_transfer(now) {
                transfer.read(&ops.memory_map(&self.bridge));
                transfer.write(&mut self.bridge.lcd_controller.oam_data);

                if transfer.is_done() {
                    self.dma_transfer = None;
                    self.bridge.lcd_controller.oam_data.release();
                    break;
                }
            }
        }
    }

    /// If the stack happens to overflow into the IO registers, it can cause weird behavior when
    /// handling interrupts.
    ///
    /// Before handling an interrupt we push the return address to the stack. If the high byte when
    /// writing that address overwrites the IE register (0xFFFF) the interrupt handling will
    /// short-circuit and jump to address 0x0000.
    fn maybe_handle_ie_push_bug(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        flag: InterruptFlag,
    ) -> bool {
        let sp = self.cpu.read_register_pair(Intel8080Register::SP);

        let flag_value = self
            .bridge
            .registers
            .interrupt_enable
            .read_flag(flag.into());

        if sp == 0xFFFE && !flag_value {
            let mut memory_map = ops.memory_map_mut(&mut self.bridge);
            self.cpu.jump(&mut memory_map, 0x0000);
            true
        } else {
            false
        }
    }

    fn deliver_interrupt(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        flag: InterruptFlag,
        address: u16,
    ) {
        let pc = self.cpu.read_program_counter();

        self.cpu
            .push_u16_onto_stack(&mut ops.memory_map_mut(&mut self.bridge), pc);

        if self.maybe_handle_ie_push_bug(ops, flag) {
            return;
        }

        self.cpu
            .jump(&mut ops.memory_map_mut(&mut self.bridge), address);
        self.cpu.push_frame(address);

        self.bridge.registers.interrupt_flag.set_flag(flag, false);
    }

    fn maybe_deliver_interrupt(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        flag: InterruptFlag,
        address: u16,
    ) -> bool {
        let interrupt_flag_value = self.bridge.registers.interrupt_flag.read_flag(flag);
        let interrupt_enable_value = self
            .bridge
            .registers
            .interrupt_enable
            .read_flag(flag.into());

        if interrupt_flag_value && interrupt_enable_value {
            self.cpu.resume();
            if self.cpu.get_interrupts_enabled() {
                self.deliver_interrupt(ops, flag, address);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_interrupts(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        let mut interrupted = false;

        for &(flag, address) in &ALL_INTERRUPTS {
            interrupted |= self.maybe_deliver_interrupt(ops, flag, address);
        }

        if interrupted {
            self.cpu.set_interrupts_enabled(false);
        }
    }

    fn set_state_post_bios(&mut self) {
        self.bridge.sound_controller.set_state_post_bios();
        self.bridge.lcd_controller.set_state_post_bios();

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

        self.bridge.registers.serial_transfer_data.set_value(0x0);
        self.bridge
            .registers
            .serial_transfer_control
            .set_value(0x7e);

        self.bridge.registers.divider.set_state_post_bios();
        self.bridge.timer.set_state_post_bios();

        self.bridge.registers.interrupt_flag.set_value(0xe1);

        /* 10 - 26 Sound Controller */

        /* 40 - 4B LCD Controller */

        let high_ram = include_bytes!("assets/high_ram.bin");
        self.bridge.high_ram.clone_from_slice(&high_ram[..]);

        let internal_ram = include_bytes!("assets/internal_ram.bin");

        let split = (INTERNAL_RAM_A.end - INTERNAL_RAM_A.start) as usize;
        self.bridge
            .internal_ram_a
            .clone_from_slice(&internal_ram[0..split]);
        self.bridge
            .internal_ram_b
            .clone_from_slice(&internal_ram[split..]);

        self.bridge.registers.interrupt_enable.set_value(0x0);
    }

    fn schedule_initial_events(&mut self) {
        let now = self.cpu.elapsed_cycles;
        self.scheduler
            .schedule(now + 52, GameBoyEmulatorEvent::DividerTick);
        self.scheduler
            .schedule(now + 456, GameBoyEmulatorEvent::DriveJoypad);

        self.bridge.lcd_controller.schedule_initial_events(now);
        self.bridge.timer.schedule_initial_events(now);
        self.bridge.sound_controller.schedule_initial_events(now);
    }

    pub fn elapsed_cycles(&self) -> u64 {
        self.cpu.elapsed_cycles
    }

    fn run_inner(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> std::result::Result<(), UserControl> {
        let mut underclocker = Underclocker::new(self.cpu.elapsed_cycles, ops.clock_speed_hz);
        let mut sometimes = ModuloCounter::new(SLEEP_INPUT_TICKS);

        while self.crashed().is_none() {
            self.tick(ops);

            // We can't do this every tick because it is too slow. So instead so only every so
            // often.
            if sometimes.incr() {
                underclocker.underclock(self.elapsed_cycles());
                self.read_key_events(ops)?;
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

    fn run(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        loop {
            let res = self.run_inner(ops);
            match res {
                Err(UserControl::SaveStateLoaded) => {
                    if let Err(e) = self.load_state_from_storage(ops) {
                        println!("Failed to load state {:?}", e);
                    }
                }
                Err(UserControl::SpeedChange) => {}
                _ => break,
            }
        }
    }

    fn write_memory(
        &self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        w: &mut dyn io::Write,
    ) -> Result<()> {
        let memory_map = ops.memory_map(&self.bridge);
        let mut mem = [0u8; 0x10000];
        for i in 0..0x10000 {
            mem[i] = memory_map.read_memory(i as u16);
        }

        w.write(&mem)?;
        Ok(())
    }

    fn hash(
        &self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> u32 {
        let memory_map = ops.memory_map(&self.bridge);
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

    fn save_state_to_storage(
        &self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> Result<()> {
        let mut stream = ops.storage.open(OpenMode::Write, "save_state.bin")?;
        self.save_state(ops.game_pak.as_ref(), &mut stream)?;
        Ok(())
    }

    fn load_state_from_storage(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> Result<()> {
        let mut stream = ops.storage.open(OpenMode::Read, "save_state.bin")?;
        self.load_state(ops.game_pak.as_mut(), &mut stream)?;
        Ok(())
    }

    fn load_state<R: io::Read>(
        &mut self,
        game_pak: Option<&mut GamePak<impl PersistentStorage>>,
        mut input: R,
    ) -> Result<()> {
        println!("Loading save state");

        *self = bincode::deserialize_from(&mut input)?;

        if let Some(game_pak) = game_pak {
            game_pak.load_state(&mut input)?;
        }

        Ok(())
    }

    fn save_state<W: io::Write>(
        &self,
        game_pak: Option<&GamePak<impl PersistentStorage>>,
        mut writer: W,
    ) -> Result<()> {
        println!("Saving state");

        bincode::serialize_into(&mut writer, self)?;

        if let Some(game_pak) = game_pak {
            game_pak.save_state(writer)?;
        }

        Ok(())
    }
}

#[test]
fn initial_state_test() {
    let mut ops = GameBoyOps::null();
    let e = GameBoyEmulator::new();
    ops.plug_in_joy_pad(joypad::PlainJoyPad::new());

    // Lock down the initial state.
    assert_eq!(e.hash(&ops), 1497694477);
}

#[cfg(test)]
mod tests;
