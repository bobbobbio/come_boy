// copyright 2021 Remi Bernotavicius

use crate::io;

#[cfg(feature = "std")]
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

pub struct PanicFile;

impl StorageFile for PanicFile {
    fn set_len(&mut self, _len: u64) -> io::Result<()> {
        unreachable!()
    }
}

impl io::Seek for PanicFile {
    fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
        unreachable!()
    }
}

impl io::Write for PanicFile {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unreachable!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unreachable!()
    }
}

impl io::Read for PanicFile {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unreachable!()
    }
}

#[derive(Default)]
pub struct PanicStorage;

impl PersistentStorage for PanicStorage {
    type File = PanicFile;

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
