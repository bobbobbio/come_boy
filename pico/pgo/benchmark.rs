// Copyright 2021 Remi Bernotavicius

#![feature(start)]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[link(name = "come_boy_pico", kind = "static")]
extern "C" {
    fn init();
    fn update(ticks: u32);
    fn draw(ticks: u32);
}

extern "C" {
    fn printf(fmt: *const i8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn pen(_r: u8, _g: u8, _b: u8) {}
#[no_mangle]
pub extern "C" fn blend_copy() {}
#[no_mangle]
pub extern "C" fn clear() {}
#[no_mangle]
pub extern "C" fn text(_c: *const i8, _x: i32, _y: i32) {}
#[no_mangle]
pub extern "C" fn wait_vsync() {}
#[no_mangle]
pub extern "C" fn flip() {}
#[no_mangle]
pub extern "C" fn button(_b: u32) -> bool {
    false
}

#[repr(C)]
pub struct Buffer {
    w: i32,
    h: i32,
    data: *const u16,
}

unsafe impl Sync for Buffer {}

static mut MAIN_BUFFER_DATA: [u16; 240 * 240] = [0; 240 * 240];

static MAIN_BUFFER: Buffer = Buffer {
    w: 240,
    h: 240,
    data: unsafe { MAIN_BUFFER_DATA.as_ptr() },
};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { printf("panic hit\n\0".as_ptr() as *const i8) };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
pub extern "C" fn target_buffer() -> *const Buffer {
    &MAIN_BUFFER
}

const NUM_ITERATIONS: u32 = 10_000;

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    unsafe {
        printf(
            "running %d iterations\n\0".as_ptr() as *const i8,
            NUM_ITERATIONS,
        )
    };

    unsafe { init() };
    for i in 0..NUM_ITERATIONS {
        unsafe { update(0) };
        unsafe { draw(0) };
        if i % 500 == 0 {
            unsafe { printf("%d/%d\n\0".as_ptr() as *const i8, i, NUM_ITERATIONS) };
        }
    }
    0
}
