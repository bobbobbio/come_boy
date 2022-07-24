// Copyright 2021 Remi Bernotavicius

use super::{Color, Event, Keycode, Renderer, RenderingOptions};
use crate::io;
use alloc::{vec, vec::Vec};
use speedy2d::{
    image::{ImageDataType, ImageSmoothingMode},
    shape::Rectangle,
    window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D, Window,
};
use std::ops::Deref;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct SpeedyColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
struct ScreenBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl ScreenBuffer {
    fn buffer(&self) -> impl Deref<Target = Vec<u8>> + '_ {
        self.buffer.lock().unwrap()
    }

    fn swap(&mut self, other: &mut Vec<u8>) {
        let mut buffer = self.buffer.lock().unwrap();
        std::mem::swap(&mut *buffer, other);
    }
}

struct SpeedyWindowHandler {
    buffer: ScreenBuffer,
    width: usize,
    height: usize,
    scale: usize,
    events: Sender<Event>,
    done: Receiver<()>,
}

impl SpeedyWindowHandler {
    fn update_screen(&self, buffer: &[u8], graphics: &mut Graphics2D) {
        let image = graphics
            .create_image_from_raw_pixels(
                ImageDataType::RGB,
                ImageSmoothingMode::NearestNeighbor,
                (self.width as u32, self.height as u32),
                buffer,
            )
            .unwrap();
        let width = (self.width * self.scale) as f32;
        let height = (self.height * self.scale) as f32;
        graphics.draw_rectangle_image(Rectangle::from_tuples((0.0, 0.0), (width, height)), &image);
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
        VirtualKeyCode::F2 => Keycode::F2,
        VirtualKeyCode::F3 => Keycode::F3,
        VirtualKeyCode::F4 => Keycode::F4,
        _ => Keycode::Unknown,
    }
}

impl WindowHandler for SpeedyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.update_screen(&self.buffer.buffer(), graphics);
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

impl Drop for SpeedyWindowHandler {
    fn drop(&mut self) {
        self.events.send(Event::Quit).unwrap();
        self.done.recv().unwrap();
    }
}

impl Drop for SpeedyRenderer {
    fn drop(&mut self) {
        self.done.send(()).unwrap();
    }
}

pub fn run_loop<F: FnOnce(&mut SpeedyRenderer) + Send>(options: RenderingOptions, body: F) -> ! {
    let RenderingOptions {
        window_title,
        scale,
        width,
        height,
        ..
    } = options;

    let window = Window::new_centered(&window_title, (width * scale, height * scale)).unwrap();

    let base_buffer = vec![u8::MAX; width as usize * height as usize * 3];
    let screen_buffer = ScreenBuffer {
        buffer: Arc::new(Mutex::new(base_buffer.clone())),
    };

    let (events_input, events_output) = channel();
    let (done_input, done_output) = channel();

    let mut renderer = SpeedyRenderer {
        screen_buffer: screen_buffer.clone(),
        back_buffer: base_buffer,
        width: width as usize,
        height: height as usize,
        events: events_output,
        done: done_input,
    };

    let handler = SpeedyWindowHandler {
        buffer: screen_buffer,
        width: width as usize,
        height: height as usize,
        scale: scale as usize,
        events: events_input,
        done: done_output,
    };

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            body(&mut renderer);
            drop(renderer);
        });
        window.run_loop(handler)
    })
    .unwrap()
}

impl Color for SpeedyColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        SpeedyColor { r, g, b }
    }
}

pub struct SpeedyRenderer {
    screen_buffer: ScreenBuffer,
    back_buffer: Vec<u8>,
    width: usize,
    height: usize,
    events: Receiver<Event>,
    done: Sender<()>,
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

    fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
        unimplemented!()
    }

    #[inline(always)]
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        assert!(x < self.width as i32, "x = {} > {}", x, self.width);
        assert!(y < self.height as i32, "y = {} > {}", y, self.height);
        assert!(x >= 0, "x = {} > 0", x);
        assert!(y >= 0, "y = {} > 0", y);

        let i = (y as usize * self.width + x as usize) * 3;
        self.back_buffer[i] = color.r;
        self.back_buffer[i + 1] = color.g;
        self.back_buffer[i + 2] = color.b;
    }

    fn present(&mut self) {
        self.screen_buffer.swap(&mut self.back_buffer);
    }
}
