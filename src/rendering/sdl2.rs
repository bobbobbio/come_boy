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
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

impl Sdl2WindowRenderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        Self {
            video_subsystem: sdl_context.video().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            canvas: None,
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

    fn start(&mut self, pixel_scale: u32) {
        let window = self
            .video_subsystem
            .window("come boy", 160 * pixel_scale, 144 * pixel_scale)
            .position_centered()
            .allow_highdpi()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();

        self.canvas = Some(canvas);
    }

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

    fn save_buffer<P: AsRef<Path>>(&self, path: P, pixel_scale: u32) -> Result<()> {
        let mut pixels = self
            .canvas
            .as_ref()
            .unwrap()
            .read_pixels(None, sdl2::pixels::PixelFormatEnum::ABGR8888)?;
        let s = sdl2::surface::Surface::from_data(
            &mut pixels,
            160 * pixel_scale,
            140 * pixel_scale,
            160 * pixel_scale * 4,
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        s.save_bmp(path)?;
        Ok(())
    }

    fn set_draw_color(&mut self, color: Self::Color) {
        if let Some(ref mut c) = self.canvas {
            c.set_draw_color(color)
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        if let Some(ref mut c) = self.canvas {
            c.fill_rect(sdl2::rect::Rect::new(x, y, w, h)).unwrap();
        }
    }

    fn present(&mut self) {
        if let Some(ref mut c) = self.canvas {
            c.present()
        }
    }
}
