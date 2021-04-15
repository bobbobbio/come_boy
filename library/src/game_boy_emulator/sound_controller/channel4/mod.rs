// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::memory_controller::{FlagMask, GameBoyFlags, GameBoyRegister};
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom)]
#[repr(u8)]
pub enum SoundLength {
    Length = 0b00111111,
}

impl FlagMask for SoundLength {
    fn read_mask() -> u8 {
        Self::Length as u8
    }

    fn write_mask() -> u8 {
        Self::Length as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom)]
#[repr(u8)]
pub enum Counter {
    Initial = 0b10000000,
    Selection = 0b01000000,
}

impl FlagMask for Counter {
    fn read_mask() -> u8 {
        Self::Selection as u8
    }

    fn write_mask() -> u8 {
        Self::Initial as u8 | Self::Selection as u8
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Channel4 {
    pub sound_length: GameBoyFlags<SoundLength>,
    pub volume_envelope: GameBoyRegister,
    pub polynomial_counter: GameBoyRegister,
    pub counter: GameBoyFlags<Counter>,
}
