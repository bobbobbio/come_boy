// Copyright 2021 Remi Bernotavicius

use self::memory_map::{Channel2MemoryMap, Channel2MemoryMapMut};
use super::{Channel, Frequency};
use crate::game_boy_emulator::memory_controller::{
    GameBoyRegister, MemoryAccessor, MemoryMappedHardware,
};
use serde_derive::{Deserialize, Serialize};

#[macro_use]
mod memory_map;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel2 {
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    enabled: bool,
}

impl Channel for Channel2 {
    const FREQUENCY_ADDRESS: u16 = 0xFF18;

    fn restart(&mut self, _freq: &mut Frequency) {}

    fn deliver_events(&mut self, _now: u64, _freq: &mut Frequency, _using_length: bool) {}

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl MemoryMappedHardware for Channel2 {
    fn read_value(&self, address: u16) -> u8 {
        let memory_map = channel2_memory_map!(self);
        memory_map.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let mut memory_map = channel2_memory_map_mut!(self);
        memory_map.set_memory(address, value);
    }
}
