// copyright 2021 Remi Bernotavicius

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use allocator::ALLOCATOR;

mod allocator;
mod emulator;
mod graphics;
mod joypad;
mod picosystem;
mod time;

#[cfg(not(feature = "std"))]
mod panic;

#[cfg(not(feature = "std"))]
mod mutex;

#[cfg(feature = "std")]
use std::sync as mutex;

extern "C" fn core1_main() {
    emulator::run_from_core1();
}

#[no_mangle]
pub extern "C" fn init() {
    unsafe { graphics::init_from_core0() };
    unsafe { emulator::init_from_core0() };
    unsafe { picosystem::launch_core1(Some(core1_main)) };
}

#[no_mangle]
pub extern "C" fn update(_tick: u32) {
    // not much to do, other CPU does most the work
}

#[no_mangle]
pub extern "C" fn draw(_tick: u32) {
    unsafe {
        graphics::draw_from_core0();
    }
}
