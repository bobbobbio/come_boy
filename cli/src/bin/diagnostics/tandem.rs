// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::storage::fs::Fs;
use std::path::PathBuf;

#[derive(clap::Args)]
#[command(about = "Runs emulator in tandem with another source")]
pub struct Options {
    emulator_trace: PathBuf,
    rom: PathBuf,
    #[arg(long = "pc-only")]
    pc_only: bool,
}

pub fn main(options: Options) -> Result<()> {
    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;
    game_boy_emulator::run_in_tandem_with(
        &mut std::io::stdout().lock(),
        fs,
        options.emulator_trace.to_str().unwrap(),
        game_pak,
        options.pc_only,
    )
}
