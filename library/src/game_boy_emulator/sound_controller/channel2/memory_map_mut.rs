use super::Channel2;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl crate::game_boy_emulator::memory_controller::MemoryAccessor for Channel2 {
    #[allow(clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65302u16 {
            MemoryMappedHardware::read_value(&self.sound_length, address - 65302u16)
        } else if address == 65303u16 {
            MemoryMappedHardware::read_value(&self.volume_envelope, address - 65303u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables, clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65302u16 {
            MemoryMappedHardware::set_value(&mut self.sound_length, address - 65302u16, value)
        } else if address == 65303u16 {
            MemoryMappedHardware::set_value(&mut self.volume_envelope, address - 65303u16, value)
        }
    }
    #[allow(unused_variables)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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
