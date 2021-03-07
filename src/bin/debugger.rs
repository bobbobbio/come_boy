// Copyright 2017 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use come_boy::rendering::sdl2::Sdl2WindowRenderer;
use nix::sys::signal;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

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
}

fn main() -> Result<()> {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );
    unsafe { signal::sigaction(signal::SIGINT, &sig_action) }.unwrap();

    let options = Options::from_args();

    let window_title = "come boy (in debugger)";
    let renderer = Sdl2WindowRenderer::new(options.scale, window_title, 160, 144);
    let game_pak = GamePak::from_path(&options.rom)?;
    game_boy_emulator::run_debugger(renderer, game_pak, &|| {
        INTERRUPTED.swap(false, Ordering::Relaxed)
    });
    Ok(())
}
