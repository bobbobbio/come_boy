// Copyright 2017 Remi Bernotavicius

use bin_common::backend::BackendMap;
use bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

struct Frontend {
    game_pak: GamePak,
    save_state: Option<Vec<u8>>,
}

impl Frontend {
    fn new(game_pak: GamePak, save_state: Option<Vec<u8>>) -> Self {
        Self {
            game_pak,
            save_state,
        }
    }
}

impl bin_common::frontend::Frontend for Frontend {
    fn run<R: Renderer>(self, renderer: &mut R) {
        game_boy_emulator::run_emulator(renderer, self.game_pak, self.save_state).unwrap();
    }
}

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

    let rendering_options = RenderingOptions {
        scale: options.scale,
        ..Default::default()
    };

    let backend_map = BackendMap::new(rendering_options, Frontend::new(game_pak, save_state));
    backend_map.run(&options.renderer)?;
    Ok(())
}
