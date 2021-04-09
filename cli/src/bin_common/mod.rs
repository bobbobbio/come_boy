// Copyright 2021 Remi Bernotavicius

use come_boy::game_boy_emulator;

pub mod backend;
pub mod frontend;

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
