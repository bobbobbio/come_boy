// Copyright 2017 Remi Bernotavicius

use crate::bin_common::backend::BackendMap;
use crate::bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::SoundStream;
use come_boy::storage::fs::Fs;
use std::io::{self, BufRead as _};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

#[allow(unused)]
#[derive(clap::Args)]
#[command(about = "Game Boy (DMG) emulator debugger")]
pub struct Options {
    rom: PathBuf,

    #[arg(long = "scale", default_value = "4")]
    scale: u32,

    #[arg(long = "renderer", default_value = "default")]
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

impl crate::bin_common::frontend::Frontend for Frontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        let stdin = &mut io::stdin();
        let stdin_locked = &mut stdin.lock();
        let input = stdin_locked.lines();
        let output = &mut io::stdout();

        game_boy_emulator::run_debugger(
            renderer,
            sound_stream,
            self.fs,
            self.game_pak,
            input,
            output,
            &|| INTERRUPTED.swap(false, Ordering::Relaxed),
        );
    }
}

fn set_up_signal_handler() {
    ctrlc::set_handler(move || INTERRUPTED.store(true, Ordering::Relaxed)).unwrap();
}

pub fn main(options: Options) -> Result<()> {
    set_up_signal_handler();

    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;

    let rendering_options = RenderingOptions {
        scale: options.scale,
        window_title: "come boy (in debugger)".into(),
        stop_on_ctrl_c: false,
        ..Default::default()
    };

    let backend_map = BackendMap::new(rendering_options, Frontend::new(fs, game_pak));
    backend_map.run(&options.renderer)?;
    Ok(())
}
