// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Emulator Screenshot Taker",
    about = "Runs emulator for specified amount of time and takes screenshot"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "ticks")]
    ticks: u64,
    #[structopt(long = "output", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(&options.rom)?;
    game_boy_emulator::run_until_and_take_screenshot(game_pak, options.ticks, options.output);

    Ok(())
}
