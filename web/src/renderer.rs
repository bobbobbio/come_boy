// copyright 2021 Remi Bernotavicius

pub use come_boy::rendering::glow::{HEIGHT, PIXEL_SIZE, WIDTH};
use come_boy::rendering::{
    self,
    glow::{GlowBackRenderer, GlowFrontRenderer},
    Event, Keycode, Renderer,
};
use std::{io, mem};

pub struct CanvasFrontRenderer {
    inner: GlowFrontRenderer,
}

pub struct CanvasBackRenderer {
    inner: GlowBackRenderer,
    keyboard_events: Vec<Event>,
    pub dirty: bool,
}

fn keycode_from_native_code(code: &str) -> Keycode {
    match code {
        "ArrowDown" => Keycode::Down,
        "ArrowLeft" => Keycode::Left,
        "ArrowRight" => Keycode::Right,
        "ArrowUp" => Keycode::Up,
        "Tab" => Keycode::Tab,
        "KeyX" => Keycode::X,
        "KeyZ" => Keycode::Z,
        "Enter" => Keycode::Return,
        "F2" => Keycode::F2,
        "F3" => Keycode::F3,
        "F4" => Keycode::F4,
        _ => Keycode::Unknown,
    }
}

pub fn render_pair(gl: &glow::Context) -> (CanvasFrontRenderer, CanvasBackRenderer) {
    let (front, back) = rendering::glow::render_pair(gl);
    (
        CanvasFrontRenderer::new(front),
        CanvasBackRenderer::new(back),
    )
}

impl CanvasFrontRenderer {
    pub fn new(inner: GlowFrontRenderer) -> Self {
        Self { inner }
    }

    pub fn render(&self, gl: &glow::Context) {
        self.inner.render(gl);
    }
}

impl CanvasBackRenderer {
    pub fn new(inner: GlowBackRenderer) -> Self {
        Self {
            inner,
            keyboard_events: vec![],
            dirty: true,
        }
    }

    pub fn on_key_down(&mut self, code: &str) {
        self.keyboard_events
            .push(Event::KeyDown(keycode_from_native_code(code)));
    }

    pub fn on_key_up(&mut self, code: &str) {
        self.keyboard_events
            .push(Event::KeyUp(keycode_from_native_code(code)));
    }
}

impl Renderer for CanvasBackRenderer {
    type Color = <GlowBackRenderer as Renderer>::Color;

    fn poll_events(&mut self) -> Vec<Event> {
        mem::take(&mut self.keyboard_events)
    }

    fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
        unimplemented!()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        self.inner.color_pixel(x, y, color);
    }

    fn present(&mut self) {
        self.inner.present();
        self.dirty = true;
    }
}
