// Copyright 2019 Remi Bernotavicius

use super::memory_controller::MemoryMappedHardware;
use crate::sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum KeyEvent {
    Down(sdl2::keyboard::Keycode),
    Up(sdl2::keyboard::Keycode),
}

enum JoypadFlag {
    SelectButtonKeys = 0b00100000,
    SelectDirectionKeys = 0b00010000,
    DownOrStart = 0b00001000,
    UpOrSelect = 0b00000100,
    LeftOrB = 0b00000010,
    RightOrA = 0b00000001,
}

#[derive(Clone, Copy, PartialEq)]
enum ButtonState {
    Pressed,
    NotPressed,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::NotPressed
    }
}

#[derive(Clone, Copy, PartialEq)]
enum KeyBank {
    Neither,
    Both,
    ButtonKeys,
    DirectionKeys,
}

impl Default for KeyBank {
    fn default() -> Self {
        KeyBank::Both
    }
}

#[derive(Default)]
struct JoyPadState {
    a: ButtonState,
    b: ButtonState,
    start: ButtonState,
    select: ButtonState,
    up: ButtonState,
    down: ButtonState,
    left: ButtonState,
    right: ButtonState,
    bank: KeyBank,
}

#[derive(Default, Clone)]
pub struct JoyPadRegister {
    state: Rc<RefCell<JoyPadState>>,
}

impl MemoryMappedHardware for JoyPadRegister {
    fn read_value(&self, _: u16) -> u8 {
        let state = self.state.borrow();
        let select = match state.bank {
            KeyBank::ButtonKeys => JoypadFlag::SelectButtonKeys as u8,
            KeyBank::DirectionKeys => JoypadFlag::SelectDirectionKeys as u8,
            KeyBank::Both => {
                JoypadFlag::SelectButtonKeys as u8 | JoypadFlag::SelectDirectionKeys as u8
            }
            KeyBank::Neither => 0,
        };

        fn button_state(state: &JoyPadState) -> u8 {
            (match state.start {
                ButtonState::Pressed => JoypadFlag::DownOrStart as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.select {
                ButtonState::Pressed => JoypadFlag::UpOrSelect as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.b {
                ButtonState::Pressed => JoypadFlag::LeftOrB as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.a {
                ButtonState::Pressed => JoypadFlag::RightOrA as u8,
                ButtonState::NotPressed => 0,
            })
        }

        fn direction_state(state: &JoyPadState) -> u8 {
            (match state.down {
                ButtonState::Pressed => JoypadFlag::DownOrStart as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.up {
                ButtonState::Pressed => JoypadFlag::UpOrSelect as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.left {
                ButtonState::Pressed => JoypadFlag::LeftOrB as u8,
                ButtonState::NotPressed => 0,
            }) | (match state.right {
                ButtonState::Pressed => JoypadFlag::RightOrA as u8,
                ButtonState::NotPressed => 0,
            })
        }

        let buttons = match state.bank {
            KeyBank::ButtonKeys => button_state(&state),
            KeyBank::DirectionKeys => direction_state(&state),
            KeyBank::Both => button_state(&state) | direction_state(&state),
            KeyBank::Neither => 0,
        };

        // When a bank is selected, or a button is pressed, the bit is unset;
        (0xFF & !select) & !buttons
    }

    fn set_value(&mut self, _: u16, value: u8) {
        let mut state = self.state.borrow_mut();
        state.bank = if value & JoypadFlag::SelectButtonKeys as u8 == 0
            && value & JoypadFlag::SelectDirectionKeys as u8 == 0
        {
            KeyBank::Both
        } else if value & JoypadFlag::SelectButtonKeys as u8 == 0 {
            KeyBank::ButtonKeys
        } else if value & JoypadFlag::SelectDirectionKeys as u8 == 0 {
            KeyBank::DirectionKeys
        } else {
            KeyBank::Neither
        };
    }

    fn len(&self) -> u16 {
        1
    }
}

impl JoyPadRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick(&mut self, key_events: Vec<KeyEvent>) {
        let mut state = self.state.borrow_mut();
        for event in key_events {
            match event {
                KeyEvent::Up(Keycode::Z) => state.a = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Z) => state.a = ButtonState::Pressed,
                KeyEvent::Up(Keycode::X) => state.b = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::X) => state.b = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Return) => state.start = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Return) => state.start = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Tab) => state.select = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Tab) => state.select = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Up) => state.up = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Up) => state.up = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Down) => state.down = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Down) => state.down = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Left) => state.left = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Left) => state.left = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Right) => state.right = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Right) => state.right = ButtonState::Pressed,
                _ => {}
            }
        }
    }
}
