pub struct Channel3MemoryMapMut<'a> {
    pub enabled: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub output_level: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sound_length: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub wave_pattern: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel3_memory_map_mut {
    ($ f : expr) => {
        Channel3MemoryMapMut {
            enabled: &mut $f.enabled,
            output_level: &mut $f.output_level,
            sound_length: &mut $f.sound_length,
            wave_pattern: &mut $f.wave_pattern,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel3MemoryMapMut<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65306u16 {
            self.enabled.read_value(address - 65306u16)
        } else if address == 65307u16 {
            self.sound_length.read_value(address - 65307u16)
        } else if address == 65308u16 {
            self.output_level.read_value(address - 65308u16)
        } else if address >= 65328u16 && address < 65344u16 {
            self.wave_pattern.read_value(address - 65328u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65306u16 {
            self.enabled.set_value(address - 65306u16, value)
        } else if address == 65307u16 {
            self.sound_length.set_value(address - 65307u16, value)
        } else if address == 65308u16 {
            self.output_level.set_value(address - 65308u16, value)
        } else if address >= 65328u16 && address < 65344u16 {
            self.wave_pattern.set_value(address - 65328u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
pub struct Channel3MemoryMap<'a> {
    pub enabled: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub output_level: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sound_length: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub wave_pattern: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! channel3_memory_map {
    ($ f : expr) => {
        Channel3MemoryMap {
            enabled: &$f.enabled,
            output_level: &$f.output_level,
            sound_length: &$f.sound_length,
            wave_pattern: &$f.wave_pattern,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel3MemoryMap<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65306u16 {
            self.enabled.read_value(address - 65306u16)
        } else if address == 65307u16 {
            self.sound_length.read_value(address - 65307u16)
        } else if address == 65308u16 {
            self.output_level.read_value(address - 65308u16)
        } else if address >= 65328u16 && address < 65344u16 {
            self.wave_pattern.read_value(address - 65328u16)
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
