// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::rendering::bitmap::BitmapRenderer;
use come_boy::storage::fs::Fs;
use std::path::PathBuf;

#[derive(clap::Args)]
#[command(about = "Runs emulator for specified amount of time and takes screenshot")]
pub struct Options {
    rom: PathBuf,
    #[arg(long = "ticks")]
    ticks: u64,
    #[arg(long = "replay")]
    replay: Option<PathBuf>,
    #[arg(long = "output")]
    output: PathBuf,
}

pub fn main(options: Options) -> Result<()> {
    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let replay_key = if let Some(replay) = &options.replay {
        Some(Fs::path_to_key(replay)?)
    } else {
        None
    };
    let output_key = Fs::path_to_key(&options.output)?;

    let renderer = BitmapRenderer::new(Default::default());
    let game_pak = GamePak::from_storage_without_sav(&mut fs, &rom_key)?;
    game_boy_emulator::run_until_and_take_screenshot(
        renderer,
        fs,
        game_pak,
        options.ticks,
        replay_key.as_deref(),
        &output_key,
    )?;

    Ok(())
}
