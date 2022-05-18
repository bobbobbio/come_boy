// copyright 2021 Remi Bernotavicius
use emulator::Emulator;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod emulator;
mod renderer;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    let document = window().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn input() -> web_sys::HtmlInputElement {
    let document = window().document().unwrap();
    let input = document.get_element_by_id("input").unwrap();
    input
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn set_up_rendering(emulator: Rc<RefCell<Emulator>>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        emulator.borrow_mut().render();

        // Schedule ourselves for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn set_up_file_input(emulator: Rc<RefCell<Emulator>>) {
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

fn set_up_input(emulator: Rc<RefCell<Emulator>>) {
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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Come Boy Starting");

    let canvas = canvas();
    canvas.set_width((renderer::WIDTH * renderer::PIXEL_SIZE) as u32);
    canvas.set_height((renderer::HEIGHT * renderer::PIXEL_SIZE) as u32);

    let emulator = Rc::new(RefCell::new(Emulator::new(&canvas)));
    set_up_rendering(emulator.clone());

    set_up_file_input(emulator.clone());

    set_up_input(emulator.clone());

    schedule(move || emulator.borrow_mut().tick(), 0);

    Ok(())
}
