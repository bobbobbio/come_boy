// Copyright 2017 Remi Bernotavicius

use bin_common::backend::BackendMap;
use bin_common::Result;
use come_boy::game_boy_emulator::{self, GamePak};
use come_boy::rendering::{Renderer, RenderingOptions};
use come_boy::sound::{NullSoundStream, SoundStream};
use come_boy::storage::fs::Fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[path = "../bin_common/mod.rs"]
mod bin_common;

struct Frontend {
    fs: Fs,
    disable_sound: bool,
    unlock_cpu: bool,
    game_pak: GamePak<Fs>,
    save_state: Option<Vec<u8>>,
}

impl Frontend {
    fn new(
        fs: Fs,
        disable_sound: bool,
        unlock_cpu: bool,
        game_pak: GamePak<Fs>,
        save_state: Option<Vec<u8>>,
    ) -> Self {
        Self {
            fs,
            disable_sound,
            unlock_cpu,
            game_pak,
            save_state,
        }
    }

    fn run_inner(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        game_boy_emulator::run_emulator(
            renderer,
            sound_stream,
            self.fs,
            self.game_pak,
            self.save_state,
            self.unlock_cpu,
        )
        .unwrap();
    }
}

impl bin_common::frontend::Frontend for Frontend {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream) {
        if self.disable_sound {
            self.run_inner(renderer, &mut NullSoundStream);
        } else {
            self.run_inner(renderer, sound_stream);
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

    #[structopt(long = "unlock-cpu")]
    unlock_cpu: bool,
}

fn read_save_state(path: PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let options = Options::from_args();

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
        options.unlock_cpu,
        game_pak,
        save_state,
    );
    let backend_map = BackendMap::new(rendering_options, front_end);
    backend_map.run(&options.renderer)?;
    Ok(())
}
