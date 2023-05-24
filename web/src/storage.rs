// 2022 copyright Remi Bernotavicius

use base64::Engine as _;
use come_boy::storage::OpenMode;
use std::cmp::min;
use std::io::{self, SeekFrom};

pub struct WebStorageFile {
    key: String,
    buffer: Vec<u8>,
    position: usize,
    storage: web_sys::Storage,
}

fn load_bytes_from_storage(storage: &web_sys::Storage, key: &str) -> Option<Vec<u8>> {
    storage
        .get_item(key)
        .ok()
        .flatten()
        .and_then(|s| base64::engine::general_purpose::STANDARD.decode(s).ok())
}

fn save_bytes_to_storage(storage: &web_sys::Storage, key: &str, bytes: &[u8]) {
    storage
        .set_item(
            key,
            &base64::engine::general_purpose::STANDARD.encode(bytes),
        )
        .ok();
}

impl WebStorageFile {
    fn new(storage: web_sys::Storage, key: &str) -> Self {
        Self {
            key: key.to_string(),
            buffer: load_bytes_from_storage(&storage, key).unwrap_or_default(),
            position: 0,
            storage,
        }
    }

    fn save(&self) {
        save_bytes_to_storage(&self.storage, &self.key, &self.buffer);
    }
}

impl io::Seek for WebStorageFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(p) => self.position = p.try_into().unwrap(),
            SeekFrom::End(delta) => {
                let end: i64 = self.buffer.len().try_into().unwrap();
                let new_pos = end + delta;
                if new_pos < 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid argument",
                    ));
                }
                self.position = new_pos.try_into().unwrap();
            }
            SeekFrom::Current(p) => {
                let mut new_pos: i64 = self.position.try_into().unwrap();
                new_pos += p;
                if new_pos < 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid argument",
                    ));
                }
                self.position = new_pos.try_into().unwrap();
            }
        }
        Ok(self.position.try_into().unwrap())
    }
}

impl io::Write for WebStorageFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let (start, end) = (self.position, self.position + buf.len());
        if end >= self.buffer.len() {
            self.buffer.resize(end, 0);
        }

        self.buffer[start..end].copy_from_slice(buf);
        self.position += buf.len();
        self.save();

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for WebStorageFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let to_copy = min(buf.len(), self.buffer.len() - self.position);
        let (start, end) = (self.position, self.position + to_copy);
        buf.copy_from_slice(&self.buffer[start..end]);
        self.position += to_copy;
        Ok(to_copy)
    }
}

impl come_boy::storage::StorageFile for WebStorageFile {
    fn set_len(&mut self, len: u64) -> io::Result<()> {
        self.buffer.resize(len.try_into().unwrap(), 0);
        Ok(())
    }
}

pub struct WebStorage(web_sys::Storage);

impl WebStorage {
    pub fn new(storage: web_sys::Storage) -> Self {
        Self(storage)
    }
}

impl come_boy::storage::PersistentStorage for WebStorage {
    type File = WebStorageFile;

    fn open(&mut self, _mode: OpenMode, key: &str) -> io::Result<Self::File> {
        Ok(WebStorageFile::new(self.0.clone(), key))
    }
}
