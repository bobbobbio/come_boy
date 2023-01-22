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
            Self::UnknownRenderer(m) => write!(f, "unknown renderer {m}"),
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

        #[cfg(feature = "speedy2d-renderer")]
        map.add("speedy2d", |m| {
            m.run_backend(self::speedy2d::Speedy2dBackend)
        });

        #[cfg(feature = "sdl2-renderer")]
        map.add("sdl2", |m| m.run_backend(self::sdl2::Sdl2Backend));

        #[cfg(feature = "eframe-renderer")]
        map.add("eframe", |m| m.run_backend(self::eframe::EframeBackend));

        map.add("null", |m| m.run_backend(self::null::NullBackend));

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
                .find(|n| &n[..] != "null")
                .ok_or(Error::MissingRenderer)?
                .clone();
        }

        let f = self
            .map
            .remove(&renderer_name)
            .ok_or(Error::UnknownRenderer(renderer_name))?;
        f(self);
        Ok(())
    }
}

#[cfg(feature = "speedy2d-renderer")]
mod speedy2d {
    use super::{Backend, Frontend};
    use come_boy::rendering::{speedy, RenderingOptions};
    use come_boy::sound::cpal::CpalSoundStream;

    pub(super) struct Speedy2dBackend;

    impl Backend for Speedy2dBackend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            log::info!("Using speedy2d renderer");
            speedy::run_loop(rendering_options, move |renderer| {
                let mut sound_stream = CpalSoundStream::new();
                frontend.run(renderer, &mut sound_stream);
                log::info!("Exiting...");
            })
        }
    }
}

#[cfg(feature = "sdl2-renderer")]
mod sdl2 {
    use super::{Backend, Frontend};
    use come_boy::rendering::{sdl2::Sdl2WindowRenderer, RenderingOptions};
    use come_boy::sound::cpal::CpalSoundStream;

    pub(super) struct Sdl2Backend;

    impl Backend for Sdl2Backend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            log::info!("Using sdl2 renderer");
            let mut sound_stream = CpalSoundStream::new();
            let mut renderer = Sdl2WindowRenderer::new(rendering_options);
            frontend.run(&mut renderer, &mut sound_stream);
            log::info!("Exiting...");
            std::process::exit(0)
        }
    }
}

#[cfg(feature = "eframe-renderer")]
mod eframe {
    use super::{Backend, Frontend};
    use come_boy::rendering::{glow, RenderingOptions};
    use come_boy::sound::cpal::CpalSoundStream;
    use std::sync::Arc;

    struct App {
        paint_callback: Arc<egui_glow::CallbackFn>,
        window_vec: egui::Vec2,
    }

    impl App {
        fn new(front: glow::GlowFrontRenderer, window_vec: egui::Vec2) -> Self {
            let paint_cb = move |_: egui::PaintCallbackInfo,
                                 painter: &egui_glow::painter::Painter| {
                front.render(painter.gl());
            };

            Self {
                paint_callback: Arc::new(egui_glow::CallbackFn::new(paint_cb)),
                window_vec,
            }
        }

        fn render_game_screen(&mut self, ui: &mut egui::Ui) {
            let (rect, _) = ui.allocate_exact_size(self.window_vec.clone(), egui::Sense::drag());

            let callback = egui::PaintCallback {
                rect,
                callback: self.paint_callback.clone(),
            };
            ui.painter().add(callback);
        }
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default()
                .frame(egui::Frame::canvas(&Default::default()))
                .show(ctx, |ui| {
                    self.render_game_screen(ui);
                });
        }
    }

    pub(super) struct EframeBackend;

    impl Backend for EframeBackend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            log::info!("Using eframe renderer");

            let width = rendering_options.width;
            let height = rendering_options.height;
            let pixel_size = rendering_options.scale;
            let window_vec =
                egui::Vec2::new((width * pixel_size) as f32, (height * pixel_size) as f32);

            let native_options = eframe::NativeOptions {
                initial_window_size: Some(window_vec.clone()),
                max_window_size: Some(window_vec.clone()),
                ..Default::default()
            };

            std::thread::scope(move |scope| {
                let (sender, receiver) = std::sync::mpsc::channel();
                scope.spawn(move || {
                    let mut back_renderer = receiver.recv().unwrap();
                    let mut sound_stream = CpalSoundStream::new();
                    frontend.run(&mut back_renderer, &mut sound_stream);
                });

                eframe::run_native(
                    "come_boy",
                    native_options,
                    Box::new(move |cc| {
                        let gl = cc.gl.as_ref().unwrap();
                        let (front_renderer, back_renderer) = glow::render_pair(gl);
                        sender.send(back_renderer).unwrap();
                        Box::new(App::new(front_renderer, window_vec))
                    }),
                );
            });
            log::info!("Exiting...");
            std::process::exit(0)
        }
    }
}

mod null {
    use super::{Backend, Frontend};
    use come_boy::rendering::{Color, Event, Renderer, RenderingOptions};
    use come_boy::sound::NullSoundStream;
    use std::io;
    use std::sync::mpsc::{channel, Receiver};

    pub struct NullRendererWithEvents(Receiver<Event>);

    impl Renderer for NullRendererWithEvents {
        fn poll_events(&mut self) -> Vec<Event> {
            self.0.try_recv().into_iter().collect()
        }

        fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
            Ok(())
        }

        fn color_pixel(&mut self, _: i32, _: i32, _: Color) {}
        fn present(&mut self) {}
    }

    pub(super) struct NullBackend;

    impl Backend for NullBackend {
        fn run<F: Frontend>(&self, rendering_options: RenderingOptions, frontend: F) -> ! {
            log::info!("Using null renderer");

            let (sender, recv) = channel();
            if rendering_options.stop_on_ctrl_c {
                ctrlc::set_handler(move || drop(sender.send(Event::Quit))).unwrap();
            }

            let mut renderer = NullRendererWithEvents(recv);
            frontend.run(&mut renderer, &mut NullSoundStream);

            log::info!("Exiting...");
            std::process::exit(0)
        }
    }
}
