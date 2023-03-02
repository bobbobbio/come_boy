// Copyright 2022 Remi Bernotavicius

use crate::bin_common::{backend::BackendMap, Result};
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::SoundStream;
use come_boy::storage::fs::Fs;
use std::fs::File;
use std::io::{self, Read as _};
use std::path::PathBuf;

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
        game_boy_emulator::run_with_coverage(
            self.fs,
            renderer,
            sound_stream,
            self.game_pak,
            &self.output,
        )
        .unwrap()
    }
}

#[derive(clap::Args)]
#[command(
    about = "Runs emulator and records coverage information",
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
    Display {
        rom: PathBuf,
        #[arg(long = "input")]
        input: PathBuf,
        #[arg(
            long = "visited-threshold",
            help = "Only show lines which were visited more than the given percentage of the time"
        )]
        visited_threshold: Option<f64>,
    },
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
            let rom_key = Fs::path_to_key(&rom)?;
            let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;
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
        Subcommand::Display {
            rom,
            input,
            visited_threshold,
        } => {
            let mut rom_file = File::open(rom)?;
            let mut rom: Vec<u8> = vec![];
            rom_file.read_to_end(&mut rom)?;

            let input_file = File::open(input)?;

            game_boy_emulator::display_coverage(
                &rom,
                &input_file,
                visited_threshold,
                io::stdout(),
            )?;
            Ok(())
        }
    }
}
