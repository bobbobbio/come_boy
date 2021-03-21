// Copyright 2019 Remi Bernotavicius

use bin_common::{backend::BackendMap, Result};
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use std::path::PathBuf;
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

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
        #[structopt(long = "renderer", default_value = "default")]
        renderer: String,
    },
    Playback {
        #[structopt(parse(from_os_str))]
        rom: PathBuf,
        #[structopt(long = "input", parse(from_os_str))]
        input: PathBuf,
        #[structopt(long = "scale", default_value = "4")]
        scale: u32,
        #[structopt(long = "renderer", default_value = "default")]
        renderer: String,
    },
    Print {
        #[structopt(long = "input", parse(from_os_str))]
        input: PathBuf,
    },
}

struct RecordFrontend {
    game_pak: GamePak,
    output: PathBuf,
}

impl RecordFrontend {
    fn new(game_pak: GamePak, output: PathBuf) -> Self {
        Self { game_pak, output }
    }
}

impl bin_common::frontend::Frontend for RecordFrontend {
    fn run<R: Renderer>(self, renderer: &mut R) {
        game_boy_emulator::run_and_record_replay(renderer, self.game_pak, &self.output).unwrap()
    }
}

struct PlaybackFrontend {
    game_pak: GamePak,
    input: PathBuf,
}

impl PlaybackFrontend {
    fn new(game_pak: GamePak, input: PathBuf) -> Self {
        Self { game_pak, input }
    }
}

impl bin_common::frontend::Frontend for PlaybackFrontend {
    fn run<R: Renderer>(self, renderer: &mut R) {
        game_boy_emulator::playback_replay(renderer, self.game_pak, &self.input).unwrap()
    }
}

fn main() -> Result<()> {
    let options = Options::from_args();
    match options {
        Options::Record {
            rom,
            output,
            scale,
            renderer,
        } => {
            let game_pak = GamePak::from_path(rom)?;
            let rendering_options = RenderingOptions {
                scale,
                ..Default::default()
            };
            let backend_map =
                BackendMap::new(rendering_options, RecordFrontend::new(game_pak, output));
            backend_map.run(&renderer)?;
            Ok(())
        }
        Options::Playback {
            rom,
            input,
            scale,
            renderer,
        } => {
            let game_pak = GamePak::from_path(rom)?;
            let rendering_options = RenderingOptions {
                scale,
                ..Default::default()
            };
            let backend_map =
                BackendMap::new(rendering_options, PlaybackFrontend::new(game_pak, input));
            backend_map.run(&renderer)?;
            Ok(())
        }
        Options::Print { input } => {
            game_boy_emulator::print_replay(&input)?;
            Ok(())
        }
    }
}
