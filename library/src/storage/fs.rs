// copyright 2021 Remi Bernotavicius

use super::{OpenMode, PersistentStorage, StorageFile};
use alloc::{borrow::ToOwned, string::String};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct Fs {
    working_dir: PathBuf,
}

impl Fs {
    pub fn new(working_dir: Option<&Path>) -> Self {
        Self {
            working_dir: working_dir.unwrap_or_else(|| Path::new("./")).to_owned(),
        }
    }

    pub fn path_to_key(path: &Path) -> io::Result<String> {
        let mut parent = path.parent().unwrap_or_else(|| Path::new("./"));
        if parent == Path::new("") {
            parent = Path::new("./");
        }
        let path_out = parent.canonicalize()?.join(path.file_name().unwrap());
        Ok(path_out.to_str().unwrap().to_owned())
    }
}

impl StorageFile for fs::File {
    fn set_len(&mut self, len: u64) -> io::Result<()> {
        fs::File::set_len(self, len)
    }
}

impl PersistentStorage for Fs {
    type File = fs::File;

    fn open(&mut self, mode: OpenMode, key: &str) -> io::Result<Self::File> {
        let mut path: PathBuf = key.into();
        if !path.is_absolute() {
            path = self.working_dir.join(key);
        }

        match mode {
            OpenMode::Read => fs::OpenOptions::new().read(true).open(path),
            OpenMode::Write => fs::OpenOptions::new()
                .write(true)
                .truncate(false)
                .create(true)
                .open(path),
            OpenMode::ReadWrite => fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(key),
        }
    }
}
