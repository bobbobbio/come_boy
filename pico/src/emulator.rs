// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use crate::joypad::PicoJoyPad;
use come_boy::game_boy_emulator::GamePak;
use come_boy::game_boy_emulator::{GameBoyEmulator, GameBoyOps};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;

const ROM: &[u8] = include_bytes!("../rom.bin");

pub struct Emulator {
    game_boy: GameBoyEmulator,
    ops: GameBoyOps<Graphics, NullSoundStream, PanicStorage>,
}

impl Emulator {
    pub fn new() -> Self {
        let mut ops = GameBoyOps::new(Graphics::new(), NullSoundStream, PanicStorage);
        ops.renderer.blend_copy();

        let game_pak = GamePak::new(ROM, &mut ops.storage, None).unwrap();
        ops.load_game_pak(game_pak);

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
