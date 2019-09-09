// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak};
use std::io;
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

fn main() -> io::Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(&options.rom)?;
    game_boy_emulator::run_in_tandem_with(&options.emulator_path, game_pak, options.pc_only)
}
