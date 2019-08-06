// Copyright 2017 Remi Bernotavicius

use come_boy::game_boy_emulator;
use std::fs::File;
use std::io::{Read, Result};
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

    let mut rom_file = File::open(&options.rom)?;
    let mut rom: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom)?;

    game_boy_emulator::run_emulator(&rom, options.scale);

    Ok(())
}
