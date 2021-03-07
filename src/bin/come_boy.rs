// Copyright 2017 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Come Boy", about = "Game Boy (DMG) emulator")]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "scale", default_value = "4")]
    scale: u32,
    #[structopt(long = "renderer", default_value = "sdl2")]
    renderer: String,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(options.rom)?;

    match &options.renderer[..] {
        #[cfg(feature = "speedy2d")]
        "speedy2d" => game_boy_emulator::run_emulator_with_speedy(game_pak, options.scale),
        #[cfg(feature = "sdl2")]
        "sdl2" => game_boy_emulator::run_emulator(game_pak, options.scale),
        _ => unimplemented! {},
    }

    Ok(())
}
