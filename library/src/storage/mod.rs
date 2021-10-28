// copyright 2021 Remi Bernotavicius

use crate::io;

pub mod fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
}

pub trait StorageFile: io::Read + io::Write + io::Seek {
    fn set_len(&mut self, len: u64) -> io::Result<()>;
}

pub trait PersistentStorage {
    type File: StorageFile;

    fn open(&mut self, mode: OpenMode, key: &str) -> io::Result<Self::File>;
}

#[derive(Default)]
pub struct PanicStorage;

impl PersistentStorage for PanicStorage {
    type File = <fs::Fs as PersistentStorage>::File;

    fn open(&mut self, _mode: OpenMode, _key: &str) -> io::Result<Self::File> {
        panic!("open called on PanicStorage");
    }
}

#[derive(Default)]
pub struct ReadOnly<Fs>(Fs);

impl<Fs: PersistentStorage> PersistentStorage for ReadOnly<Fs> {
    type File = Fs::File;

    fn open(&mut self, mode: OpenMode, key: &str) -> io::Result<Self::File> {
        if mode == OpenMode::Write || mode == OpenMode::ReadWrite {
            panic!("tried to open for write on ReadOnly fs.");
        } else {
            self.0.open(mode, key)
        }
    }
}

impl<T: PersistentStorage> PersistentStorage for &mut T {
    type File = T::File;

    fn open(&mut self, mode: OpenMode, key: &str) -> io::Result<Self::File> {
        (*self).open(mode, key)
    }
}
