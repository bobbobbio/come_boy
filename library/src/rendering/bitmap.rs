// Copyright 2021 Remi Bernotavicius

use super::{Color, Event, Renderer, RenderingOptions};
use crate::io;
use alloc::{vec, vec::Vec};
use bmp::{Image, Pixel};

pub struct BitmapRenderer {
    front: Image,
    back: Image,
}

impl BitmapRenderer {
    pub fn new(options: RenderingOptions) -> Self {
        let RenderingOptions { width, height, .. } = options;

        Self {
            front: Image::new(width, height),
            back: Image::new(width, height),
        }
    }
}

impl From<Color> for Pixel {
    fn from(c: Color) -> Self {
        Pixel::new(c.r, c.g, c.b)
    }
}

impl Renderer for BitmapRenderer {
    fn poll_events(&mut self) -> Vec<Event> {
        vec![]
    }

    fn save_buffer(&self, mut w: impl io::Write) -> io::Result<()> {
        self.front.to_writer(&mut w)?;
        Ok(())
    }

    fn color_pixel(&mut self, x: i32, y: i32, color: Color) {
        let width = self.back.get_width();
        let height = self.back.get_height();
        assert!(x < width as i32, "x = {x} > {width}");
        assert!(y < height as i32, "y = {y} > {height}");

        if x < 0 || y < 0 {
            return;
        }

        self.back.set_pixel(x as u32, y as u32, color.into());
    }

    fn present(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
}
