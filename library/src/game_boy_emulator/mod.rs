// Copyright 2017 Remi Bernotavicius

pub use self::game_pak::{rom_hash, GamePak};
pub use self::joypad::ControllerJoyPad;
use self::joypad::{JoyPad, KeyEvent};
use self::lcd_controller::{LcdController, OAM_DATA};
pub use self::memory_controller::MemoryMappedHardware;
use self::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyMemoryMap, GameBoyMemoryMapMut, GameBoyRegister, MemoryAccessor,
    MemoryChunk,
};
use self::sound_controller::SoundController;
use crate::io;
use crate::lr35902_emulator::{Intel8080Register, LR35902Emulator, LR35902Flag};
use crate::rendering::{Keycode, NullRenderer, Renderer};
use crate::sound::{NullSoundStream, SoundStream};
use crate::storage::{OpenMode, PanicStorage, PersistentStorage};
use crate::util::super_fast_hash;
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::fmt::Debug;
use core::ops::{Range, RangeFrom};
use core::{fmt, mem};
use enum_iterator::IntoEnumIterator;
pub use lcd_controller::Palette;
use num_enum::IntoPrimitive;
use perf::PerfObserver;
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "std")]
pub use self::assembler::assemble;
pub use self::disassembler::disassemble_game_boy_rom;
pub use self::trampolines::*;
pub use self::underclocker::*;
pub use perf::NullPerfObserver;

#[cfg(feature = "std")]
mod assembler;
mod coverage;
mod debugger;
mod disassembler;
mod game_pak;
pub mod joypad;
mod lcd_controller;
mod memory_controller;
#[macro_use]
pub mod perf;
mod runner;
mod sound_controller;
mod tandem;
mod underclocker;

pub mod trampolines;

#[derive(Debug, Copy, Clone)]
pub enum UserControl {
    SaveStateLoaded,
    ScreenClosed,
    SpeedChange,
}

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

#[derive(Debug)]
pub enum Error {
    Coverage(coverage::Error),
    Io(io::Error),
    Replay(joypad::replay::Error),
    Serde(crate::codec::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<joypad::replay::Error> for Error {
    fn from(e: joypad::replay::Error) -> Self {
        Self::Replay(e)
    }
}

impl From<coverage::Error> for Error {
    fn from(e: coverage::Error) -> Self {
        Self::Coverage(e)
    }
}

impl From<crate::codec::Error> for Error {
    fn from(e: crate::codec::Error) -> Self {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, IntoEnumIterator)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, IntoEnumIterator)]
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn increment(&mut self) {
        self.0.add(1)
    }
}

impl MemoryMappedHardware for Divider {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.0.read_value()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_value(&mut self, address: u16, _value: u8) {
        assert_eq!(address, 0);
        self.0.set_value(0);
    }
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
struct InterruptsEnabled(bool);

impl InterruptsEnabled {
    fn get(&self) -> bool {
        self.0
    }

    fn set(&mut self, value: bool) {
        self.0 = value
    }
}

impl<'a> MemoryMappedHardware for (&'a InterruptsEnabled, &'a GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, _address: u16) -> u8 {
        unreachable!()
    }

    fn set_value(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }

    fn set_interrupts_enabled(&mut self, _enabled: bool) {
        unreachable!()
    }
}

