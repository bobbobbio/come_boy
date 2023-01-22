// copyright 2021 Remi Bernotavicius

pub use super::glow::{HEIGHT, PIXEL_SIZE, WIDTH};
use super::{
    glow::{GlowBackRenderer, GlowFrontRenderer},
    Color, Event, Keycode, Renderer,
};
use crate::io;
use alloc::{collections::BTreeSet, vec, vec::Vec};
use enum_iterator::IntoEnumIterator as _;

pub struct EguiFrontRenderer {
    renderer: GlowFrontRenderer,
}

pub struct EguiBackRenderer {
    renderer: GlowBackRenderer,
    pressed_keys: BTreeSet<Keycode>,
    ctx: egui::Context,
}

pub fn render_pair(
    ctx: egui::Context,
    gl: &glow::Context,
) -> (EguiFrontRenderer, EguiBackRenderer) {
    let (front_renderer, back_renderer) = super::glow::render_pair(gl);
    (
        EguiFrontRenderer::new(front_renderer),
        EguiBackRenderer::new(back_renderer, ctx),
    )
}

fn try_translate_keycode(code: Keycode) -> Result<egui::Key, &'static str> {
    match code {
        Keycode::Down => Ok(egui::Key::ArrowDown),
        Keycode::F2 => Ok(egui::Key::F2),
        Keycode::F3 => Ok(egui::Key::F3),
        Keycode::F4 => Ok(egui::Key::F4),
        Keycode::Left => Ok(egui::Key::ArrowLeft),
        Keycode::Return => Ok(egui::Key::Enter),
        Keycode::Right => Ok(egui::Key::ArrowRight),
        Keycode::Tab => Ok(egui::Key::Tab),
        Keycode::Unknown => Err("unknown"),
        Keycode::Up => Ok(egui::Key::ArrowUp),
        Keycode::X => Ok(egui::Key::X),
        Keycode::Z => Ok(egui::Key::Z),
    }
}

impl EguiFrontRenderer {
    pub fn new(renderer: GlowFrontRenderer) -> Self {
        Self { renderer }
    }

    pub fn render(&self, gl: &glow::Context) {
        self.renderer.render(gl);
    }
}

impl EguiBackRenderer {
    pub fn new(renderer: GlowBackRenderer, ctx: egui::Context) -> Self {
        Self {
            renderer,
            pressed_keys: BTreeSet::new(),
            ctx,
        }
    }
}

impl Renderer for EguiBackRenderer {
    fn poll_events(&mut self) -> Vec<Event> {
        let mut events = vec![];
        for key in Keycode::into_enum_iter() {
            if let Ok(egui_key) = try_translate_keycode(key) {
                if self.ctx.input().key_down(egui_key) {
                    if self.pressed_keys.insert(key) {
                        events.push(Event::KeyDown(key));
                    }
                } else if self.pressed_keys.remove(&key) {
                    events.push(Event::KeyUp(key));
                }
            }
        }
        events
    }

    fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
        unimplemented!()
    }

    #[inline(always)]
    fn color_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.renderer.color_pixel(x, y, color);
    }

    fn present(&mut self) {
        self.renderer.present();
    }
}
