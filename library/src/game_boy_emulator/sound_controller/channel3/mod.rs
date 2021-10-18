// Copyright 2021 Remi Bernotavicius

use super::{Channel, Frequency};
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryAccessor, MemoryChunk, MemoryMappedHardware,
};
use enum_iterator::IntoEnumIterator;
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};

mod memory_map_mut;

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom, IntoEnumIterator)]
#[repr(u8)]
pub enum EnabledFlag {
    Enabled = 0b10000000,
}

impl FlagMask for EnabledFlag {
    fn read_mask() -> u8 {
        Self::Enabled as u8
    }

    fn write_mask() -> u8 {
        Self::Enabled as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom, IntoEnumIterator)]
#[repr(u8)]
pub enum OutputLevel {
    Level = 0b01100000,
}

impl FlagMask for OutputLevel {
    fn read_mask() -> u8 {
        Self::Level as u8
    }

    fn write_mask() -> u8 {
        Self::Level as u8
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel3 {
    pub enabled: GameBoyFlags<EnabledFlag>,
    pub sound_length: GameBoyRegister,
    pub output_level: GameBoyFlags<OutputLevel>,
    pub wave_pattern: MemoryChunk,
}

impl Default for Channel3 {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            sound_length: Default::default(),
            output_level: Default::default(),
            wave_pattern: MemoryChunk::from_range(0..0x10),
        }
    }
}

impl Channel for Channel3 {
    const FREQUENCY_ADDRESS: u16 = 0xFF1D;

    fn restart(&mut self, _freq: &mut Frequency) {}

    fn deliver_events(&mut self, _now: u64, _freq: &mut Frequency, _using_length: bool) {}

    fn enabled(&self) -> bool {
        false
    }

    fn disable(&mut self) {}
}

impl MemoryMappedHardware for Channel3 {
    fn read_value(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }
}
