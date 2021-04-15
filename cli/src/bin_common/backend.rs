// Copyright 2021 Remi Bernotavicius

use super::frontend::Frontend;
use come_boy::rendering::RenderingOptions;
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
    fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> !;
}

pub struct BackendMap<F> {
    map: BTreeMap<String, Box<dyn FnOnce(Self)>>,
    frontend: F,
    rendering_options: RenderingOptions,
}

impl<F: Frontend> BackendMap<F> {
    pub fn new(rendering_options: RenderingOptions, frontend: F) -> Self {
        let mut map = Self {
            map: BTreeMap::new(),
            frontend,
            rendering_options,
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
        backend.run(self.rendering_options, self.frontend)
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
    use come_boy::rendering::{speedy, RenderingOptions};
    use come_boy::sound::cpal::CpalSoundStream;

    pub(super) struct Speedy2dBackend;

    impl Backend for Speedy2dBackend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            println!("Using speedy2d renderer");
            let mut sound_stream = CpalSoundStream::new();
            speedy::run_loop(rendering_options, move |renderer| {
                frontend.run(renderer, &mut sound_stream);
            })
        }
    }
}

#[cfg(feature = "sdl2")]
mod sdl2 {
    use super::{Backend, Frontend};
    use come_boy::rendering::{sdl2::Sdl2WindowRenderer, RenderingOptions};
    use come_boy::sound::cpal::CpalSoundStream;

    pub(super) struct Sdl2Backend;

    impl Backend for Sdl2Backend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            println!("Using sdl2 renderer");
            let mut sound_stream = CpalSoundStream::new();
            let mut renderer = Sdl2WindowRenderer::new(rendering_options);
            frontend.run(&mut renderer, &mut sound_stream);
            std::process::exit(0)
        }
    }
}
