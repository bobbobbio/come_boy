// Copyright 2019 Remi Bernotavicius

use super::{button_events_from_key_events, JoyPad, KeyEvent, MemoryMappedHardware, PlainJoyPad};
use alloc::{vec, vec::Vec};

pub struct ControllerJoyPad {
    inner: PlainJoyPad,
    #[cfg(feature = "gilrs")]
    gilrs: gilrs::Gilrs,
}

impl ControllerJoyPad {
    pub fn new() -> Self {
        Self {
            inner: PlainJoyPad::new(),
            #[cfg(feature = "gilrs")]
            gilrs: gilrs::Gilrs::new().unwrap(),
        }
    }

    #[cfg(feature = "gilrs")]
    fn read_gilrs_events(&mut self) -> Vec<super::ButtonEvent> {
        use super::{ButtonCode, ButtonEvent};

        let mut button_events = vec![];
        while let Some(event) = self.gilrs.next_event() {
            use gilrs::ev::{Axis, EventType};
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    if let Some(b) = button_code_from_controller_button(button) {
                        button_events.push(ButtonEvent::Down(b));
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if let Some(b) = button_code_from_controller_button(button) {
                        button_events.push(ButtonEvent::Up(b));
                    }
                }
                EventType::AxisChanged(Axis::LeftStickX, v, _) if v > 0.0 => {
                    button_events.push(ButtonEvent::Down(ButtonCode::Right));
                }
                EventType::AxisChanged(Axis::LeftStickX, v, _) if v < 0.0 => {
                    button_events.push(ButtonEvent::Down(ButtonCode::Left));
                }
                EventType::AxisChanged(Axis::LeftStickX, v, _) if v == 0.0 => {
                    button_events.push(ButtonEvent::Up(ButtonCode::Right));
                    button_events.push(ButtonEvent::Up(ButtonCode::Left));
                }
                EventType::AxisChanged(Axis::LeftStickY, v, _) if v > 0.0 => {
                    button_events.push(ButtonEvent::Down(ButtonCode::Up));
                }
                EventType::AxisChanged(Axis::LeftStickY, v, _) if v < 0.0 => {
                    button_events.push(ButtonEvent::Down(ButtonCode::Down));
                }
                EventType::AxisChanged(Axis::LeftStickY, v, _) if v == 0.0 => {
                    button_events.push(ButtonEvent::Up(ButtonCode::Up));
                    button_events.push(ButtonEvent::Up(ButtonCode::Down));
                }
                _ => (),
            };
        }

        button_events
    }
}

#[cfg(feature = "gilrs")]
fn button_code_from_controller_button(button: gilrs::ev::Button) -> Option<super::ButtonCode> {
    use super::ButtonCode;
    use gilrs::ev::Button;

    match button {
        Button::East => Some(ButtonCode::A),
        Button::South => Some(ButtonCode::B),
        Button::Start => Some(ButtonCode::Start),
        Button::Select => Some(ButtonCode::Select),
        Button::DPadUp => Some(ButtonCode::Up),
        Button::DPadDown => Some(ButtonCode::Down),
        Button::DPadLeft => Some(ButtonCode::Left),
        Button::DPadRight => Some(ButtonCode::Right),
        _ => None,
    }
}

impl JoyPad for ControllerJoyPad {
    fn tick(&mut self, _now: u64, key_events: Vec<KeyEvent>) {
        let mut button_events = vec![];

        #[cfg(feature = "gilrs")]
        button_events.extend(self.read_gilrs_events());

        if cfg!(not(feature = "gilrs")) {
            panic!("No controller backend");
        }

        button_events.extend(button_events_from_key_events(key_events));
        let button_events = self.inner.filter_events(button_events);
        self.inner.respond_to_events(button_events);
    }
}

impl MemoryMappedHardware for ControllerJoyPad {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value)
    }
}
