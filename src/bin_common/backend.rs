// Copyright 2021 Remi Bernotavicius

use super::frontend::Frontend;
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
    fn run<F: Frontend>(&self, scale: u32, frontend: F) -> !;
}

pub struct BackendMap<F> {
    map: BTreeMap<String, Box<dyn FnOnce(Self)>>,
    frontend: F,
    scale: u32,
}

impl<F: Frontend> BackendMap<F> {
    pub fn new(scale: u32, frontend: F) -> Self {
        let mut map = Self {
            map: BTreeMap::new(),
            frontend,
            scale,
        };

        #[cfg(feature = "speedy2d")]
        map.add("speedy2d", |m| {
            m.run_backend(self::speedy2d::Speedy2dBackend)
        });

        #[cfg(feature = "sdl2")]
        map.add("sdl2", |m| m.run_backend(self::sdl2::Sdl2Backend));

        map
    }

    fn add<B: FnOnce(Self) + 'static>(&mut self, renderer_name: &str, func: B) {
        assert_ne!(renderer_name, "default");
        self.map.insert(
            renderer_name.to_owned(),
            Box::new(func) as Box<dyn FnOnce(Self)>,
        );
    }

    fn run_backend<B: Backend>(self, backend: B) {
        backend.run(self.scale, self.frontend)
    }

    pub fn run(mut self, renderer_name: &str) -> Result<()> {
        let mut renderer_name = renderer_name.to_owned();

        if renderer_name == "default" {
            renderer_name = self
                .map
                .keys()
                .next()
                .ok_or(Error::MissingRenderer)?
                .clone();
        }

        let f = self
            .map
            .remove(&renderer_name)
            .ok_or(Error::UnknownRenderer(renderer_name.into()))?;
        f(self);
        Ok(())
    }
}

#[cfg(feature = "speedy2d")]
mod speedy2d {
    use super::{Backend, Frontend};
    use come_boy::rendering::speedy;

    pub(super) struct Speedy2dBackend;

    impl Backend for Speedy2dBackend {
        fn run<F: Frontend>(&self, scale: u32, frontend: F) -> ! {
            println!("Using speed2d renderer");
            speedy::run_loop(scale, "come boy", 160, 144, move |renderer| {
                frontend.run(renderer);
            })
        }
    }
}

#[cfg(feature = "sdl2")]
mod sdl2 {
    use super::{Backend, Frontend};
    use come_boy::rendering::sdl2::Sdl2WindowRenderer;

    pub(super) struct Sdl2Backend;

    impl Backend for Sdl2Backend {
        fn run<F: Frontend>(&self, scale: u32, frontend: F) -> ! {
            println!("Using sdl2 renderer");
            let mut renderer = Sdl2WindowRenderer::new(scale, "come boy", 160, 144);
            frontend.run(&mut renderer);
            std::process::exit(0)
        }
    }
}
