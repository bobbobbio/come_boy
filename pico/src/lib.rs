// copyright 2021 Remi Bernotavicius

#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod allocator;
mod emulator;
mod graphics;
mod joypad;
mod picosystem;

#[cfg(not(feature = "std"))]
mod panic;

use core::cell::UnsafeCell;
use emulator::Emulator;

struct SingleThreadedGlobal<T> {
    payload: UnsafeCell<T>,
}

impl<T> SingleThreadedGlobal<T> {
    /// safety: Must only be used on single-threaded machine
    const unsafe fn new(payload: T) -> Self {
        Self {
            payload: UnsafeCell::new(payload),
        }
    }

    /// safety: Must only be one caller at a time
    unsafe fn get_mut(&self) -> &mut T {
        &mut *self.payload.get()
    }
}

unsafe impl<T> Sync for SingleThreadedGlobal<T> {}

static EMULATOR: SingleThreadedGlobal<Option<Emulator>> =
    unsafe { SingleThreadedGlobal::new(None) };

#[no_mangle]
pub extern "C" fn init() {
    *unsafe { EMULATOR.get_mut() } = Some(Emulator::new());
}

#[no_mangle]
pub extern "C" fn update(_tick: u32) {
    unsafe { EMULATOR.get_mut() }.as_mut().unwrap().update();
}

#[no_mangle]
pub extern "C" fn draw(_tick: u32) {
    unsafe { EMULATOR.get_mut() }.as_mut().unwrap().draw();
}
