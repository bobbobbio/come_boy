// Copyright 2021 Remi Bernotavicius

#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod emulator_common;
pub mod game_boy_emulator;
pub mod intel_8080_emulator;
pub mod lr35902_emulator;
mod no_std;
pub mod rendering;
pub mod sound;
pub mod storage;
mod util;

#[cfg(feature = "std")]
pub use {
    bincode as codec,
    std::{io, thread::sleep, time::Instant},
};

#[cfg(not(feature = "std"))]
pub use no_std::*;
