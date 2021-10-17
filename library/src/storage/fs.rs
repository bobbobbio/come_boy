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

    fn save(&mut self, _key: &str) -> io::Result<Self::Stream> {
        panic!("save called on FsStorage");
    }

    fn load(&mut self, _key: &str) -> io::Result<Self::Stream> {
        panic!("load called on FsStorage");
    }
}
