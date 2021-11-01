// Copyright 2018 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::rendering::bitmap::BitmapRenderer;
use come_boy::storage::fs::Fs;
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
    #[structopt(long = "replay")]
    replay: Option<PathBuf>,
    #[structopt(long = "output", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let options = Options::from_args();

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
        replay_key.as_ref().map(|s| s.as_str()),
        &output_key,
    )?;

    Ok(())
}
