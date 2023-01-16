// copyright 2021 Remi Bernotavicius
use egui::widgets::Hyperlink;
use emulator::Emulator;
use renderer::{HEIGHT, PIXEL_SIZE, WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod emulator;
mod renderer;
mod storage;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn input() -> web_sys::HtmlInputElement {
    let document = window().document().unwrap();
    let input = document.get_element_by_id("input").unwrap();
    input
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap()
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

fn set_up_file_input(emulator: EmulatorRef) {
    let on_change = Closure::wrap(Box::new(move || {
        let file_list = input().files().unwrap();
        let file = file_list.get(0).unwrap();

        let file_reader = web_sys::FileReader::new().unwrap();
        file_reader.read_as_array_buffer(&file).unwrap();
        let emulator = emulator.clone();
        let on_load = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let file_reader: web_sys::FileReader = event.target().unwrap().dyn_into().unwrap();
            let rom = file_reader.result().unwrap();
            let rom = js_sys::Uint8Array::new(&rom);
            let mut rom_file = vec![0; rom.length() as usize];
            rom.copy_to(&mut rom_file);
            emulator.borrow_mut().load_rom(&rom_file);
        }) as Box<dyn FnMut(_)>);
        file_reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();
    }) as Box<dyn FnMut()>);
    input()
        .add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref())
        .unwrap();
    on_change.forget();
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

struct MyEguiApp {
    paint_callback: Arc<egui_glow::CallbackFn>,
}

impl MyEguiApp {
    #[allow(dead_code)]
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl.as_ref().unwrap();

        let (front, back) = renderer::render_pair(gl);
        let emulator = EmulatorRef::new(Emulator::new(back));
        set_up_file_input(emulator.clone());
        set_up_input(emulator.clone());
        set_up_tick(emulator);

        let paint_cb = move |_: egui::PaintCallbackInfo, painter: &egui_glow::painter::Painter| {
            front.render(painter.gl());
        };

        Self {
            paint_callback: Arc::new(egui_glow::CallbackFn::new(paint_cb)),
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui) {
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
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("performance appears to be available")
}

/// This is roughly 1s / 60 (60fps)
const FRAME_DURATION: Duration = Duration::from_millis(17);

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_| {
            egui::SidePanel::right("options").show(ctx, |ui| {
                if ui.button("Load ROM").clicked() {
                    input().click();
                }
                ui.add(Hyperlink::from_label_and_url(
                    "come_boy on github",
                    "https://github.com/bobbobbio/come_boy",
                ));
                ui.label(&format!("revision: {}", meta("revision")));
                ui.label(&format!("built at: {}", meta("build_date")));
            });
            egui::SidePanel::left("screen").show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui);
                });
            });
        });

        ctx.request_repaint_after(FRAME_DURATION);
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
