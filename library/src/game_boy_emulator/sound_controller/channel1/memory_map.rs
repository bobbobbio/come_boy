pub struct Channel1MemoryMapMut<'a> {
    pub length_and_wave:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sweep: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub volume_envelope:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel1_memory_map_mut {
    ($ f : expr) => {
        Channel1MemoryMapMut {
            length_and_wave: &mut $f.length_and_wave,
            sweep: &mut $f.sweep,
            volume_envelope: &mut $f.volume_envelope,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel1MemoryMapMut<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65296u16 {
            self.sweep.read_value(address - 65296u16)
        } else if address == 65297u16 {
            self.length_and_wave.read_value(address - 65297u16)
        } else if address == 65298u16 {
            self.volume_envelope.read_value(address - 65298u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65296u16 {
            self.sweep.set_value(address - 65296u16, value)
        } else if address == 65297u16 {
            self.length_and_wave.set_value(address - 65297u16, value)
        } else if address == 65298u16 {
            self.volume_envelope.set_value(address - 65298u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
pub struct Channel1MemoryMap<'a> {
    pub length_and_wave: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sweep: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub volume_envelope: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel1_memory_map {
    ($ f : expr) => {
        Channel1MemoryMap {
            length_and_wave: &$f.length_and_wave,
            sweep: &$f.sweep,
            volume_envelope: &$f.volume_envelope,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel1MemoryMap<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65296u16 {
            self.sweep.read_value(address - 65296u16)
        } else if address == 65297u16 {
            self.length_and_wave.read_value(address - 65297u16)
        } else if address == 65298u16 {
            self.volume_envelope.read_value(address - 65298u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        panic!("Called set_memory on non-mutable MemoryMap")
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
