// Copyright 2017 Remi Bernotavicius

use bin_common::backend::BackendMap;
use bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::SoundStream;
use come_boy::storage::fs::Fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

#[allow(unused)]
#[derive(StructOpt)]
#[structopt(name = "Come Boy Debugger", about = "Game Boy (DMG) emulator debugger")]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,

    #[structopt(long = "scale", default_value = "4")]
    scale: u32,

    #[structopt(long = "renderer", default_value = "default")]
    renderer: String,
}

struct Frontend {
    fs: Fs,
    game_pak: GamePak<Fs>,
}

impl Frontend {
    fn new(fs: Fs, game_pak: GamePak<Fs>) -> Self {
        Self { fs, game_pak }
    }
}

impl bin_common::frontend::Frontend for Frontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::run_debugger(renderer, sound_stream, self.fs, self.game_pak, &|| {
            INTERRUPTED.swap(false, Ordering::Relaxed)
        });
    }
}

fn set_up_signal_handler() {
    ctrlc::set_handler(move || INTERRUPTED.store(true, Ordering::Relaxed)).unwrap();
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();
    set_up_signal_handler();

    let options = Options::from_args();

    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;

    let rendering_options = RenderingOptions {
        scale: options.scale,
        window_title: "come boy (in debugger)".into(),
        ..Default::default()
    };

    let backend_map = BackendMap::new(rendering_options, Frontend::new(fs, game_pak));
    backend_map.run(&options.renderer)?;
    Ok(())
}
