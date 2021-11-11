// copyright 2021 Remi Bernotavicius

#![no_std]
#![feature(default_alloc_error_handler)]

mod picosystem;

#[cfg(not(test))]
use panic_halt as _;

#[no_mangle]
pub extern "C" fn init() {}

#[no_mangle]
pub extern "C" fn update(_tick: u32) {}

#[no_mangle]
pub extern "C" fn draw(_tick: u32) {}
