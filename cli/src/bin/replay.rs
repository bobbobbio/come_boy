// Copyright 2019 Remi Bernotavicius

use bin_common::{backend::BackendMap, Result};
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::SoundStream;
use come_boy::storage::{fs::Fs, PanicStorage};
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
    fs: Fs,
    game_pak: GamePak<Fs>,
    output: PathBuf,
}

impl RecordFrontend {
    fn new(fs: Fs, game_pak: GamePak<Fs>, output: PathBuf) -> Self {
        Self {
            fs,
            game_pak,
            output,
        }
    }
}

impl bin_common::frontend::Frontend for RecordFrontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::run_and_record_replay(
            renderer,
            sound_stream,
            self.fs,
            self.game_pak,
            &self.output,
        )
        .unwrap()
    }
}

struct PlaybackFrontend {
    game_pak: GamePak<PanicStorage>,
    input: PathBuf,
}

impl PlaybackFrontend {
    fn new(game_pak: GamePak<PanicStorage>, input: PathBuf) -> Self {
        Self { game_pak, input }
    }
}

impl bin_common::frontend::Frontend for PlaybackFrontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::playback_replay(renderer, sound_stream, self.game_pak, &self.input)
            .unwrap()
    }
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let options = Options::from_args();
    match options {
        Options::Record {
            rom,
            output,
            scale,
            renderer,
        } => {
            let mut fs = Fs::new(rom.parent());
            let game_pak = GamePak::from_storage_without_sav(&mut fs, rom.to_str().unwrap())?;
            let rendering_options = RenderingOptions {
                scale,
                ..Default::default()
            };
            let backend_map =
                BackendMap::new(rendering_options, RecordFrontend::new(fs, game_pak, output));
            backend_map.run(&renderer)?;
            Ok(())
        }
        Options::Playback {
            rom,
            input,
            scale,
            renderer,
        } => {
            let rom_key = Fs::path_to_key(&rom)?;
            let game_pak = GamePak::from_storage_without_sav(&mut PanicStorage, &rom_key)?;
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
            print!("{}", game_boy_emulator::print_replay(&input)?);
            Ok(())
        }
    }
}
