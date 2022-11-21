// Copyright 2019 Remi Bernotavicius

use crate::bin_common::{backend::BackendMap, Result};
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::SoundStream;
use come_boy::storage::fs::Fs;
use std::path::PathBuf;

#[derive(clap::Args)]
#[command(
    about = "Record / Playback Emulator Gameplay.",
    rename_all = "kebab-case"
)]
pub struct Options {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Record {
        rom: PathBuf,
        #[arg(long = "output")]
        output: PathBuf,
        #[arg(long = "scale", default_value = "4")]
        scale: u32,
        #[arg(long = "renderer", default_value = "default")]
        renderer: String,
    },
    Playback {
        rom: PathBuf,
        #[arg(long = "input")]
        input: PathBuf,
        #[arg(long = "scale", default_value = "4")]
        scale: u32,
        #[arg(long = "renderer", default_value = "default")]
        renderer: String,
    },
    Print {
        #[arg(long = "input")]
        input: PathBuf,
    },
}

struct RecordFrontend {
    fs: Fs,
    game_pak: GamePak<Fs>,
    output: String,
}

impl RecordFrontend {
    fn new(fs: Fs, game_pak: GamePak<Fs>, output: String) -> Self {
        Self {
            fs,
            game_pak,
            output,
        }
    }
}

impl crate::bin_common::frontend::Frontend for RecordFrontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::run_and_record_replay(
            self.fs,
            renderer,
            sound_stream,
            self.game_pak,
            &self.output,
        )
        .unwrap()
    }
}

struct PlaybackFrontend {
    game_pak: GamePak<Fs>,
    input: String,
    fs: Fs,
}

impl PlaybackFrontend {
    fn new(fs: Fs, game_pak: GamePak<Fs>, input: String) -> Self {
        Self {
            fs,
            game_pak,
            input,
        }
    }
}

impl crate::bin_common::frontend::Frontend for PlaybackFrontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::playback_replay(
            self.fs,
            renderer,
            sound_stream,
            self.game_pak,
            &self.input,
        )
        .unwrap()
    }
}

pub fn main(options: Options) -> Result<()> {
    match options.command {
        Subcommand::Record {
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
            let output_key = Fs::path_to_key(&output)?;
            let backend_map = BackendMap::new(
                rendering_options,
                RecordFrontend::new(fs, game_pak, output_key),
            );
            backend_map.run(&renderer)?;
            Ok(())
        }
        Subcommand::Playback {
            rom,
            input,
            scale,
            renderer,
        } => {
            let mut fs = Fs::new(rom.parent());
            let rom_key = Fs::path_to_key(&rom)?;
            let game_pak = GamePak::from_storage_without_sav(&mut fs, &rom_key)?;
            let rendering_options = RenderingOptions {
                scale,
                ..Default::default()
            };
            let input_key = Fs::path_to_key(&input)?;
            let backend_map = BackendMap::new(
                rendering_options,
                PlaybackFrontend::new(fs, game_pak, input_key),
            );
            backend_map.run(&renderer)?;
            Ok(())
        }
        Subcommand::Print { input } => {
            let file = std::fs::File::open(input)?;
            print!("{}", game_boy_emulator::print_replay(file)?);
            Ok(())
        }
    }
}
