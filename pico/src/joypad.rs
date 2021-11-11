// copyright 2021 Remi Bernotavicius

use alloc::vec;
use alloc::vec::Vec;
use come_boy::game_boy_emulator::{
    joypad::{ButtonCode, ButtonEvent, JoyPad, KeyEvent, PlainJoyPad},
    MemoryMappedHardware,
};
use enum_iterator::IntoEnumIterator;

pub struct PicoJoyPad {
    inner: PlainJoyPad,
    last_button_state: [bool; MAX_BUTTON + 1],
}

impl PicoJoyPad {
    pub fn new() -> Self {
        Self {
            inner: PlainJoyPad::new(),
            last_button_state: [false; MAX_BUTTON + 1],
        }
    }
}

impl MemoryMappedHardware for PicoJoyPad {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value)
    }
}

impl JoyPad for PicoJoyPad {
    fn tick(&mut self, _now: u64, _key_events: Vec<KeyEvent>) {
        let mut events = vec![];
        for b in Button::into_enum_iter() {
            if b.pressed() {
                self.last_button_state[b as usize] = true;
                events.push(ButtonEvent::Down(b.into()));
            } else if self.last_button_state[b as usize] {
                self.last_button_state[b as usize] = false;
                events.push(ButtonEvent::Up(b.into()));
            }
        }
        self.inner.respond_to_events(events);
    }
}

#[derive(IntoEnumIterator, Copy, Clone)]
#[repr(u32)]
enum Button {
    Up = 23,
    Down = 20,
    Left = 22,
    Right = 21,
    A = 18,
    B = 19,
    X = 17,
    Y = 16,
}

const MAX_BUTTON: usize = 23;

impl Button {
    fn pressed(&self) -> bool {
        unsafe { crate::picosystem::button(*self as u32) }
    }
}

impl From<Button> for ButtonCode {
    fn from(button: Button) -> ButtonCode {
        match button {
            Button::Up => Self::Up,
            Button::Down => Self::Down,
            Button::Left => Self::Left,
            Button::Right => Self::Right,
            Button::A => Self::A,
            Button::B => Self::B,
            Button::Y => Self::Select,
            Button::X => Self::Start,
        }
    }
}
