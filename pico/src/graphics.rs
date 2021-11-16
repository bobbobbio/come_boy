// copyright 2021 Remi Bernotavicius

use crate::picosystem;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl come_boy::rendering::Color for Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        fn conv(v: u8) -> u8 {
            ((v as u32) * 100 / 255 * 0xF / 100) as u8
        }
        Self::rgb(conv(r), conv(g), conv(b))
    }
}

pub struct Graphics {
    pub dirty: bool,
}

#[allow(dead_code)]
impl Graphics {
    pub const fn new() -> Self {
        Self { dirty: false }
    }

    #[inline(always)]
    pub fn set_pen(&self, color: Color) {
        unsafe { picosystem::pen(color.r, color.g, color.b) }
    }

    pub fn blend_copy(&self) {
        unsafe { picosystem::blend_copy() };
    }

    #[inline(always)]
    pub fn clear(&self) {
        unsafe { picosystem::clear() };
    }

    #[inline(always)]
    pub fn hline(&self, x: i32, y: i32, l: i32) {
        unsafe { picosystem::hline(x, y, l) }
    }

    #[inline(always)]
    pub fn pixel(&self, x: i32, y: i32) {
        unsafe { picosystem::pixel(x, y) }
    }

    pub fn text(&self, msg: &str) {
        assert!(msg.is_ascii());

        let mut buffer = vec![0; msg.len() + 1];
        buffer[..(msg.len())].clone_from_slice(msg.as_bytes());

        unsafe { picosystem::text(buffer.as_ptr() as *const i8, 3, 3) }
    }
}

impl come_boy::rendering::Renderer for Graphics {
    type Color = Color;

    fn poll_events(&mut self) -> Vec<come_boy::rendering::Event> {
        vec![]
    }

    fn save_buffer(&self, _w: impl come_boy::io::Write) -> come_boy::io::Result<()> {
        unimplemented!()
    }

    #[inline(always)]
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        if x < 0 || x >= 240 || y < 0 || y >= 240 {
            return;
        }

        self.set_pen(color);
        self.pixel(x, y);
    }

    fn present(&mut self) {
        self.dirty = true;
    }
}
