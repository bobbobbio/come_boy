// Copyright 2021 Remi Bernotavicius

use come_boy::game_boy_emulator::GamePak;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownRenderer(String),
    MissingRenderer,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRenderer(m) => write!(f, "unknown renderer {}", m),
            Self::MissingRenderer => write!(f, "must be compiled with at least one renderer"),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub trait Backend {
    fn run(&self, game_pak: GamePak, save_state: Option<Vec<u8>>, scale: u32) -> !;
}

pub struct BackendMap(BTreeMap<String, Box<dyn Backend>>);

impl BackendMap {
    pub fn new() -> Self {
        let mut map = Self(BTreeMap::new());

        #[cfg(feature = "speedy2d")]
        map.add("speedy2d", self::speedy2d::Speedy2dBackend);

        #[cfg(feature = "sdl2")]
        map.add("sdl2", self::sdl2::Sdl2Backend);

        map
    }

    fn add<B: Backend + 'static>(&mut self, renderer_name: &str, backend: B) {
        assert_ne!(renderer_name, "default");
        self.0.insert(
            renderer_name.to_owned(),
            Box::new(backend) as Box<dyn Backend>,
        );
    }

    pub fn get(&self, renderer_name: &str) -> Result<&dyn Backend> {
        let mut renderer_name = renderer_name.to_owned();

        if renderer_name == "default" {
            renderer_name = self.0.keys().next().ok_or(Error::MissingRenderer)?.clone();
        }

        Ok(&**self
            .0
            .get(&renderer_name)
            .ok_or(Error::UnknownRenderer(renderer_name.into()))?)
    }
}

#[cfg(feature = "speedy2d")]
mod speedy2d {
    use super::Backend;
    use come_boy::game_boy_emulator::{self, GamePak};
    use come_boy::rendering::speedy;

    pub(super) struct Speedy2dBackend;

    impl Backend for Speedy2dBackend {
        fn run(&self, game_pak: GamePak, save_state: Option<Vec<u8>>, scale: u32) -> ! {
            println!("Using speed2d renderer");
            speedy::run_loop(scale, "come boy", 160, 144, move |renderer| {
                game_boy_emulator::run_emulator(renderer, game_pak, save_state).unwrap();
            })
        }
    }
}

#[cfg(feature = "sdl2")]
mod sdl2 {
    use super::Backend;
    use come_boy::game_boy_emulator::{self, GamePak};
    use come_boy::rendering::sdl2::Sdl2WindowRenderer;

    pub(super) struct Sdl2Backend;

    impl Backend for Sdl2Backend {
        fn run(&self, game_pak: GamePak, save_state: Option<Vec<u8>>, scale: u32) -> ! {
            println!("Using sdl2 renderer");
            let mut renderer = Sdl2WindowRenderer::new(scale, "come boy", 160, 144);
            game_boy_emulator::run_emulator(&mut renderer, game_pak, save_state).unwrap();
            std::process::exit(0)
        }
    }
}
