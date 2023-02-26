// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use crate::joypad::PicoJoyPad;
use alloc::boxed::Box;
use come_boy::game_boy_emulator::{GameBoyEmulator, GameBoyOps, GamePak};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;
use core::cell::UnsafeCell;

const ROM: &[u8] = include_bytes!("../rom.bin");

struct Emulator {
    game_boy: GameBoyEmulator,
    ops: GameBoyOps<Graphics, NullSoundStream, PanicStorage>,
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
        }
    }

    fn tick(&mut self) {
        self.game_boy.tick(&mut self.ops);
    }
}

struct UnsafeEmulator(UnsafeCell<*mut Emulator>);

unsafe impl Sync for UnsafeEmulator {}

impl UnsafeEmulator {
    const fn new() -> Self {
        Self(UnsafeCell::new(core::ptr::null_mut()))
    }
}

static EMULATOR: UnsafeEmulator = UnsafeEmulator::new();

pub unsafe fn init_from_core0() {
    *unsafe { &mut *EMULATOR.0.get() } = Box::into_raw(Box::new(Emulator::new()));
}

pub fn run_from_core1() -> ! {
    let emulator = unsafe { &mut **EMULATOR.0.get() };
    loop {
        emulator.tick();
    }
}
