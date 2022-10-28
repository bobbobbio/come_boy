// Copyright 2017 Remi Bernotavicius

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use bin_common::backend::BackendMap;
use bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak, NullPerfObserver, PerfObserver};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::{NullSoundStream, SoundStream};
use come_boy::storage::fs::Fs;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

#[derive(Default)]
struct PerfStats {
    in_flight: HashMap<&'static str, Instant>,
    stats: HashMap<&'static str, (Duration, u32)>,
    num_ticks: u32,
}

impl PerfObserver for PerfStats {
    #[inline(always)]
    fn start_observation(&mut self, tag: &'static str) {
        let existing = self.in_flight.insert(tag, Instant::now()).is_some();
        assert!(!existing, "unfinished tag {}", tag);
    }

    #[inline(always)]
    fn end_observation(&mut self, tag: &'static str) {
        let start = self
            .in_flight
            .remove(tag)
            .unwrap_or_else(|| panic!("unexpected tag {}", tag));
        let entry = self.stats.entry(tag).or_insert((Duration::ZERO, 0));
        entry.0 += start.elapsed();
        entry.1 += 1;
    }

    #[inline(always)]
    fn tick_observed(&mut self) {
        self.num_ticks += 1;
    }
}

impl fmt::Display for PerfStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ticks = self.num_ticks;
        let mut sorted_stats: Vec<_> = self
            .stats
            .iter()
            .map(|(t, &(d, n))| {
                if ticks > n {
                    (t, d / n / (ticks / n))
                } else {
                    (t, d / n * (n / ticks))
                }
            })
            .collect();
        sorted_stats.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

        writeln!(f)?;
        for (tag, duration) in &sorted_stats {
            write!(
                f,
                "{tag:<30}: {:>10}ns amortized per tick",
                duration.as_nanos()
            )?;

            let &(dur, samples) = self.stats.get(*tag).unwrap();
            let avg = (dur / samples).as_nanos();
            writeln!(
                f,
                " ({avg:>10}ns average per call, {samples:>10} sample(s))",
            )?;
        }
        write!(f, "{} sample(s)", ticks)?;

        Ok(())
    }
}

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
            let mut perf_stats = PerfStats::default();
            self.run_with_observer(renderer, sound_stream, &mut perf_stats);
            log::info!("{}", &perf_stats);
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

#[derive(StructOpt)]
#[structopt(name = "Come Boy", about = "Game Boy (DMG) emulator")]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,

    #[structopt(long = "scale", default_value = "4")]
    scale: u32,

    #[structopt(long = "renderer", default_value = "default")]
    renderer: String,

    #[structopt(long = "save-state", parse(from_os_str))]
    save_state: Option<PathBuf>,

    #[structopt(long = "disable-sound")]
    disable_sound: bool,

    #[structopt(long = "disable-joypad")]
    disable_joypad: bool,

    #[structopt(long = "unlock-cpu")]
    unlock_cpu: bool,

    #[structopt(long = "perf-stats")]
    perf_stats: bool,

    #[structopt(long = "run-until")]
    run_until: Option<u64>,

    #[structopt(long = "log-level", default_value = "info")]
    log_level: log::LevelFilter,
}

fn read_save_state(path: PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let options = Options::from_args();
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
