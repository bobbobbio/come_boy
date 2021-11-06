// Copyright 2021 Remi Bernotavicius

#![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate std;

mod emulator_common;
pub mod game_boy_emulator;
pub mod intel_8080_emulator;
pub mod lr35902_emulator;
pub mod rendering;
pub mod sound;
pub mod storage;
mod util;

use ::bincode as codec;
use ::byteorder as bytes;
use std::collections;
use std::io;
