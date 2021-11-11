// copyright 2021 Remi Bernotavicius

use crate::picosystem;
use alloc::vec;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Graphics;

impl Graphics {
    pub const fn new() -> Self {
        Self
    }

    pub fn set_pen(&self, color: Color) {
        unsafe { picosystem::pen(color.r, color.g, color.b, color.a) }
    }

    pub fn clear(&self) {
        unsafe { picosystem::clear() };
    }

    pub fn hline(&self, x: i32, y: i32, l: i32) {
        unsafe { picosystem::hline(x, y, l) }
    }

    pub fn text(&self, msg: &str) {
        assert!(msg.is_ascii());

        let mut buffer = vec![0; msg.len() + 1];
        buffer[..(msg.len())].clone_from_slice(msg.as_bytes());

        unsafe { picosystem::text(buffer.as_ptr() as *const i8, 3, 3) }
    }
}
