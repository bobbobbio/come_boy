use super::Channel3;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel3 {
    #[allow(clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65306u16 {
            MemoryMappedHardware::read_value(&self.enabled, address - 65306u16)
        } else if address == 65307u16 {
            MemoryMappedHardware::read_value(&self.sound_length, address - 65307u16)
        } else if address == 65308u16 {
            MemoryMappedHardware::read_value(&self.output_level, address - 65308u16)
        } else if (65328u16..65344u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.wave_pattern, address - 65328u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables, clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65306u16 {
            MemoryMappedHardware::set_value(&mut self.enabled, address - 65306u16, value)
        } else if address == 65307u16 {
            MemoryMappedHardware::set_value(&mut self.sound_length, address - 65307u16, value)
        } else if address == 65308u16 {
            MemoryMappedHardware::set_value(&mut self.output_level, address - 65308u16, value)
        } else if (65328u16..65344u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.wave_pattern, address - 65328u16, value)
        }
    }
    #[allow(unused_variables)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_interrupts_enabled(&mut self, enabled: bool) {
        panic!("unexpected set_interrupts_enabled call")
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
