// Copyright 2021 Remi Bernotavicius

use super::{Channel, Frequency};
use crate::game_boy_emulator::memory_controller::{
    GameBoyRegister, MemoryAccessor, MemoryMappedHardware,
};
use serde_derive::{Deserialize, Serialize};

mod memory_map_mut;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel2 {
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    enabled: bool,
}

impl Channel for Channel2 {
    const FREQUENCY_ADDRESS: u16 = 0xFF18;

    fn restart(&mut self, _freq: &mut Frequency) {}

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl MemoryMappedHardware for Channel2 {
    fn read_value(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }
}
