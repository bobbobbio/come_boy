// Copyright 2017 Remi Bernotavicius

use bin_common::backend::BackendMap;
use bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use nix::sys::signal;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigint(_: i32) {
    INTERRUPTED.store(true, Ordering::Relaxed)
}

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
    game_pak: GamePak,
}

impl Frontend {
    fn new(game_pak: GamePak) -> Self {
        Self { game_pak }
    }
}

impl bin_common::frontend::Frontend for Frontend {
    fn run<R: Renderer>(self, renderer: &mut R) {
        game_boy_emulator::run_debugger(renderer, self.game_pak, &|| {
            INTERRUPTED.swap(false, Ordering::Relaxed)
        });
    }
}

fn main() -> Result<()> {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );
    unsafe { signal::sigaction(signal::SIGINT, &sig_action) }.unwrap();

    let options = Options::from_args();

    let game_pak = GamePak::from_path(&options.rom)?;

    let rendering_options = RenderingOptions {
        scale: options.scale,
        window_title: "come boy (in debugger)".into(),
        ..Default::default()
    };

    let backend_map = BackendMap::new(rendering_options, Frontend::new(game_pak));
    backend_map.run(&options.renderer)?;
    Ok(())
}
