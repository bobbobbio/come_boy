// Copyright 2019 Remi Bernotavicius

use std::path::Path;

pub mod sdl2;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub enum Keycode {
    Down,
    Left,
    Return,
    Right,
    Tab,
    Up,
    X,
    Z,
    Unknown,
}

pub enum Event {
    Quit,
    KeyDown(Keycode),
    KeyUp(Keycode),
}

pub trait Color {
    fn new(r: u8, g: u8, b: u8) -> Self;
}

pub trait Renderer {
    type Color: Color;
    fn poll_events(&mut self) -> Vec<Event>;
    fn save_buffer<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    fn set_draw_color(&mut self, color: Self::Color);
    fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32);
    fn present(&mut self);
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
    type Color = ();

    fn poll_events(&mut self) -> Vec<Event> {
        vec![]
    }
    fn save_buffer<P: AsRef<Path>>(&self, _: P) -> Result<()> {
        Ok(())
    }
    fn set_draw_color(&mut self, _: Self::Color) {}
    fn fill_rect(&mut self, _: i32, _: i32, _: u32, _: u32) {}
    fn present(&mut self) {}
}

impl Color for () {
    fn new(_: u8, _: u8, _: u8) {}
}
