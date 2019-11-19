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
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(options.rom)?;
    game_boy_emulator::run_emulator(game_pak, options.scale);

    Ok(())
}
