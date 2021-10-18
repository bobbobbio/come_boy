// Copyright 2019 Remi Bernotavicius

use come_boy::game_boy_emulator::{GamePak, Result};
use come_boy::storage::fs::Fs;
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

    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage_without_sav(&mut fs, &rom_key)?;
    println!("{:?}", game_pak);

    Ok(())
}
