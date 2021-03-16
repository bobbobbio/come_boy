// Copyright 2019 Remi Bernotavicius

use serde_derive::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[cfg(feature = "sdl2")]
pub mod sdl2;

#[cfg(feature = "speedy2d")]
pub mod speedy;

pub mod bitmap;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Keycode {
    Down,
    F2,
    F3,
    F4,
    Left,
    Return,
    Right,
    Tab,
    Unknown,
    Up,
    X,
    Z,
}

pub enum Event {
    #[allow(unused)]
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
    fn save_buffer<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color);
    fn present(&mut self);
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
    type Color = ();

    fn poll_events(&mut self) -> Vec<Event> {
        vec![]
    }
    fn save_buffer<P: AsRef<Path>>(&self, _: P) -> io::Result<()> {
        Ok(())
    }
    fn color_pixel(&mut self, _: i32, _: i32, _: Self::Color) {}
    fn present(&mut self) {}
}

impl Color for () {
    fn new(_: u8, _: u8, _: u8) {}
}

pub struct RenderingOptions {
    pub window_title: String,
    pub scale: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            window_title: "come boy".into(),
            scale: 1,
            width: 160,
            height: 144,
        }
    }
}
