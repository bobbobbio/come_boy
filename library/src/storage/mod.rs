// copyright 2021 Remi Bernotavicius

use std::io;

pub mod fs;

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

impl<T: PersistentStorage> PersistentStorage for &mut T {
    type Stream = T::Stream;

    fn save(&mut self, key: &str) -> io::Result<Self::Stream> {
        (*self).save(key)
    }

    fn load(&mut self, key: &str) -> io::Result<Self::Stream> {
        (*self).load(key)
    }
}
