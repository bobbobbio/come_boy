// Copyright 2019 Remi Bernotavicius

use super::{
    button_events_from_key_events, ButtonEvent, JoyPad, KeyEvent, MemoryMappedHardware, PlainJoyPad,
};
use crate::io::{self, Write as _};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DecodingError(crate::codec::Error),
}

type Result<T> = core::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::codec::Error> for Error {
    fn from(e: crate::codec::Error) -> Self {
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
        crate::codec::serialize_into(&mut output_file, &header)?;

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
            crate::codec::serialize_into(&mut self.output_file, &entry).ok();

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
        let header: ReplayFileHeader = crate::codec::deserialize_from(&mut input_file)?;
        if header.game_pak_hash != game_pak_hash {
            log::warn!(
                "Warning, replay hash mismatch. Replay recorded for {:?}",
                header.game_pak_title
            );
        }
        let current_entry = Some(crate::codec::deserialize_from(&mut input_file)?);
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
            self.current_entry = crate::codec::deserialize_from(&mut self.input_file).ok();
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

pub fn print(mut input: impl io::Read) -> Result<String> {
    let mut out = vec![];

    let header: ReplayFileHeader = crate::codec::deserialize_from(&mut input)?;
    writeln!(&mut out, "{:#?}", header)?;

    while let Ok(entry) = crate::codec::deserialize_from::<_, ReplayFileEntry>(&mut input) {
        write!(&mut out, "{:#?}", entry)?;
    }

    Ok(String::from_utf8(out).unwrap())
}
