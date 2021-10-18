// Copyright 2019 Remi Bernotavicius

use super::memory_controller::MemoryMappedHardware;
use crate::rendering::Keycode;
use serde_derive::{Deserialize, Serialize};
use std::io::{Seek, SeekFrom};
use std::path::Path;

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
enum ButtonCode {
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
enum ButtonEvent {
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

    fn filter_events(&mut self, button_events: Vec<ButtonEvent>) -> Vec<ButtonEvent> {
        button_events
            .into_iter()
            .filter_map(|e| match e {
                ButtonEvent::Up(c) if *self.get_state(c) == ButtonState::Pressed => Some(e),
                ButtonEvent::Down(c) if *self.get_state(c) == ButtonState::NotPressed => Some(e),
                _ => None,
            })
            .collect()
    }

    fn respond_to_events(&mut self, button_events: Vec<ButtonEvent>) {
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

#[derive(Debug)]
pub enum ReplayError {
    Io(std::io::Error),
    DecodingError(bincode::Error),
}

type Result<T> = std::result::Result<T, ReplayError>;

impl From<std::io::Error> for ReplayError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<bincode::Error> for ReplayError {
    fn from(e: bincode::Error) -> Self {
        Self::DecodingError(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum ReplayFileVersion {
    Version1,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReplayFileHeader {
    version: ReplayFileVersion,
    game_pak_title: String,
    game_pak_hash: u32,
}

pub struct RecordingJoyPad {
    output_file: std::fs::File,
    inner: PlainJoyPad,
}

impl RecordingJoyPad {
    pub fn new<P: AsRef<Path>>(
        game_pak_title: &str,
        game_pak_hash: u32,
        output_path: P,
    ) -> Result<Self> {
        // XXX remi: Should use storage instead
        let mut output_file = std::fs::File::create(output_path)?;
        let header = ReplayFileHeader {
            version: ReplayFileVersion::Version1,
            game_pak_title: game_pak_title.into(),
            game_pak_hash,
        };
        bincode::serialize_into(&mut output_file, &header)?;

        Ok(Self {
            output_file,
            inner: PlainJoyPad::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ReplayFileEntry {
    time: u64,
    button_events: Vec<ButtonEvent>,
}

impl JoyPad for RecordingJoyPad {
    fn tick(&mut self, now: u64, key_events: Vec<KeyEvent>) {
        let button_events = button_events_from_key_events(key_events);
        let button_events = self.inner.filter_events(button_events);
        let entry = ReplayFileEntry {
            time: now,
            button_events,
        };

        if entry.button_events.len() > 0 {
            // XXX ignoring error.
            bincode::serialize_into(&mut self.output_file, &entry).ok();

            self.inner.respond_to_events(entry.button_events);
        }
    }
}

impl MemoryMappedHardware for RecordingJoyPad {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value)
    }
}

pub struct PlaybackJoyPad {
    input_file: std::fs::File,
    current_entry: Option<ReplayFileEntry>,
    inner: PlainJoyPad,
}

impl PlaybackJoyPad {
    pub fn new<P: AsRef<Path>>(game_pak_hash: u32, input_path: P) -> Result<Self> {
        let mut input_file = std::fs::File::open(input_path)?;
        let header: ReplayFileHeader = bincode::deserialize_from(&mut input_file)?;
        if header.game_pak_hash != game_pak_hash {
            println!(
                "Warning, replay hash mismatch. Replay recorded for {:?}",
                header.game_pak_title
            );
        }
        let current_entry = Some(bincode::deserialize_from(&mut input_file)?);
        Ok(Self {
            input_file,
            current_entry,
            inner: PlainJoyPad::new(),
        })
    }
}

impl JoyPad for PlaybackJoyPad {
    fn tick(&mut self, now: u64, _key_events: Vec<KeyEvent>) {
        while self.current_entry.is_some() && now >= self.current_entry.as_ref().unwrap().time {
            let current_entry = self.current_entry.take().unwrap();
            self.inner.respond_to_events(current_entry.button_events);
            self.current_entry = bincode::deserialize_from(&mut self.input_file).ok();
        }
    }
}

impl MemoryMappedHardware for PlaybackJoyPad {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value)
    }
}

pub fn print_replay<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let mut f = std::fs::File::open(file_path)?;
    let header: ReplayFileHeader = bincode::deserialize_from(&mut f)?;
    println!("{:#?}", header);

    let len = f.metadata()?.len();
    while f.seek(SeekFrom::Current(0))? < len {
        let entry: ReplayFileEntry = bincode::deserialize_from(&mut f)?;
        println!("{:#?}", entry);
    }
    Ok(())
}

pub struct ControllerJoyPad {
    inner: PlainJoyPad,
    gilrs: gilrs::Gilrs,
}

impl ControllerJoyPad {
    pub fn new() -> Self {
        Self {
            inner: PlainJoyPad::new(),
            gilrs: gilrs::Gilrs::new().unwrap(),
        }
    }
}

fn button_code_from_controller_button(button: gilrs::ev::Button) -> Option<ButtonCode> {
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
