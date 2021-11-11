// copyright 2021 Remi Bernotavicius

#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

mod allocator;
mod graphics;
mod picosystem;

use graphics::{Color, Graphics};

#[cfg(not(test))]
use panic_halt as _;

#[no_mangle]
pub extern "C" fn init() {}

#[no_mangle]
pub extern "C" fn update(_tick: u32) {}

#[no_mangle]
pub extern "C" fn draw(_tick: u32) {
    let graphics = Graphics::new();

    graphics.set_pen(Color::rgba(10, 12, 0, 15));
    graphics.clear();
    graphics.set_pen(Color::rgba(0, 0, 0, 4));

    graphics.hline(2, 12, 116);
    graphics.text("hello there");
}
