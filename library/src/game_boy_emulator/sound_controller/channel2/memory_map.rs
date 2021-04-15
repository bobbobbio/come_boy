pub struct Channel2MemoryMapMut<'a> {
    pub sound_length: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub volume_envelope:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel2_memory_map_mut {
    ($ f : expr) => {
        Channel2MemoryMapMut {
            sound_length: &mut $f.sound_length,
            volume_envelope: &mut $f.volume_envelope,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel2MemoryMapMut<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65302u16 {
            self.sound_length.read_value(address - 65302u16)
        } else if address == 65303u16 {
            self.volume_envelope.read_value(address - 65303u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65302u16 {
            self.sound_length.set_value(address - 65302u16, value)
        } else if address == 65303u16 {
            self.volume_envelope.set_value(address - 65303u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
pub struct Channel2MemoryMap<'a> {
    pub sound_length: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub volume_envelope: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel2_memory_map {
    ($ f : expr) => {
        Channel2MemoryMap {
            sound_length: &$f.sound_length,
            volume_envelope: &$f.volume_envelope,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel2MemoryMap<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65302u16 {
            self.sound_length.read_value(address - 65302u16)
        } else if address == 65303u16 {
            self.volume_envelope.read_value(address - 65303u16)
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
