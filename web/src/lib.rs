// copyright 2021 Remi Bernotavicius
use come_boy::game_boy_emulator::Palette;
use come_boy::rendering::{
    egui::{render_pair, HEIGHT, PIXEL_SIZE, WIDTH},
    Color,
};
use egui::widgets::color_picker::color_edit_button_rgb;
use egui::widgets::Hyperlink;
use emulator::Emulator;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod emulator;
mod storage;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[derive(Clone)]
struct EmulatorRef(Rc<RefCell<Emulator>>);

impl EmulatorRef {
    fn new(emulator: Emulator) -> Self {
        Self(Rc::new(RefCell::new(emulator)))
    }

    fn borrow_mut(&self) -> BorrowedEmulator<'_> {
        BorrowedEmulator {
            ref_mut: self.0.borrow_mut(),
            rc: self.clone(),
        }
    }
}

struct BorrowedEmulator<'a> {
    ref_mut: std::cell::RefMut<'a, Emulator>,
    rc: EmulatorRef,
}

impl Deref for BorrowedEmulator<'_> {
    type Target = Emulator;

    fn deref(&self) -> &Emulator {
        &*self.ref_mut
    }
}

impl DerefMut for BorrowedEmulator<'_> {
    fn deref_mut(&mut self) -> &mut Emulator {
        &mut *self.ref_mut
    }
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

        let (front, back) = render_pair(cc.egui_ctx.clone(), gl);
        let emulator = EmulatorRef::new(Emulator::new(back));
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
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("performance appears to be available")
}

const GITHUB_URL: &str = "https://github.com/bobbobbio/come_boy";

fn as_rgb(
    palette: &mut Palette,
    body: impl FnOnce(&mut [f32; 3], &mut [f32; 3], &mut [f32; 3], &mut [f32; 3]),
) {
    fn to_floats(color: &Color) -> [f32; 3] {
        [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        ]
    }
    fn from_floats(floats: [f32; 3]) -> Color {
        Color::new(
            (floats[0] * 255.0) as u8,
            (floats[1] * 255.0) as u8,
            (floats[2] * 255.0) as u8,
        )
    }

    let mut shade0 = to_floats(&palette.shade0);
    let mut shade1 = to_floats(&palette.shade1);
    let mut shade2 = to_floats(&palette.shade2);
    let mut shade3 = to_floats(&palette.shade3);
    body(&mut shade0, &mut shade1, &mut shade2, &mut shade3);
    palette.shade0 = from_floats(shade0);
    palette.shade1 = from_floats(shade1);
    palette.shade2 = from_floats(shade2);
    palette.shade3 = from_floats(shade3);
}

trait EmulatorUiHandler {
    fn load_rom_from_dialog(&mut self);
    fn loaded_rom(&mut self) -> Option<&str>;
    fn palette_mut(&mut self) -> &mut Palette;
    fn meta(&mut self, name: &str) -> String;
}

impl EmulatorUiHandler for BorrowedEmulator<'_> {
    fn load_rom_from_dialog(&mut self) {
        let emulator = self.rc.clone();
        let future = async move {
            if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                let rom_data = file.read().await;
                emulator.borrow_mut().load_rom(&rom_data);
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }

    fn loaded_rom(&mut self) -> Option<&str> {
        self.ref_mut.loaded_rom()
    }

    fn palette_mut(&mut self) -> &mut Palette {
        self.ref_mut.palette_mut()
    }

    fn meta(&mut self, name: &str) -> String {
        let document = window().document().unwrap();
        let head = document.head().unwrap();
        head.query_selector(&format!("[name={name}][content]"))
            .unwrap()
            .unwrap()
            .get_attribute("content")
            .unwrap()
    }
}

fn render_main_gui(ui: &mut egui::Ui, emulator: &mut impl EmulatorUiHandler) {
    egui::TopBottomPanel::top("options").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Load ROM").clicked() {
                emulator.load_rom_from_dialog();
            }
            if let Some(loaded_rom) = emulator.loaded_rom() {
                ui.label(&format!("playing: {loaded_rom}"));
            }
        });
        ui.collapsing("pallete", |ui| {
            let palette = emulator.palette_mut();
            as_rgb(palette, |shade0, shade1, shade2, shade3| {
                ui.horizontal(|ui| {
                    ui.label("Shade 0: ");
                    color_edit_button_rgb(ui, shade0);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 1: ");
                    color_edit_button_rgb(ui, shade1);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 2: ");
                    color_edit_button_rgb(ui, shade2);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 3: ");
                    color_edit_button_rgb(ui, shade3);
                });
            });
        });
    });

    egui::TopBottomPanel::bottom("information").show_inside(ui, |ui| {
        ui.add(Hyperlink::from_label_and_url(
            "come_boy on github",
            GITHUB_URL,
        ));
        ui.horizontal(|ui| {
            let revision = emulator.meta("revision");
            ui.label("revision: ");
            ui.add(Hyperlink::from_label_and_url(
                &revision,
                format!("{GITHUB_URL}/commit/{revision}"),
            ));
        });
        ui.label(&format!("built at: {}", emulator.meta("build_date")));
    });
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("sidebar").show(ctx, |ui| {
            render_main_gui(ui, &mut self.emulator.borrow_mut())
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
        let runner = eframe::WebRunner::new();
        let web_options = eframe::WebOptions::default();
        runner
            .start(
                "canvas",
                web_options,
                Box::new(|cc| Box::new(MyEguiApp::new(cc))),
            )
            .await?;
    }

    Ok(())
}
