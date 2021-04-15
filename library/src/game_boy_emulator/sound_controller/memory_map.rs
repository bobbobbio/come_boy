pub struct SoundControllerMemoryMapMut<'a> {
    pub channel1: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_counter:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_polynomial_counter:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_sound_length:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_volume_envelope:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel_control:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub enabled: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub output_terminal:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! sound_controller_memory_map_mut {
    ($ f : expr) => {
        SoundControllerMemoryMapMut {
            channel1: &mut $f.channel1,
            channel2: &mut $f.channel2,
            channel3: &mut $f.channel3,
            channel4_counter: &mut $f.channel4.counter,
            channel4_polynomial_counter: &mut $f.channel4.polynomial_counter,
            channel4_sound_length: &mut $f.channel4.sound_length,
            channel4_volume_envelope: &mut $f.channel4.volume_envelope,
            channel_control: &mut $f.channel_control,
            enabled: &mut $f.enabled,
            output_terminal: &mut $f.output_terminal,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor
    for SoundControllerMemoryMapMut<'a>
{
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65312u16 {
            self.channel4_sound_length.read_value(address - 65312u16)
        } else if address == 65313u16 {
            self.channel4_volume_envelope.read_value(address - 65313u16)
        } else if address == 65314u16 {
            self.channel4_polynomial_counter
                .read_value(address - 65314u16)
        } else if address == 65315u16 {
            self.channel4_counter.read_value(address - 65315u16)
        } else if address == 65316u16 {
            self.channel_control.read_value(address - 65316u16)
        } else if address == 65317u16 {
            self.output_terminal.read_value(address - 65317u16)
        } else if address == 65318u16 {
            self.enabled.read_value(address - 65318u16)
        } else if address >= 65296u16 && address < 65301u16 {
            self.channel1.read_value(address - 0u16)
        } else if address >= 65302u16 && address < 65306u16 {
            self.channel2.read_value(address - 0u16)
        } else if address >= 65306u16 && address < 65311u16 {
            self.channel3.read_value(address - 0u16)
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3.read_value(address - 0u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65312u16 {
            self.channel4_sound_length
                .set_value(address - 65312u16, value)
        } else if address == 65313u16 {
            self.channel4_volume_envelope
                .set_value(address - 65313u16, value)
        } else if address == 65314u16 {
            self.channel4_polynomial_counter
                .set_value(address - 65314u16, value)
        } else if address == 65315u16 {
            self.channel4_counter.set_value(address - 65315u16, value)
        } else if address == 65316u16 {
            self.channel_control.set_value(address - 65316u16, value)
        } else if address == 65317u16 {
            self.output_terminal.set_value(address - 65317u16, value)
        } else if address >= 65296u16 && address < 65301u16 {
            self.channel1.set_value(address - 0u16, value)
        } else if address >= 65302u16 && address < 65306u16 {
            self.channel2.set_value(address - 0u16, value)
        } else if address >= 65306u16 && address < 65311u16 {
            self.channel3.set_value(address - 0u16, value)
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3.set_value(address - 0u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
pub struct SoundControllerMemoryMap<'a> {
    pub channel1: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_counter: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_polynomial_counter:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_sound_length:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel4_volume_envelope:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel_control: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub enabled: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub output_terminal: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! sound_controller_memory_map {
    ($ f : expr) => {
        SoundControllerMemoryMap {
            channel1: &$f.channel1,
            channel2: &$f.channel2,
            channel3: &$f.channel3,
            channel4_counter: &$f.channel4.counter,
            channel4_polynomial_counter: &$f.channel4.polynomial_counter,
            channel4_sound_length: &$f.channel4.sound_length,
            channel4_volume_envelope: &$f.channel4.volume_envelope,
            channel_control: &$f.channel_control,
            enabled: &$f.enabled,
            output_terminal: &$f.output_terminal,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor
    for SoundControllerMemoryMap<'a>
{
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65312u16 {
            self.channel4_sound_length.read_value(address - 65312u16)
        } else if address == 65313u16 {
            self.channel4_volume_envelope.read_value(address - 65313u16)
        } else if address == 65314u16 {
            self.channel4_polynomial_counter
                .read_value(address - 65314u16)
        } else if address == 65315u16 {
            self.channel4_counter.read_value(address - 65315u16)
        } else if address == 65316u16 {
            self.channel_control.read_value(address - 65316u16)
        } else if address == 65317u16 {
            self.output_terminal.read_value(address - 65317u16)
        } else if address == 65318u16 {
            self.enabled.read_value(address - 65318u16)
        } else if address >= 65296u16 && address < 65301u16 {
            self.channel1.read_value(address - 0u16)
        } else if address >= 65302u16 && address < 65306u16 {
            self.channel2.read_value(address - 0u16)
        } else if address >= 65306u16 && address < 65311u16 {
            self.channel3.read_value(address - 0u16)
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3.read_value(address - 0u16)
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
