// Copyright 2019 Remi Bernotavicius

use super::memory_controller::MemoryMappedHardware;
use crate::rendering::Keycode;
use alloc::{vec, vec::Vec};
use serde_derive::{Deserialize, Serialize};

pub use controller::ControllerJoyPad;
pub use plain::PlainJoyPad;
pub use replay::{PlaybackJoyPad, RecordingJoyPad};

mod controller;
mod plain;
pub mod replay;

pub trait JoyPad: MemoryMappedHardware {
    fn tick(&mut self, now: u64, key_events: Vec<KeyEvent>);
}

impl MemoryMappedHardware for &dyn JoyPad {
    fn read_value(&self, address: u16) -> u8 {
        (**self).read_value(address)
    }

    fn set_value(&mut self, _address: u16, _value: u8) {
        panic!("can't write to &dyn JoyPad")
    }
}

impl MemoryMappedHardware for &mut dyn JoyPad {
    fn read_value(&self, address: u16) -> u8 {
        (**self).read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        (**self).set_value(address, value)
    }
}

impl JoyPad for &mut dyn JoyPad {
    fn tick(&mut self, now: u64, key_events: Vec<KeyEvent>) {
        (*self).tick(now, key_events)
    }
}

#[derive(Serialize, Deserialize)]
pub enum KeyEvent {
    Down(Keycode),
    Up(Keycode),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ButtonCode {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ButtonEvent {
    Down(ButtonCode),
    Up(ButtonCode),
}

fn button_code_from_key_code(code: Keycode) -> Option<ButtonCode> {
    match code {
        Keycode::Z => Some(ButtonCode::A),
        Keycode::X => Some(ButtonCode::B),
        Keycode::Return => Some(ButtonCode::Start),
        Keycode::Tab => Some(ButtonCode::Select),
        Keycode::Up => Some(ButtonCode::Up),
        Keycode::Down => Some(ButtonCode::Down),
        Keycode::Left => Some(ButtonCode::Left),
        Keycode::Right => Some(ButtonCode::Right),
        _ => None,
    }
}

fn button_events_from_key_events(key_events: Vec<KeyEvent>) -> Vec<ButtonEvent> {
    let mut new_events = vec![];
    for event in key_events {
        let code = match &event {
            KeyEvent::Up(c) => *c,
            KeyEvent::Down(c) => *c,
        };
        if let Some(new_code) = button_code_from_key_code(code) {
            match event {
                KeyEvent::Up(_) => new_events.push(ButtonEvent::Up(new_code)),
                KeyEvent::Down(_) => new_events.push(ButtonEvent::Down(new_code)),
            }
        }
    }

    new_events
}

enum JoypadFlag {
    SelectButtonKeys = 0b00100000,
    SelectDirectionKeys = 0b00010000,
    DownOrStart = 0b00001000,
    UpOrSelect = 0b00000100,
    LeftOrB = 0b00000010,
    RightOrA = 0b00000001,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum ButtonState {
    Pressed,
    #[default]
    NotPressed,
}
