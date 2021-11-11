// copyright 2021 Remi Bernotavicius

#![no_std]

#[cfg(feature = "panic-halt")]
use panic_halt as _;

#[no_mangle]
pub extern "C" fn init() {}

#[no_mangle]
pub extern "C" fn update(_tick: u32) {}

#[no_mangle]
pub extern "C" fn draw(_tick: u32) {}
