// Copyright 2017 Remi Bernotavicius

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use bin_common::backend::BackendMap;
use bin_common::{read_save_state, Result};
use clap::Parser as _;
use come_boy::game_boy_emulator::{
    self,
    perf::{PerfObserver, PerfStats},
    GamePak, NullPerfObserver,
};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::{NullSoundStream, SoundStream};
use come_boy::storage::fs::Fs;
use std::path::PathBuf;

#[path = "../bin_common/mod.rs"]
mod bin_common;

struct Frontend {
    fs: Fs,
    disable_sound: bool,
    disable_joypad: bool,
    unlock_cpu: bool,
    perf_stats: bool,
    game_pak: GamePak<Fs>,
    save_state: Option<Vec<u8>>,
    run_until: Option<u64>,
}

impl Frontend {
    #[allow(clippy::too_many_arguments)]
    fn new(
        fs: Fs,
        disable_sound: bool,
        disable_joypad: bool,
        unlock_cpu: bool,
        perf_stats: bool,
        game_pak: GamePak<Fs>,
        save_state: Option<Vec<u8>>,
        run_until: Option<u64>,
    ) -> Self {
        Self {
            fs,
            disable_sound,
            disable_joypad,
            unlock_cpu,
            perf_stats,
            game_pak,
            save_state,
            run_until,
        }
    }

    fn run_with_observer(
        self,
        renderer: &mut impl Renderer,
        sound_stream: &mut impl SoundStream,
        observer: &mut impl PerfObserver,
    ) {
        game_boy_emulator::run_emulator(
            renderer,
            sound_stream,
            self.fs,
            self.game_pak,
            self.save_state,
            self.unlock_cpu,
            observer,
            self.disable_joypad,
            self.run_until,
        )
        .unwrap();
    }

    fn run_with_sound(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        if self.perf_stats {
            let mut perf_stats = PerfStats::new();
            self.run_with_observer(renderer, sound_stream, &mut perf_stats);
            log::info!("{perf_stats:#}");
        } else {
            self.run_with_observer(renderer, sound_stream, &mut NullPerfObserver);
        }
    }
}

impl bin_common::frontend::Frontend for Frontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        if self.disable_sound {
            self.run_with_sound(renderer, &mut NullSoundStream);
        } else {
            self.run_with_sound(renderer, sound_stream);
        }
    }
}

#[derive(clap::Parser)]
#[command(about = "Game Boy (DMG) emulator")]
struct Options {
    rom: PathBuf,

    #[arg(long = "scale", default_value = "4")]
    scale: u32,

    #[arg(long = "renderer", default_value = "default")]
    renderer: String,

    #[arg(long = "save-state")]
    save_state: Option<PathBuf>,

    #[arg(long = "disable-sound")]
    disable_sound: bool,

    #[arg(long = "disable-joypad")]
    disable_joypad: bool,

    #[arg(long = "unlock-cpu")]
    unlock_cpu: bool,

    #[arg(long = "perf-stats")]
    perf_stats: bool,

    #[arg(long = "run-until")]
    run_until: Option<u64>,

    #[arg(long = "log-level", default_value = "info")]
    log_level: log::LevelFilter,
}

fn main() -> Result<()> {
    let options = Options::parse();
    simple_logger::SimpleLogger::new()
        .with_level(options.log_level)
        .init()
        .unwrap();

    let mut fs = Fs::new(options.rom.parent());
    let rom_key = Fs::path_to_key(&options.rom)?;
    let game_pak = GamePak::from_storage(&mut fs, &rom_key)?;
    let save_state = options.save_state.map(read_save_state).transpose()?;

    let rendering_options = RenderingOptions {
        scale: options.scale,
        ..Default::default()
    };

    let front_end = Frontend::new(
        fs,
        options.disable_sound,
        options.disable_joypad,
        options.unlock_cpu,
        options.perf_stats,
        game_pak,
        save_state,
        options.run_until,
    );
    let backend_map = BackendMap::new(rendering_options, front_end);
    backend_map.run(&options.renderer)?;
    Ok(())
}
