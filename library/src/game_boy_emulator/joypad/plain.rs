// Copyright 2019 Remi Bernotavicius

use super::{
    button_events_from_key_events, ButtonCode, ButtonEvent, ButtonState, JoyPad, JoypadFlag,
    KeyEvent, MemoryMappedHardware,
};
use alloc::vec::Vec;

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
pub struct PlainJoyPad {
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

impl MemoryMappedHardware for PlainJoyPad {
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

impl PlainJoyPad {
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

    fn get_state(&mut self, code: ButtonCode) -> &mut ButtonState {
        match code {
            ButtonCode::A => &mut self.a,
            ButtonCode::B => &mut self.b,
            ButtonCode::Start => &mut self.start,
            ButtonCode::Select => &mut self.select,
            ButtonCode::Up => &mut self.up,
            ButtonCode::Down => &mut self.down,
            ButtonCode::Left => &mut self.left,
            ButtonCode::Right => &mut self.right,
        }
    }

    pub(super) fn filter_events(&mut self, button_events: Vec<ButtonEvent>) -> Vec<ButtonEvent> {
        button_events
            .into_iter()
            .filter_map(|e| match e {
                ButtonEvent::Up(c) if *self.get_state(c) == ButtonState::Pressed => Some(e),
                ButtonEvent::Down(c) if *self.get_state(c) == ButtonState::NotPressed => Some(e),
                _ => None,
            })
            .collect()
    }

    pub fn respond_to_events(&mut self, button_events: Vec<ButtonEvent>) {
        for event in button_events {
            match event {
                ButtonEvent::Up(c) => *self.get_state(c) = ButtonState::NotPressed,
                ButtonEvent::Down(c) => *self.get_state(c) = ButtonState::Pressed,
            }
        }
    }
}

impl JoyPad for PlainJoyPad {
    fn tick(&mut self, _now: u64, key_events: Vec<KeyEvent>) {
        let button_events = button_events_from_key_events(key_events);
        let button_events = self.filter_events(button_events);
        self.respond_to_events(button_events);
    }
}
