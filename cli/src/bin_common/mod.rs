// Copyright 2021 Remi Bernotavicius

use come_boy::game_boy_emulator;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod backend;
pub mod frontend;

#[allow(dead_code)] // Debug use of field doesn't count as a usage
#[derive(Debug)]
pub enum Error {
    Emulator(game_boy_emulator::Error),
    Backend(backend::Error),
    Io(std::io::Error),
}

impl From<game_boy_emulator::Error> for Error {
    fn from(error: game_boy_emulator::Error) -> Self {
        Self::Emulator(error)
    }
}

impl From<backend::Error> for Error {
    fn from(error: backend::Error) -> Self {
        Self::Backend(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn read_save_state(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    Ok(contents)
}
