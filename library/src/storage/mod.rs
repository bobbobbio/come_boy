// copyright 2021 Remi Bernotavicius

use std::io;

pub trait PersistentStorage {
    type Stream: io::Read + io::Write;

    fn save(&mut self, key: &str) -> io::Result<Self::Stream>;
    fn load(&mut self, key: &str) -> io::Result<Self::Stream>;
}

pub struct PanicStorage;

impl PersistentStorage for PanicStorage {
    type Stream = io::Cursor<Vec<u8>>;

    fn save(&mut self, _key: &str) -> io::Result<Self::Stream> {
        panic!("save called on PanicStorage");
    }

    fn load(&mut self, _key: &str) -> io::Result<Self::Stream> {
        panic!("load called on PanicStorage");
    }
}

pub mod fs;
