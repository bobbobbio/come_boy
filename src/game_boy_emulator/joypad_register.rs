// Copyright 2019 Remi Bernotavicius

use super::memory_controller::MemoryMappedHardware;
use crate::rendering::Keycode;

#[derive(Debug)]
pub enum KeyEvent {
    Down(Keycode),
    Up(Keycode),
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
pub struct JoyPadRegister {
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

impl MemoryMappedHardware for JoyPadRegister {
    fn read_value(&self, _: u16) -> u8 {
        let select = match self.bank {
            KeyBank::ButtonKeys => JoypadFlag::SelectButtonKeys as u8,
            KeyBank::DirectionKeys => JoypadFlag::SelectDirectionKeys as u8,
            KeyBank::Both => {
                JoypadFlag::SelectButtonKeys as u8 | JoypadFlag::SelectDirectionKeys as u8
            }
            KeyBank::Neither => 0,
        };

        let buttons = match self.bank {
            KeyBank::ButtonKeys => self.button_state(),
            KeyBank::DirectionKeys => self.direction_state(),
            KeyBank::Both => self.button_state() | self.direction_state(),
            KeyBank::Neither => 0,
        };

        // When a bank is selected, or a button is pressed, the bit is unset;
        (0xFF & !select) & !buttons
    }

    fn set_value(&mut self, _: u16, value: u8) {
        self.bank = if value & JoypadFlag::SelectButtonKeys as u8 == 0
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
}

impl JoyPadRegister {
    pub fn new() -> Self {
        Default::default()
    }

    fn button_state(&self) -> u8 {
        (match self.start {
            ButtonState::Pressed => JoypadFlag::DownOrStart as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.select {
            ButtonState::Pressed => JoypadFlag::UpOrSelect as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.b {
            ButtonState::Pressed => JoypadFlag::LeftOrB as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.a {
            ButtonState::Pressed => JoypadFlag::RightOrA as u8,
            ButtonState::NotPressed => 0,
        })
    }

    fn direction_state(&self) -> u8 {
        (match self.down {
            ButtonState::Pressed => JoypadFlag::DownOrStart as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.up {
            ButtonState::Pressed => JoypadFlag::UpOrSelect as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.left {
            ButtonState::Pressed => JoypadFlag::LeftOrB as u8,
            ButtonState::NotPressed => 0,
        }) | (match self.right {
            ButtonState::Pressed => JoypadFlag::RightOrA as u8,
            ButtonState::NotPressed => 0,
        })
    }

    pub fn tick(&mut self, key_events: Vec<KeyEvent>) {
        for event in key_events {
            match event {
                KeyEvent::Up(Keycode::Z) => self.a = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Z) => self.a = ButtonState::Pressed,
                KeyEvent::Up(Keycode::X) => self.b = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::X) => self.b = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Return) => self.start = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Return) => self.start = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Tab) => self.select = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Tab) => self.select = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Up) => self.up = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Up) => self.up = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Down) => self.down = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Down) => self.down = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Left) => self.left = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Left) => self.left = ButtonState::Pressed,
                KeyEvent::Up(Keycode::Right) => self.right = ButtonState::NotPressed,
                KeyEvent::Down(Keycode::Right) => self.right = ButtonState::Pressed,
                _ => {}
            }
        }
    }
}
