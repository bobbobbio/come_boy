// Copyright 2017 Remi Bernotavicius

mod emulator_common;
pub mod game_boy_emulator;
pub mod intel_8080_emulator;
pub mod lr35902_emulator;
pub mod rendering;
pub mod sound;
pub mod storage;
mod util;

use ::bincode as codec;
use std::io;
