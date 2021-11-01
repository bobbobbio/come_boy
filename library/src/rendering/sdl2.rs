// Copyright 2019 Remi Bernotavicius

use super::{Color, Event, Keycode, Renderer, RenderingOptions};
use crate::io;
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
            F2 => Self::F2,
            F3 => Self::F3,
            F4 => Self::F4,
            _ => Self::Unknown,
        }
    }
}

pub struct Sdl2WindowRenderer {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    scale: u32,
    width: u32,
    height: u32,
}

impl Sdl2WindowRenderer {
    pub fn new(options: RenderingOptions) -> Self {
        let RenderingOptions {
            window_title,
            scale,
            width,
            height,
            ..
        } = options;
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&window_title, width * scale, height * scale)
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
            scale,
            width,
            height,
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

    fn save_buffer(&self, mut w: impl io::Write) -> io::Result<()> {
        let mut pixels = self
            .canvas
            .read_pixels(None, sdl2::pixels::PixelFormatEnum::ABGR8888)
            .unwrap();
        const BIT_DEPTH: u32 = 4;
        let s = sdl2::surface::Surface::from_data(
            &mut pixels,
            160 * self.scale,
            140 * self.scale,
            160 * self.scale * BIT_DEPTH,
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )
        .unwrap();

        let mut buf = vec![0; (160 * 140 * self.scale * BIT_DEPTH) as usize];
        let mut rw_ops = sdl2::rwops::RWops::from_bytes_mut(&mut buf).unwrap();
        s.save_bmp_rw(&mut rw_ops).unwrap();
        drop(rw_ops);
        w.write_all(&buf)?;

        Ok(())
    }

    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        assert!(x < self.width as i32, "x = {} > {}", x, self.width);
        assert!(y < self.height as i32, "y = {} > {}", x, self.height);

        self.canvas.set_draw_color(color);
        let rect = sdl2::rect::Rect::new(
            x * self.scale as i32,
            y * self.scale as i32,
            self.scale,
            self.scale,
        );
        self.canvas.fill_rect(rect).unwrap();
    }

    fn present(&mut self) {
        self.canvas.present()
    }
}
