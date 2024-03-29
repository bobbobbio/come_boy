use super::SoundController;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl crate::game_boy_emulator::memory_controller::MemoryAccessor for SoundController {
    #[allow(clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65312u16 {
            MemoryMappedHardware::read_value(&self.channel4.sound_length, address - 65312u16)
        } else if address == 65313u16 {
            MemoryMappedHardware::read_value(&self.channel4.volume_envelope, address - 65313u16)
        } else if address == 65314u16 {
            MemoryMappedHardware::read_value(&self.channel4.polynomial_counter, address - 65314u16)
        } else if address == 65315u16 {
            MemoryMappedHardware::read_value(&self.channel4.counter, address - 65315u16)
        } else if address == 65316u16 {
            MemoryMappedHardware::read_value(&self.channel_control, address - 65316u16)
        } else if address == 65317u16 {
            MemoryMappedHardware::read_value(&self.output_terminal, address - 65317u16)
        } else if (65296u16..65301u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.channel1, address - 0u16)
        } else if (65302u16..65306u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.channel2, address - 0u16)
        } else if (65306u16..65311u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.channel3, address - 0u16)
        } else if (65328u16..65344u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.channel3, address - 0u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables, clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65312u16 {
            MemoryMappedHardware::set_value(
                &mut self.channel4.sound_length,
                address - 65312u16,
                value,
            )
        } else if address == 65313u16 {
            MemoryMappedHardware::set_value(
                &mut self.channel4.volume_envelope,
                address - 65313u16,
                value,
            )
        } else if address == 65314u16 {
            MemoryMappedHardware::set_value(
                &mut self.channel4.polynomial_counter,
                address - 65314u16,
                value,
            )
        } else if address == 65315u16 {
            MemoryMappedHardware::set_value(&mut self.channel4.counter, address - 65315u16, value)
        } else if address == 65316u16 {
            MemoryMappedHardware::set_value(&mut self.channel_control, address - 65316u16, value)
        } else if address == 65317u16 {
            MemoryMappedHardware::set_value(&mut self.output_terminal, address - 65317u16, value)
        } else if (65296u16..65301u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.channel1, address - 0u16, value)
        } else if (65302u16..65306u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.channel2, address - 0u16, value)
        } else if (65306u16..65311u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.channel3, address - 0u16, value)
        } else if (65328u16..65344u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.channel3, address - 0u16, value)
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
