use super::renderer::CanvasRenderer;
use super::window;
use come_boy::game_boy_emulator::{ControllerJoyPad, GameBoyEmulator, GamePak, SLEEP_INPUT_TICKS};

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

pub struct Emulator {
    renderer: CanvasRenderer,
    emulator: GameBoyEmulator,
    last_tick: f64,
}

impl Emulator {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        Self {
            renderer: CanvasRenderer::new(canvas),
            emulator: GameBoyEmulator::new(),
            last_tick: performance().now(),
        }
    }

    pub fn on_key_down(&mut self, code: &str) {
        self.renderer.on_key_down(code);
    }

    pub fn on_key_up(&mut self, code: &str) {
        self.renderer.on_key_up(code);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.emulator = GameBoyEmulator::new();
        let game_pak = GamePak::new(rom, None);
        self.emulator.load_game_pak(game_pak);
        self.emulator.plug_in_joy_pad(ControllerJoyPad::new());
    }

    pub fn render(&self) {
        self.renderer.render();
    }

    pub fn tick(&mut self) -> i32 {
        let perf = performance();

        let start_cycles = self.emulator.elapsed_cycles();

        for _ in 0..SLEEP_INPUT_TICKS {
            self.emulator.tick(&mut self.renderer);
        }
        self.emulator.read_key_events(&mut self.renderer).unwrap();

        let elapsed_cycles = self.emulator.elapsed_cycles() - start_cycles;
        let delay = 1000.0 / self.emulator.clock_speed_hz() as f64;
        let expected_time = elapsed_cycles as f64 * delay;

        let now = perf.now();
        let elapsed_time = now - self.last_tick;
        self.last_tick = now;

        f64_saturating_sub(expected_time, elapsed_time) as i32
    }
}
