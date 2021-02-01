// Copyright 2017 Remi Bernotavicius

use come_boy::game_boy_emulator::{self, GamePak, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(feature = "sdl2")]
use come_boy::rendering::sdl2::Sdl2WindowRenderer;

#[cfg(feature = "speedy2d")]
use come_boy::rendering::speedy;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const DEFAULT_RENDERER: &'static str = "sdl2";

#[cfg(target_os = "windows")]
const DEFAULT_RENDERER: &'static str = "speedy2d";

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
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let game_pak = GamePak::from_path(options.rom)?;

    let mut renderer = options.renderer;
    if renderer == "default" {
        renderer = DEFAULT_RENDERER.to_owned();
    }

    let save_state = options.save_state.map(|path| {
        let mut file = File::open(path).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        contents
    });

    match &renderer[..] {
        #[cfg(feature = "speedy2d")]
        "speedy2d" => {
            speedy::run_loop(options.scale, "come boy", 160, 144, move |renderer| {
                game_boy_emulator::run_emulator(renderer, game_pak, save_state).unwrap();
            });
        }
        #[cfg(feature = "sdl2")]
        "sdl2" => {
            let mut renderer = Sdl2WindowRenderer::new(options.scale, "come boy", 160, 144);
            game_boy_emulator::run_emulator(&mut renderer, game_pak, save_state)?;
        }
        _ => unimplemented! {},
    }

    Ok(())
}
