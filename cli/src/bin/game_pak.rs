// Copyright 2019 Remi Bernotavicius

use come_boy::game_boy_emulator::{GamePak, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "GamePak Info", about = "Prints information about GamePaks")]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path_without_sav(options.rom)?;
    println!("{:?}", game_pak);

    Ok(())
}
