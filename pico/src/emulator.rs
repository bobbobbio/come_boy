// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use crate::joypad::PicoJoyPad;
use crate::time::Instant;
use alloc::format;
use come_boy::game_boy_emulator::{perf::PerfStats, GameBoyEmulator, GameBoyOps, GamePak};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;

const ROM: &[u8] = include_bytes!("../rom.bin");

pub struct Emulator {
    game_boy: GameBoyEmulator,
    ops: GameBoyOps<Graphics, NullSoundStream, PanicStorage>,
    perf_stats: PerfStats<crate::time::Instant>,
    ticks: u64,
    frames: u64,
    update_total: core::time::Duration,
    between_update_total: core::time::Duration,
    last_update_ended: Option<Instant>,
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
            perf_stats: Default::default(),
            ticks: 0,
            frames: 0,
            update_total: core::time::Duration::ZERO,
            between_update_total: core::time::Duration::ZERO,
            last_update_ended: None,
        }
    }

    pub fn sample(&self) -> bool {
        self.ticks % 1_000 == 0
    }

    pub fn update(&mut self) {
        if let Some(u) = self.last_update_ended.take() {
            self.between_update_total += u.elapsed();
        }

        self.ops.renderer.dirty = false;

        let frame = Instant::now();
        while !self.ops.renderer.dirty {
            if self.sample() {
                self.game_boy
                    .tick_with_observer(&mut self.ops, &mut self.perf_stats);
            } else {
                self.game_boy.tick(&mut self.ops);
            }
            self.ticks += 1;
        }
        self.frames += 1;
        self.update_total += frame.elapsed();

        self.last_update_ended = Some(Instant::now());
    }

    pub fn draw(&mut self) {
        // clear stats
        self.ops
            .renderer
            .set_pen(crate::graphics::Color::rgb(0, 0, 0));
        self.ops.renderer.filled_rect(3, 160, 240, 100);

        // draw stats
        self.ops
            .renderer
            .set_pen(crate::graphics::Color::rgb(255, 0, 0));

        let update = (self.update_total / self.frames as u32).as_millis();

        let update_betweens = self.frames.saturating_sub(1) as u32;
        let between_update = (update_betweens > 0)
            .then(|| self.between_update_total / update_betweens)
            .unwrap_or_default()
            .as_millis();

        let ticks_per_frame = self.ticks / self.frames;

        self.ops.renderer.text(
            &format!(
                "update: {update}ms\n\
                 between update: {between_update}ms\n\
                 ticks per frame: {ticks_per_frame}\n\
                 {}",
                &self.perf_stats
            ),
            3,
            160,
        );
    }
}
