// Copyright 2018 Remi Bernotavicius

extern crate come_boy;
extern crate structopt;

use come_boy::game_boy_emulator;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Tandem Runner",
    about = "Runs emulator in tandem with another source"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    emulator_path: PathBuf,
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "pc-only")]
    pc_only: bool,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let mut rom_file = File::open(&options.rom)?;
    let mut rom: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom)?;

    game_boy_emulator::run_in_tandem_with(&options.emulator_path, &rom, options.pc_only)
}
