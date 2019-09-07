// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Emulator Screen Shot Taker",
    about = "Runs emulator for specified amount of time and takes screen shot"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "ticks")]
    ticks: u64,
    #[structopt(long = "output", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let mut rom_file = File::open(&options.rom)?;
    let mut rom: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom)?;

    game_boy_emulator::run_until_and_take_screenshot(&rom, options.ticks, options.output);
    Ok(())
}
