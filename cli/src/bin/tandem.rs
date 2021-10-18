// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::storage::fs::Fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Tandem Runner",
    about = "Runs emulator in tandem with another source"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    emulator_trace: PathBuf,
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "pc-only")]
    pc_only: bool,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;
    game_boy_emulator::run_in_tandem_with(
        fs,
        options.emulator_trace.to_str().unwrap(),
        game_pak,
        options.pc_only,
    )
}
