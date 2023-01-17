// copyright 2021 Remi Bernotavicius

use crate::picosystem;
use alloc::vec;
use alloc::vec::Vec;

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 240;

#[derive(Clone, Copy)]
pub struct Color(u16);

impl Color {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        const fn conv(v: u8) -> u16 {
            ((v as u32) * 100 / 255 * 0xF / 100) as u16
        }
        Self(conv(r) | conv(b) << 8 | conv(g) << 12)
    }

    fn r(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    fn g(&self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }

    fn b(&self) -> u8 {
        ((self.0 >> 8) & 0xF) as u8
    }
}

impl From<come_boy::rendering::Color> for Color {
    fn from(color: come_boy::rendering::Color) -> Self {
        Color::rgb(color.r, color.g, color.b)
    }
}

pub struct Graphics {
    pub dirty: bool,
    target_buffer: *mut picosystem::buffer,
}

#[allow(dead_code)]
impl Graphics {
    pub fn new() -> Self {
        let target_buffer = unsafe { picosystem::target_buffer() };
        Self {
            dirty: false,
            target_buffer,
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_pen(&self, color: Color) {
        unsafe { picosystem::pen(color.r(), color.g(), color.b()) }
    }

    pub fn blend_copy(&self) {
        unsafe { picosystem::blend_copy() };
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn clear(&self) {
        unsafe { picosystem::clear() };
    }

    pub fn text(&self, msg: &str, x: i32, y: i32) {
        assert!(msg.is_ascii());

        let mut buffer = vec![0; msg.len() + 1];
        buffer[..(msg.len())].clone_from_slice(msg.as_bytes());

        unsafe { picosystem::text(buffer.as_ptr() as *const i8, x, y) }
    }

    pub fn filled_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { picosystem::frect(x, y, w, h) }
    }
}

impl come_boy::rendering::Renderer for Graphics {
    fn poll_events(&mut self) -> Vec<come_boy::rendering::Event> {
        vec![]
    }

    fn save_buffer(&self, _w: impl come_boy::io::Write) -> come_boy::io::Result<()> {
        unimplemented!()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn color_pixel(&mut self, x: i32, y: i32, color: come_boy::rendering::Color) {
        let color: Color = color.into();
        if x < 0 || x >= SCREEN_WIDTH as i32 || y < 0 || y >= SCREEN_HEIGHT as i32 {
            return;
        }
        let buffer = unsafe { &*self.target_buffer };
        let color_data = buffer.data as *mut u16;
        let color_data =
            unsafe { color_data.offset(SCREEN_WIDTH as isize * (y as isize) + x as isize) };
        unsafe { color_data.write(color.0) };
    }

    fn present(&mut self) {
        self.dirty = true;
    }
}
