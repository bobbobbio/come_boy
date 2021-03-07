// Copyright 2021 Remi Bernotavicius

use super::{Color, Event, Keycode, Renderer, Result};
use speedy2d::{
    color::Color as SpeedyColor,
    shape::Rectangle,
    window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D, Window,
};
use std::ops::Deref;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ScreenBuffer {
    buffer: Arc<Mutex<Vec<SpeedyColor>>>,
}

impl ScreenBuffer {
    fn buffer<'a>(&'a self) -> impl Deref<Target = Vec<SpeedyColor>> + 'a {
        self.buffer.lock().unwrap()
    }

    fn swap(&mut self, other: &mut Vec<SpeedyColor>) {
        let mut buffer = self.buffer.lock().unwrap();
        std::mem::swap(&mut *buffer, other);
    }
}

struct SpeedyWindowHandler {
    buffer: ScreenBuffer,
    width: usize,
    height: usize,
    pixel_scale: usize,
    events: Sender<Event>,
}

impl SpeedyWindowHandler {
    fn update_screen(&self, buffer: &[SpeedyColor], graphics: &mut Graphics2D) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = buffer[y * self.width + x];
                let width = self.pixel_scale as f32;
                let height = self.pixel_scale as f32;
                let x = x as f32 * width;
                let y = y as f32 * height;
                let rect = Rectangle::from_tuples((x, y), (x + width, y + height));
                graphics.draw_rectangle(rect, color);
            }
        }
    }
}

fn keycode_translate(keycode: VirtualKeyCode) -> Keycode {
    match keycode {
        VirtualKeyCode::Down => Keycode::Down,
        VirtualKeyCode::Left => Keycode::Left,
        VirtualKeyCode::Return => Keycode::Return,
        VirtualKeyCode::Right => Keycode::Right,
        VirtualKeyCode::Tab => Keycode::Tab,
        VirtualKeyCode::Up => Keycode::Up,
        VirtualKeyCode::X => Keycode::X,
        VirtualKeyCode::Z => Keycode::Z,
        _ => Keycode::Unknown,
    }
}

impl WindowHandler for SpeedyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.update_screen(&*self.buffer.buffer(), graphics);
        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(keycode) = virtual_key_code {
            let keycode = keycode_translate(keycode);
            self.events.send(Event::KeyDown(keycode)).unwrap();
        }
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(keycode) = virtual_key_code {
            let keycode = keycode_translate(keycode);
            self.events.send(Event::KeyUp(keycode)).unwrap();
        }
    }
}

pub fn run_loop<F: FnOnce(&mut SpeedyRenderer) + Send>(
    pixel_scale: u32,
    title: &str,
    width: u32,
    height: u32,
    body: F,
) {
    let window = Window::new_centered(title, (width * pixel_scale, height * pixel_scale)).unwrap();

    let base_buffer = vec![SpeedyColor::WHITE; width as usize * height as usize];
    let screen_buffer = ScreenBuffer {
        buffer: Arc::new(Mutex::new(base_buffer.clone())),
    };

    let (sender, receiver) = channel();

    let mut renderer = SpeedyRenderer {
        screen_buffer: screen_buffer.clone(),
        back_buffer: base_buffer,
        width: width as usize,
        height: height as usize,
        events: receiver,
    };

    let handler = SpeedyWindowHandler {
        buffer: screen_buffer,
        width: width as usize,
        height: height as usize,
        pixel_scale: pixel_scale as usize,
        events: sender,
    };

    crossbeam::scope(|scope| {
        scope.spawn(|_| body(&mut renderer));
        window.run_loop(handler);
    })
    .unwrap();
}

impl Color for SpeedyColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        SpeedyColor::from_int_rgb(r, g, b)
    }
}

pub struct SpeedyRenderer {
    screen_buffer: ScreenBuffer,
    back_buffer: Vec<SpeedyColor>,
    width: usize,
    height: usize,
    events: Receiver<Event>,
}

impl Renderer for SpeedyRenderer {
    type Color = SpeedyColor;

    fn poll_events(&mut self) -> Vec<Event> {
        let mut events = vec![];
        while let Ok(event) = self.events.try_recv() {
            events.push(event);
        }
        events
    }

    fn save_buffer<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        unimplemented!()
    }

    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        assert!(x < self.width as i32, "x = {} > {}", x, self.width);
        assert!(y < self.height as i32, "y = {} > {}", x, self.height);

        if x < 0 || y < 0 {
            return;
        }
        self.back_buffer[y as usize * self.width + x as usize] = color;
    }

    fn present(&mut self) {
        self.screen_buffer.swap(&mut self.back_buffer);
        self.back_buffer.fill(SpeedyColor::WHITE);
    }
}
