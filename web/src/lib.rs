// copyright 2021 Remi Bernotavicius
use egui::widgets::Hyperlink;
use emulator::Emulator;
use renderer::{HEIGHT, PIXEL_SIZE, WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod emulator;
mod renderer;
mod storage;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn meta(name: &str) -> String {
    let document = window().document().unwrap();
    let head = document.head().unwrap();
    head.query_selector(&format!("[name={name}][content]"))
        .unwrap()
        .unwrap()
        .get_attribute("content")
        .unwrap()
}

#[derive(Clone)]
struct EmulatorRef(Rc<RefCell<Emulator>>);

impl EmulatorRef {
    fn new(emulator: Emulator) -> Self {
        Self(Rc::new(RefCell::new(emulator)))
    }

    fn borrow_mut(&self) -> std::cell::RefMut<'_, Emulator> {
        self.0.borrow_mut()
    }
}

fn set_up_input(emulator: EmulatorRef) {
    let window = window();

    let their_emulator = emulator.clone();
    let on_key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        their_emulator.borrow_mut().on_key_down(&event.code());
        event.prevent_default();
    }) as Box<dyn FnMut(_)>);

    let their_emulator = emulator;
    let on_key_up = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        their_emulator.borrow_mut().on_key_up(&event.code());
        event.prevent_default();
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref())
        .unwrap();
    window
        .add_event_listener_with_callback("keyup", on_key_up.as_ref().unchecked_ref())
        .unwrap();
    on_key_down.forget();
    on_key_up.forget();
}

fn request_timeout(f: &Closure<dyn FnMut()>, from_now: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), from_now)
        .expect("should register `setTimeout` OK");
}

fn schedule<F: FnMut() -> i32 + 'static>(mut body: F, from_now: i32) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let next = body();
        request_timeout(f.borrow().as_ref().unwrap(), next);
    }) as Box<dyn FnMut()>));
    request_timeout(g.borrow().as_ref().unwrap(), from_now);
}

fn set_up_tick(emulator: EmulatorRef) {
    schedule(move || emulator.borrow_mut().tick(), 0);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn set_up_rendering(ctx: egui::Context) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        ctx.request_repaint();

        // Schedule ourselves for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

struct MyEguiApp {
    emulator: EmulatorRef,
    paint_callback: Arc<egui_glow::CallbackFn>,
}

impl MyEguiApp {
    #[allow(dead_code)]
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl.as_ref().unwrap();

        let (front, back) = renderer::render_pair(gl);
        let emulator = EmulatorRef::new(Emulator::new(back));
        set_up_input(emulator.clone());
        set_up_tick(emulator.clone());
        set_up_rendering(cc.egui_ctx.clone());

        let paint_cb = move |_: egui::PaintCallbackInfo, painter: &egui_glow::painter::Painter| {
            front.render(painter.gl());
        };

        Self {
            emulator,
            paint_callback: Arc::new(egui_glow::CallbackFn::new(paint_cb)),
        }
    }

    fn render_game_screen(&mut self, ui: &mut egui::Ui) {
        let (rect, _) = ui.allocate_exact_size(
            egui::Vec2::new((WIDTH * PIXEL_SIZE) as f32, (HEIGHT * PIXEL_SIZE) as f32),
            egui::Sense::drag(),
        );

        let callback = egui::PaintCallback {
            rect,
            callback: self.paint_callback.clone(),
        };
        ui.painter().add(callback);
    }

    fn load_rom(&mut self) {
        let emulator = self.emulator.clone();
        let future = async move {
            if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                let rom_data = file.read().await;
                emulator.borrow_mut().load_rom(&rom_data);
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("performance appears to be available")
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("sidebar").show(ctx, |ui| {
            egui::TopBottomPanel::top("options").show_inside(ui, |ui| {
                if ui.button("Load ROM").clicked() {
                    self.load_rom();
                }
                if let Some(loaded_rom) = self.emulator.borrow_mut().loaded_rom() {
                    ui.label(&format!("playing: {loaded_rom}"));
                }
            });

            egui::TopBottomPanel::bottom("information").show_inside(ui, |ui| {
                ui.add(Hyperlink::from_label_and_url(
                    "come_boy on github",
                    "https://github.com/bobbobbio/come_boy",
                ));
                ui.label(&format!("revision: {}", meta("revision")));
                ui.label(&format!("built at: {}", meta("build_date")));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.render_game_screen(ui);
            });
        });
    }
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Come Boy Starting");

    #[cfg(target_arch = "wasm32")]
    {
        let web_options = eframe::WebOptions::default();
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|cc| Box::new(MyEguiApp::new(cc))),
        )
        .await?;
    }

    Ok(())
}
