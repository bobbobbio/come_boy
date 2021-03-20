// Copyright 2017 Remi Bernotavicius

use backend::BackendMap;
use come_boy::game_boy_emulator::{self, GamePak};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

mod backend;

#[derive(Debug)]
enum Error {
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

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt)]
#[structopt(name = "Come Boy", about = "Game Boy (DMG) emulator")]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,

    #[structopt(long = "scale", default_value = "4")]
    scale: u32,

    #[structopt(long = "renderer", default_value = "default")]
    renderer: String,

    #[structopt(long = "save-state", parse(from_os_str))]
    save_state: Option<PathBuf>,
}

fn read_save_state(path: PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(options.rom)?;
    let save_state = options.save_state.map(read_save_state).transpose()?;

    let backend_map = BackendMap::new();
    let backend = backend_map.get(&options.renderer)?;
    backend.run(game_pak, save_state, options.scale)
}
