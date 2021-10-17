// copyright 2021 Remi Bernotavicius

use super::PersistentStorage;
use std::{fs, io};

pub struct Fs;

impl Fs {
    pub fn new() -> Self {
        Self
    }
}

impl PersistentStorage for Fs {
    type Stream = fs::File;

    fn save(&mut self, key: &str) -> io::Result<Self::Stream> {
        fs::OpenOptions::new().write(true).create(true).open(key)
    }

    fn load(&mut self, key: &str) -> io::Result<Self::Stream> {
        fs::OpenOptions::new().read(true).open(key)
    }
}