impl<'a> MemoryMappedHardware for (&'a mut InterruptsEnabled, &'a mut GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, _address: u16) -> u8 {
        unreachable!()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_value(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_interrupts_enabled(&mut self, enabled: bool) {
        let (flag, scheduler) = self;
        flag.set(enabled);

        if enabled {
            scheduler.schedule(scheduler.now(), GameBoyEmulatorEvent::HandleInterrupts);
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct GameBoyRegisters {
    interrupt_flag: GameBoyFlags<InterruptFlag>,
    interrupt_enable_mask: GameBoyFlags<InterruptEnableFlag>,
    interrupts_enabled: InterruptsEnabled,

    serial_transfer_data: GameBoyRegister,
    serial_transfer_control: GameBoyRegister,
    divider: Divider,
}

/// This implementation is where reads for interrupt_flag go
impl<'a> MemoryMappedHardware for (&'a GameBoyFlags<InterruptFlag>, &'a GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (flags, _scheduler) = self;
        flags.read_value()
    }

    fn set_value(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }
}

/// This implementation is where the writes for interrupt_flag go
impl<'a> MemoryMappedHardware
    for (
        &'a mut GameBoyFlags<InterruptFlag>,
        &'a mut GameBoyScheduler,
    )
{
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (flags, _scheduler) = self;
        flags.read_value()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        let (flags, scheduler) = self;

        flags.set_value(value);

        if value != 0 {
            let now = scheduler.now();
            scheduler.schedule(now, GameBoyEmulatorEvent::HandleInterrupts);
        }
    }
}

/// This implementation is where reads for interrupt_enable_mask go
impl<'a> MemoryMappedHardware for (&'a GameBoyFlags<InterruptEnableFlag>, &'a GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (flags, _scheduler) = self;
        flags.read_value()
    }

    fn set_value(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }
}

/// This implementation is where the writes for interrupt_enable_mask go
impl<'a> MemoryMappedHardware
    for (
        &'a mut GameBoyFlags<InterruptEnableFlag>,
        &'a mut GameBoyScheduler,
    )
{
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (flags, _scheduler) = self;
        flags.read_value()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        let (flags, scheduler) = self;

        flags.set_value(value);

        if value != 0 {
            let now = scheduler.now();
            scheduler.schedule(now, GameBoyEmulatorEvent::HandleInterrupts);
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct GameBoyTimer {
    counter: GameBoyRegister,
    modulo: GameBoyRegister,
    control: GameBoyFlags<TimerFlags>,
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

#[derive(Debug, Clone, Copy, IntoPrimitive, IntoEnumIterator)]
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

    fn schedule_initial_events(&mut self, scheduler: &mut GameBoyScheduler, now: u64) {
        let speed = self.timer_speed();
        scheduler.schedule(now + speed, GameBoyEmulatorEvent::TimerTick);
    }

    fn fire(
        &mut self,
        interrupt_flag: &mut GameBoyFlags<InterruptFlag>,
        scheduler: &mut GameBoyScheduler,
        now: u64,
    ) {
        let counter = self.counter.read_value().wrapping_add(1);
        if counter == 0 {
            interrupt_flag.set_flag(InterruptFlag::Timer, true);
            scheduler.schedule(now, GameBoyEmulatorEvent::HandleInterrupts);
            let modulo_value = self.modulo.read_value();
            self.counter.set_value(modulo_value);
        } else {
            self.counter.set_value(counter);
        }
        let speed = self.timer_speed();
        scheduler.schedule(now + speed, GameBoyEmulatorEvent::TimerTick);
    }
}

/// This implementation is where reads for timer control go
impl<'a> MemoryMappedHardware for (&'a GameBoyTimer, &'a GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (timer, _scheduler) = self;
        timer.control.read_value()
    }

    fn set_value(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }
}

/// This implementation is where the writes for timer control go
impl<'a> MemoryMappedHardware for (&'a mut GameBoyTimer, &'a mut GameBoyScheduler) {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        let (timer, _scheduler) = self;
        timer.control.read_value()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        let (timer, scheduler) = self;

        timer.control.set_value(value);

        scheduler.drop_events(|e| matches!(e, GameBoyEmulatorEvent::TimerTick));
        if timer.enabled() {
            let speed = timer.timer_speed();
            let now = scheduler.now();
            scheduler.schedule(now + speed, GameBoyEmulatorEvent::TimerTick);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GameBoyEmulatorEvent {
    DividerTick,
    DriveJoypad,
    TimerTick,
    StartDmaTransfer { address: u16 },
    DriveDmaTransfer,
    HandleInterrupts,
    Lcd(lcd_controller::LcdControllerEvent),
    Sound(sound_controller::SoundControllerEvent),
}

impl<'a> From<&'a GameBoyEmulatorEvent> for &'static str {
    fn from(e: &'a GameBoyEmulatorEvent) -> Self {
        match e {
            GameBoyEmulatorEvent::DividerTick => "DividerTick",
            GameBoyEmulatorEvent::DriveJoypad => "DriveJoypad",
            GameBoyEmulatorEvent::TimerTick => "TimerTick",
            GameBoyEmulatorEvent::StartDmaTransfer { .. } => "StartDmaTransfer",
            GameBoyEmulatorEvent::DriveDmaTransfer => "DriveDmaTransfer",
            GameBoyEmulatorEvent::HandleInterrupts => "HandleInterrupts",
            GameBoyEmulatorEvent::Lcd(e) => e.into(),
            GameBoyEmulatorEvent::Sound(e) => e.into(),
        }
    }
}

