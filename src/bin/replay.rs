// Copyright 2019 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::rendering::sdl2::Sdl2WindowRenderer;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Emulator Replay Recorder / Playback",
    about = "Record / Playback Emulator Gameplay.",
    rename_all = "kebab-case"
)]
enum Options {
    Record {
        #[structopt(parse(from_os_str))]
        rom: PathBuf,
        #[structopt(long = "output", parse(from_os_str))]
        output: PathBuf,
        #[structopt(long = "scale", default_value = "4")]
        scale: u32,
    },
    Playback {
        #[structopt(parse(from_os_str))]
        rom: PathBuf,
        #[structopt(long = "input", parse(from_os_str))]
        input: PathBuf,
        #[structopt(long = "scale", default_value = "4")]
        scale: u32,
    },
    Print {
        #[structopt(long = "input", parse(from_os_str))]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let options = Options::from_args();
    match options {
        Options::Record { rom, output, scale } => {
            let mut renderer = Sdl2WindowRenderer::new(scale, "come boy", 160, 144);
            let game_pak = GamePak::from_path(rom)?;
            game_boy_emulator::run_and_record_replay(&mut renderer, game_pak, &output)
        }
        Options::Playback { rom, input, scale } => {
            let mut renderer = Sdl2WindowRenderer::new(scale, "come boy", 160, 144);
            let game_pak = GamePak::from_path(rom)?;
            game_boy_emulator::playback_replay(&mut renderer, game_pak, &input)
        }
        Options::Print { input } => game_boy_emulator::print_replay(&input),
    }
}
