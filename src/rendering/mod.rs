// Copyright 2019 Remi Bernotavicius

use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[cfg(feature = "sdl2")]
pub mod sdl2;

#[cfg(feature = "speedy2d")]
pub mod speedy;

#[derive(Debug)]
pub struct Error(String);

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Keycode {
    Down,
    F2,
    F3,
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
    fn save_buffer<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color);
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
    fn color_pixel(&mut self, _: i32, _: i32, _: Self::Color) {}
    fn present(&mut self) {}
}

impl Color for () {
    fn new(_: u8, _: u8, _: u8) {}
}
