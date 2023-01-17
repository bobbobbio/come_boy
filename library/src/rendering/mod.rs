// Copyright 2019 Remi Bernotavicius

use crate::io;
use alloc::{string::String, vec, vec::Vec};
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "sdl2-renderer")]
pub mod sdl2;

#[cfg(feature = "speedy2d-renderer")]
pub mod speedy;

#[cfg(feature = "bitmap-renderer")]
pub mod bitmap;

#[cfg(feature = "glow-renderer")]
pub mod glow;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }
}

pub trait Renderer {
    fn poll_events(&mut self) -> Vec<Event>;
    fn save_buffer(&self, w: impl io::Write) -> io::Result<()>;
    fn color_pixel(&mut self, x: i32, y: i32, color: Color);
    fn present(&mut self);
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
    fn poll_events(&mut self) -> Vec<Event> {
        vec![]
    }
    fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
        Ok(())
    }
    fn color_pixel(&mut self, _: i32, _: i32, _: Color) {}
    fn present(&mut self) {}
}

impl<T: Renderer> Renderer for &mut T {
    fn poll_events(&mut self) -> Vec<Event> {
        (**self).poll_events()
    }

    fn save_buffer(&self, p: impl io::Write) -> io::Result<()> {
        (**self).save_buffer(p)
    }

    fn color_pixel(&mut self, x: i32, y: i32, c: Color) {
        (**self).color_pixel(x, y, c)
    }

    fn present(&mut self) {
        (**self).present()
    }
}

pub struct RenderingOptions {
    pub window_title: String,
    pub scale: u32,
    pub width: u32,
    pub height: u32,
    pub stop_on_ctrl_c: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            window_title: "come boy".into(),
            scale: 1,
            width: 160,
            height: 144,
            stop_on_ctrl_c: true,
        }
    }
}