impl From<lcd_controller::LcdControllerEvent> for GameBoyEmulatorEvent {
    fn from(e: lcd_controller::LcdControllerEvent) -> Self {
        Self::Lcd(e)
    }
}

impl From<sound_controller::SoundControllerEvent> for GameBoyEmulatorEvent {
    fn from(e: sound_controller::SoundControllerEvent) -> Self {
        Self::Sound(e)
    }
}

type GameBoyScheduler = crate::util::Scheduler<GameBoyEmulatorEvent>;

impl GameBoyEmulatorEvent {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn deliver(
        self,
        emulator: &mut GameBoyEmulator,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        time: u64,
    ) {
        let scheduler = &mut emulator.bridge.scheduler;
        let interrupt_flag = &mut emulator.bridge.registers.interrupt_flag;
        match self {
            Self::DividerTick => emulator.divider_tick(time),
            Self::DriveJoypad => emulator.drive_joypad(ops, time),
            Self::TimerTick => emulator.bridge.timer.fire(interrupt_flag, scheduler, time),
            Self::StartDmaTransfer { address } => emulator.start_dma_transfer(ops, address, time),
            Self::DriveDmaTransfer => emulator.drive_dma_transfer(ops, time),
            Self::HandleInterrupts => emulator.handle_interrupts(ops),
            Self::Lcd(e) => e.deliver(
                &mut emulator.bridge.lcd_controller,
                &mut ops.renderer,
                interrupt_flag,
                scheduler,
                time,
            ),
            Self::Sound(e) => e.deliver(
                &mut emulator.bridge.sound_controller,
                &mut ops.sound_stream,
                scheduler,
                time,
            ),
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
    pub fn null() -> Self {
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn memory_map<'a>(&'a self, bridge: &'a Bridge) -> GameBoyMemoryMap<'a, Storage> {
        GameBoyMemoryMap {
            game_pak: self.game_pak.as_ref(),
            joypad: self.joypad.as_ref().map(|j| &**j as &dyn JoyPad),
            bridge,
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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
        log::info!("Loading {:?}", &game_pak);
        self.game_pak = Some(game_pak);
    }

    pub fn loaded_game_pak(&self) -> Option<&GamePak<Storage>> {
        self.game_pak.as_ref()
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
    scheduler: GameBoyScheduler,
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
            scheduler: GameBoyScheduler::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameBoyEmulator {
    pub cpu: LR35902Emulator,
    bridge: Bridge,
    dma_transfer: Option<OamDmaTransfer>,
    joypad_key_events: Vec<KeyEvent>,
}

impl Default for GameBoyEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoyEmulator {
    pub fn new() -> Self {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(),
            bridge: Bridge::new(),

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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn divider_tick(&mut self, time: u64) {
        self.bridge.registers.divider.increment();
        self.bridge
            .scheduler
            .schedule(time + 256, GameBoyEmulatorEvent::DividerTick);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn deliver_events(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        observer: &mut impl PerfObserver,
        interrupts_ok: bool,
        now: u64,
    ) {
        let m = |e: &_| interrupts_ok || !matches!(e, GameBoyEmulatorEvent::HandleInterrupts);
        while let Some((time, event)) = self.bridge.scheduler.poll_match(now, m) {
            observe!(observer, (&event).into(), event.deliver(self, ops, time));
        }
    }

    pub fn tick(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        self.tick_with_observer(ops, &mut NullPerfObserver)
    }

    #[cold]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn halted_cpu_tick(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        observer: &mut impl PerfObserver,
    ) {
        self.cpu.add_cycles(4);

        let interrupts_ok = true;
        self.deliver_events(ops, observer, interrupts_ok, self.cpu.elapsed_cycles);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn cpu_tick(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        observer: &mut impl PerfObserver,
    ) {
        let instr = observe!(observer, "load_instruction", {
            self.cpu.load_instruction(&ops.memory_map(&self.bridge))
        });

        if let Some(instr) = instr {
            let interrupts_ok = false;
            self.deliver_events(ops, observer, interrupts_ok, self.cpu.elapsed_cycles);

            observe!(observer, "execute_instruction", {
                self.cpu
                    .execute_instruction(&mut ops.memory_map_mut(&mut self.bridge), instr)
            });
        } else {
            self.cpu.crash_from_unkown_opcode();
        }
    }

    pub fn tick_with_observer(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        observer: &mut impl PerfObserver,
    ) {
        if self.cpu.is_halted() {
            self.halted_cpu_tick(ops, observer);
        } else {
            self.cpu_tick(ops, observer);
        }

        let interrupts_ok = true;
        self.deliver_events(ops, observer, interrupts_ok, self.cpu.elapsed_cycles);

        observe!(observer, "nothing", ());

        observer.tick_observed();
    }

    pub fn read_key_events(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> core::result::Result<(), UserControl> {
        use crate::rendering::Event;

        for event in ops.renderer.poll_events() {
            match event {
                Event::Quit { .. } => {
                    return Err(UserControl::ScreenClosed);
                }
                Event::KeyDown(Keycode::F2) => {
                    if let Err(e) = self.save_state_to_storage(ops) {
                        log::info!("Failed to create save state {:?}", e);
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn drive_joypad(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        time: u64,
    ) {
        if let Some(joypad) = &mut ops.joypad {
            joypad.tick(time, mem::take(&mut self.joypad_key_events));
        }

        self.bridge
            .scheduler
            .schedule(time + 456, GameBoyEmulatorEvent::DriveJoypad);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn start_dma_transfer(
        &mut self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        mut address: u16,
        now: u64,
    ) {
        if self.dma_transfer.is_some() {
            // XXX: I'm not sure what is suppose to happen in this case.
            return;
        }
        // 0xE000 to 0xFFFF is mapped differently for DMA. It ends up just being the internal
        // ram repeated again. To account for this we just adjust the source address.
        if address >= INTERNAL_RAM_B.end {
            address -= 0x2000;
        }
        self.dma_transfer = Some(OamDmaTransfer::new(address.., now));
        self.bridge.lcd_controller.oam_data.borrow();

        self.drive_dma_transfer(ops, now);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn drive_dma_transfer(
        &mut self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        now: u64,
    ) {
        let transfer = self.dma_transfer.as_mut().unwrap();

        for _ in 0..transfer.bytes_to_transfer(now) {
            transfer.read(&ops.memory_map(&self.bridge));
            transfer.write(&mut self.bridge.lcd_controller.oam_data);

            if transfer.is_done() {
                self.dma_transfer = None;
                self.bridge.lcd_controller.oam_data.release();
                return;
            }
        }
        self.bridge
            .scheduler
            .schedule(now + 4, GameBoyEmulatorEvent::DriveDmaTransfer);
    }

    /// If the stack happens to overflow into the IO registers, it can cause weird behavior when
    /// handling interrupts.
    ///
    /// Before handling an interrupt we push the return address to the stack. If the high byte when
    /// writing that address overwrites the IE register (0xFFFF) the interrupt handling will
    /// short-circuit and jump to address 0x0000.
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn maybe_handle_ie_push_bug(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        flag: InterruptFlag,
    ) -> bool {
        let sp = self.cpu.read_register_pair(Intel8080Register::SP);

        let flag_value = self
            .bridge
            .registers
            .interrupt_enable_mask
            .read_flag(flag.into());

        if sp == 0xFFFE && !flag_value {
            let mut memory_map = ops.memory_map_mut(&mut self.bridge);
            self.cpu.jump(&mut memory_map, 0x0000);
            true
        } else {
            false
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn maybe_deliver_interrupt(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        flag: InterruptFlag,
        address: u16,
    ) -> bool {
        let interrupt_flag_value = self.bridge.registers.interrupt_flag.read_flag(flag);
        let interrupts_enable_mask_value = self
            .bridge
            .registers
            .interrupt_enable_mask
            .read_flag(flag.into());

        if interrupt_flag_value && interrupts_enable_mask_value {
            self.cpu.resume();
            if self.bridge.registers.interrupts_enabled.get() {
                self.deliver_interrupt(ops, flag, address);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn handle_interrupts(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        let interrupts = self.bridge.registers.interrupt_flag.read_value();
        let interrupts_mask = self.bridge.registers.interrupt_enable_mask.read_value();

        // fast-path, no interrupts to process
        if interrupts & interrupts_mask == 0 {
            return;
        }

        let mut interrupted = false;

        for &(flag, address) in &ALL_INTERRUPTS {
            interrupted |= self.maybe_deliver_interrupt(ops, flag, address);
        }

        if interrupted {
            self.bridge.registers.interrupts_enabled.set(false);
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

        self.bridge.registers.interrupt_enable_mask.set_value(0x0);
    }

    fn schedule_initial_events(&mut self) {
        let now = self.cpu.elapsed_cycles;
        self.bridge
            .scheduler
            .schedule(now + 52, GameBoyEmulatorEvent::DividerTick);
        self.bridge
            .scheduler
            .schedule(now + 456, GameBoyEmulatorEvent::DriveJoypad);

        self.bridge
            .lcd_controller
            .schedule_initial_events(&mut self.bridge.scheduler, now);
        self.bridge
            .timer
            .schedule_initial_events(&mut self.bridge.scheduler, now);
        self.bridge
            .sound_controller
            .schedule_initial_events(&mut self.bridge.scheduler, now);
    }

    pub fn elapsed_cycles(&self) -> u64 {
        self.cpu.elapsed_cycles
    }

    fn write_memory(
        &self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        w: &mut dyn io::Write,
    ) -> Result<()> {
        let memory_map = ops.memory_map(&self.bridge);
        let mut mem = [0u8; 0x10000];
        for (i, item) in mem.iter_mut().enumerate() {
            *item = memory_map.read_memory(i as u16);
        }

        w.write_all(&mem)?;
        Ok(())
    }

    fn hash(
        &self,
        ops: &GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) -> u32 {
        let memory_map = ops.memory_map(&self.bridge);
        let mut mem = [0u8; 0x10000 + 15];
        for (i, item) in mem.iter_mut().enumerate().take(0x10000) {
            *item = memory_map.read_memory(i as u16);
        }

        mem[0x10000] = self.cpu.read_register(Intel8080Register::A);
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
        mem[0x10000 + 11] = u8::from(self.cpu.read_flag(LR35902Flag::Zero));
        mem[0x10000 + 12] = u8::from(self.cpu.read_flag(LR35902Flag::Subtract));
        mem[0x10000 + 13] = u8::from(self.cpu.read_flag(LR35902Flag::HalfCarry));
        mem[0x10000 + 14] = u8::from(self.cpu.read_flag(LR35902Flag::Carry));
        super_fast_hash(&mem[..])
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
        log::info!("Loading save state");

        *self = crate::codec::deserialize_from(&mut input)?;

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
        log::info!("Saving state");

        crate::codec::serialize_into(&mut writer, self)?;

        if let Some(game_pak) = game_pak {
            game_pak.save_state(writer)?;
        }

        Ok(())
    }

    pub fn palette_mut(&mut self) -> &mut Palette {
        self.bridge.lcd_controller.palette_mut()
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
