use super::renderer::CanvasRenderer;
use super::window;
use come_boy::game_boy_emulator::{
    ControllerJoyPad, GameBoyEmulator, GameBoyOps, GamePak, UserControl, SLEEP_INPUT_TICKS,
};
use come_boy::sound::NullSoundStream;
use come_boy::storage::PanicStorage;

fn performance() -> web_sys::Performance {
    window().performance().expect("performance to be available")
}

fn f64_saturating_sub(a: f64, b: f64) -> f64 {
    let delta = a - b;
    if delta > 0.0 {
        delta
    } else {
        0.0
    }
}

struct Underclocker {
    start_cycles: u64,
    start_time: f64,
    speed: u32,
    perf: web_sys::Performance,
}

impl Underclocker {
    fn new(now: u64, speed: u32) -> Self {
        let perf = performance();
        Self {
            start_cycles: now,
            start_time: perf.now(),
            speed,
            perf,
        }
    }

    fn get_delay(&mut self, now: u64) -> i32 {
        let elapsed_cycles = now - self.start_cycles;

        let delay = 1000.0 / self.speed as f64;
        let expected_time = elapsed_cycles as f64 * delay;

        let elapsed_time = self.perf.now() - self.start_time;
        f64_saturating_sub(expected_time, elapsed_time) as i32
    }
}

pub struct Emulator {
    emulator: GameBoyEmulator,
    ops: GameBoyOps<CanvasRenderer, NullSoundStream, PanicStorage>,
    underclocker: Underclocker,
}

impl Emulator {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let emulator = GameBoyEmulator::new();
        let ops = GameBoyOps::new(CanvasRenderer::new(canvas), NullSoundStream, PanicStorage);
        let underclocker = Underclocker::new(emulator.elapsed_cycles(), ops.clock_speed_hz);
        Self {
            emulator,
            ops,
            underclocker,
        }
    }

    pub fn on_key_down(&mut self, code: &str) {
        self.ops.renderer.on_key_down(code);
    }

    pub fn on_key_up(&mut self, code: &str) {
        self.ops.renderer.on_key_up(code);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.emulator = GameBoyEmulator::new();
        let game_pak = GamePak::new(rom, &mut self.ops.storage, None).unwrap();
        self.ops.load_game_pak(game_pak);
        self.ops.plug_in_joy_pad(ControllerJoyPad::new());
    }

    pub fn render(&self) {
        self.ops.renderer.render();
    }

    fn read_key_events(&mut self) {
        let res = self.emulator.read_key_events(&mut self.ops);
        match res {
            Err(UserControl::SpeedChange) => {
                self.underclocker =
                    Underclocker::new(self.emulator.elapsed_cycles(), self.ops.clock_speed_hz);
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) -> i32 {
        for _ in 0..SLEEP_INPUT_TICKS {
            self.emulator.tick(&mut self.ops);
        }
        self.read_key_events();

        self.underclocker.get_delay(self.emulator.elapsed_cycles())
    }
}