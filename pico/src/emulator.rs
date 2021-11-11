// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use crate::joypad::PicoJoyPad;
use come_boy::game_boy_emulator::{GameBoyEmulator, GameBoyOps};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;

pub struct Emulator {
    game_boy: GameBoyEmulator,
    ops: GameBoyOps<Graphics, NullSoundStream, PanicStorage>,
}

impl Emulator {
    pub fn new() -> Self {
        let mut ops = GameBoyOps::new(Graphics::new(), NullSoundStream, PanicStorage);
        ops.renderer.blend_copy();

        /*
         * uncomment to load ROM
        use come_boy::game_boy_emulator::GamePak;
        const ROM: &'static [u8] = include_bytes!("../../library/test/roms/tetris.gb");
        let game_pak = GamePak::new(ROM, &mut ops.storage, None).unwrap();
        ops.load_game_pak(game_pak);
        */

        ops.plug_in_joy_pad(PicoJoyPad::new());

        Self {
            game_boy: GameBoyEmulator::new(),
            ops,
        }
    }

    pub fn update(&mut self) {
        self.ops.renderer.dirty = false;

        while !self.ops.renderer.dirty {
            self.game_boy.tick(&mut self.ops);
        }
    }

    pub fn draw(&mut self) {}
}
