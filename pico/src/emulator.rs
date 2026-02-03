// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use crate::joypad::PicoJoyPad;
use crate::mutex::Mutex;
use alloc::boxed::Box;
use come_boy::game_boy_emulator::{perf::PerfStats, GameBoyEmulator, GameBoyOps, GamePak};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;
use core::cell::UnsafeCell;

const ROM: &[u8] = include_bytes!("../rom.bin");

pub struct SharedPerfStats(UnsafeCell<*mut Mutex<PerfStats<crate::time::Instant>>>);

unsafe impl Sync for SharedPerfStats {}

impl SharedPerfStats {
    const fn new() -> Self {
        Self(UnsafeCell::new(core::ptr::null_mut()))
    }

    // safety: must be called exactly once
    unsafe fn init(&self) {
        *unsafe { &mut *self.0.get() } = Box::into_raw(Box::new(Mutex::new(PerfStats::default())));
    }

    pub unsafe fn get(&self) -> &Mutex<PerfStats<crate::time::Instant>> {
        unsafe { &**self.0.get() }
    }
}

pub static PERF_STATS: SharedPerfStats = SharedPerfStats::new();

struct Emulator {
    game_boy: GameBoyEmulator,
    ops: GameBoyOps<Graphics, NullSoundStream, PanicStorage>,
    ticks: u64,
}

impl Emulator {
    fn new() -> Self {
        let mut ops = GameBoyOps::new(Graphics::new(), NullSoundStream, PanicStorage);

        let game_pak = GamePak::new_static(ROM, &mut ops.storage, None).unwrap();
        ops.load_game_pak(game_pak);

        ops.plug_in_joy_pad(PicoJoyPad::new());

        Self {
            game_boy: GameBoyEmulator::new(),
            ops,
            ticks: 0,
        }
    }

    fn sample(&self) -> bool {
        self.ticks.is_multiple_of(10_000)
    }

    fn tick(&mut self) {
        if self.sample() {
            let mut perf_stats = unsafe { PERF_STATS.get() }.lock().unwrap();
            self.game_boy
                .tick_with_observer(&mut self.ops, &mut *perf_stats);
        } else {
            self.game_boy.tick(&mut self.ops);
        }
        self.ticks += 1;
    }
}

struct UnsafeEmulator(UnsafeCell<*mut Emulator>);

unsafe impl Sync for UnsafeEmulator {}

impl UnsafeEmulator {
    const fn new() -> Self {
        Self(UnsafeCell::new(core::ptr::null_mut()))
    }

    // safety: must be called exactly once
    unsafe fn init(&self) {
        *unsafe { &mut *self.0.get() } = Box::into_raw(Box::new(Emulator::new()));
    }
}

static EMULATOR: UnsafeEmulator = UnsafeEmulator::new();

pub unsafe fn init_from_core0() {
    unsafe { PERF_STATS.init() };
    unsafe { EMULATOR.init() };
}

pub fn run_from_core1() -> ! {
    let emulator = unsafe { &mut **EMULATOR.0.get() };
    loop {
        emulator.tick();
    }
}
