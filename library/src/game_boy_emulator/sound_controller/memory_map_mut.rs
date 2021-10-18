use super::SoundController;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl crate::game_boy_emulator::memory_controller::MemoryAccessor for SoundController {
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
        } else if address >= 65296u16 && address < 65301u16 {
            MemoryMappedHardware::read_value(&self.channel1, address - 0u16)
        } else if address >= 65302u16 && address < 65306u16 {
            MemoryMappedHardware::read_value(&self.channel2, address - 0u16)
        } else if address >= 65306u16 && address < 65311u16 {
            MemoryMappedHardware::read_value(&self.channel3, address - 0u16)
        } else if address >= 65328u16 && address < 65344u16 {
            MemoryMappedHardware::read_value(&self.channel3, address - 0u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
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
        } else if address >= 65296u16 && address < 65301u16 {
            MemoryMappedHardware::set_value(&mut self.channel1, address - 0u16, value)
        } else if address >= 65302u16 && address < 65306u16 {
            MemoryMappedHardware::set_value(&mut self.channel2, address - 0u16, value)
        } else if address >= 65306u16 && address < 65311u16 {
            MemoryMappedHardware::set_value(&mut self.channel3, address - 0u16, value)
        } else if address >= 65328u16 && address < 65344u16 {
            MemoryMappedHardware::set_value(&mut self.channel3, address - 0u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
