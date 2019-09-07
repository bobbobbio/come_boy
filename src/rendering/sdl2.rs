// Copyright 2019 Remi Bernotavicius

use super::{Color, Event, Keycode, Renderer, Result};
use std::path::Path;

impl From<sdl2::keyboard::Keycode> for Keycode {
    fn from(keycode: sdl2::keyboard::Keycode) -> Self {
        use sdl2::keyboard::Keycode::*;
        match keycode {
            Down => Self::Down,
            Left => Self::Left,
            Return => Self::Return,
            Right => Self::Right,
            Tab => Self::Tab,
            Up => Self::Up,
            X => Self::X,
            Z => Self::Z,
            _ => Self::Unknown,
        }
    }
}

pub struct Sdl2WindowRenderer {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pixel_scale: u32,
}

impl Sdl2WindowRenderer {
    pub fn new(pixel_scale: u32, title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width * pixel_scale, height * pixel_scale)
            .position_centered()
            .allow_highdpi()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();

        Self {
            event_pump: sdl_context.event_pump().unwrap(),
            canvas,
            pixel_scale,
        }
    }
}

impl Color for sdl2::pixels::Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        sdl2::pixels::Color::RGB(r, g, b)
    }
}

impl Renderer for Sdl2WindowRenderer {
    type Color = sdl2::pixels::Color;

    fn poll_events(&mut self) -> Vec<Event> {
        let mut events = vec![];
        for event in self.event_pump.poll_iter() {
            events.push(match event {
                sdl2::event::Event::Quit { .. } => Event::Quit,
                sdl2::event::Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => Event::KeyDown(code.into()),
                sdl2::event::Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => Event::KeyUp(code.into()),
                _ => {
                    continue;
                }
            });
        }
        events
    }

    fn save_buffer<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut pixels = self
            .canvas
            .read_pixels(None, sdl2::pixels::PixelFormatEnum::ABGR8888)?;
        let s = sdl2::surface::Surface::from_data(
            &mut pixels,
            160 * self.pixel_scale,
            140 * self.pixel_scale,
            160 * self.pixel_scale * 4,
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        s.save_bmp(path)?;
        Ok(())
    }

    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        self.canvas.set_draw_color(color);
        let rect = sdl2::rect::Rect::new(
            x * self.pixel_scale as i32,
            y * self.pixel_scale as i32,
            self.pixel_scale,
            self.pixel_scale,
        );
        self.canvas.fill_rect(rect).unwrap();
    }

    fn present(&mut self) {
        self.canvas.present()
    }
}
