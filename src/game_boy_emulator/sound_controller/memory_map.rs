pub struct SoundControllerMemoryMapMut<'a> {
    pub channel1_frequency_high:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_frequency_low:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_sound_length:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_sweep:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_volume_envelope:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_frequency_high:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_frequency_low:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_sound_length:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_volume_envelope:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_enabled:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_frequency_high:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_frequency_low:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_output_level:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_sound_length:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_wave_pattern:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
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
    ( $ f : expr ) => {
        SoundControllerMemoryMapMut {
            channel1_frequency_high: &mut $f.channel1.frequency_high,
            channel1_frequency_low: &mut $f.channel1.frequency_low,
            channel1_sound_length: &mut $f.channel1.sound_length,
            channel1_sweep: &mut $f.channel1.sweep,
            channel1_volume_envelope: &mut $f.channel1.volume_envelope,
            channel2_frequency_high: &mut $f.channel2.frequency_high,
            channel2_frequency_low: &mut $f.channel2.frequency_low,
            channel2_sound_length: &mut $f.channel2.sound_length,
            channel2_volume_envelope: &mut $f.channel2.volume_envelope,
            channel3_enabled: &mut $f.channel3.enabled,
            channel3_frequency_high: &mut $f.channel3.frequency_high,
            channel3_frequency_low: &mut $f.channel3.frequency_low,
            channel3_output_level: &mut $f.channel3.output_level,
            channel3_sound_length: &mut $f.channel3.sound_length,
            channel3_wave_pattern: &mut $f.channel3.wave_pattern,
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
        if address == 65296u16 {
            self.channel1_sweep.read_value(address - 65296u16)
        } else if address == 65297u16 {
            self.channel1_sound_length.read_value(address - 65297u16)
        } else if address == 65298u16 {
            self.channel1_volume_envelope.read_value(address - 65298u16)
        } else if address == 65299u16 {
            self.channel1_frequency_low.read_value(address - 65299u16)
        } else if address == 65300u16 {
            self.channel1_frequency_high.read_value(address - 65300u16)
        } else if address == 65302u16 {
            self.channel2_sound_length.read_value(address - 65302u16)
        } else if address == 65303u16 {
            self.channel2_volume_envelope.read_value(address - 65303u16)
        } else if address == 65304u16 {
            self.channel2_frequency_low.read_value(address - 65304u16)
        } else if address == 65305u16 {
            self.channel2_frequency_high.read_value(address - 65305u16)
        } else if address == 65306u16 {
            self.channel3_enabled.read_value(address - 65306u16)
        } else if address == 65307u16 {
            self.channel3_sound_length.read_value(address - 65307u16)
        } else if address == 65308u16 {
            self.channel3_output_level.read_value(address - 65308u16)
        } else if address == 65309u16 {
            self.channel3_frequency_low.read_value(address - 65309u16)
        } else if address == 65310u16 {
            self.channel3_frequency_high.read_value(address - 65310u16)
        } else if address == 65312u16 {
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
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3_wave_pattern.read_value(address - 65328u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65296u16 {
            self.channel1_sweep.set_value(address - 65296u16, value)
        } else if address == 65297u16 {
            self.channel1_sound_length
                .set_value(address - 65297u16, value)
        } else if address == 65298u16 {
            self.channel1_volume_envelope
                .set_value(address - 65298u16, value)
        } else if address == 65299u16 {
            self.channel1_frequency_low
                .set_value(address - 65299u16, value)
        } else if address == 65300u16 {
            self.channel1_frequency_high
                .set_value(address - 65300u16, value)
        } else if address == 65302u16 {
            self.channel2_sound_length
                .set_value(address - 65302u16, value)
        } else if address == 65303u16 {
            self.channel2_volume_envelope
                .set_value(address - 65303u16, value)
        } else if address == 65304u16 {
            self.channel2_frequency_low
                .set_value(address - 65304u16, value)
        } else if address == 65305u16 {
            self.channel2_frequency_high
                .set_value(address - 65305u16, value)
        } else if address == 65306u16 {
            self.channel3_enabled.set_value(address - 65306u16, value)
        } else if address == 65307u16 {
            self.channel3_sound_length
                .set_value(address - 65307u16, value)
        } else if address == 65308u16 {
            self.channel3_output_level
                .set_value(address - 65308u16, value)
        } else if address == 65309u16 {
            self.channel3_frequency_low
                .set_value(address - 65309u16, value)
        } else if address == 65310u16 {
            self.channel3_frequency_high
                .set_value(address - 65310u16, value)
        } else if address == 65312u16 {
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
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3_wave_pattern
                .set_value(address - 65328u16, value)
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
    pub channel1_frequency_high:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_frequency_low:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_sound_length:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_sweep: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel1_volume_envelope:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_frequency_high:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_frequency_low:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_sound_length:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel2_volume_envelope:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_enabled: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_frequency_high:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_frequency_low:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_output_level:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_sound_length:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub channel3_wave_pattern:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
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
    ( $ f : expr ) => {
        SoundControllerMemoryMap {
            channel1_frequency_high: &$f.channel1.frequency_high,
            channel1_frequency_low: &$f.channel1.frequency_low,
            channel1_sound_length: &$f.channel1.sound_length,
            channel1_sweep: &$f.channel1.sweep,
            channel1_volume_envelope: &$f.channel1.volume_envelope,
            channel2_frequency_high: &$f.channel2.frequency_high,
            channel2_frequency_low: &$f.channel2.frequency_low,
            channel2_sound_length: &$f.channel2.sound_length,
            channel2_volume_envelope: &$f.channel2.volume_envelope,
            channel3_enabled: &$f.channel3.enabled,
            channel3_frequency_high: &$f.channel3.frequency_high,
            channel3_frequency_low: &$f.channel3.frequency_low,
            channel3_output_level: &$f.channel3.output_level,
            channel3_sound_length: &$f.channel3.sound_length,
            channel3_wave_pattern: &$f.channel3.wave_pattern,
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
        if address == 65296u16 {
            self.channel1_sweep.read_value(address - 65296u16)
        } else if address == 65297u16 {
            self.channel1_sound_length.read_value(address - 65297u16)
        } else if address == 65298u16 {
            self.channel1_volume_envelope.read_value(address - 65298u16)
        } else if address == 65299u16 {
            self.channel1_frequency_low.read_value(address - 65299u16)
        } else if address == 65300u16 {
            self.channel1_frequency_high.read_value(address - 65300u16)
        } else if address == 65302u16 {
            self.channel2_sound_length.read_value(address - 65302u16)
        } else if address == 65303u16 {
            self.channel2_volume_envelope.read_value(address - 65303u16)
        } else if address == 65304u16 {
            self.channel2_frequency_low.read_value(address - 65304u16)
        } else if address == 65305u16 {
            self.channel2_frequency_high.read_value(address - 65305u16)
        } else if address == 65306u16 {
            self.channel3_enabled.read_value(address - 65306u16)
        } else if address == 65307u16 {
            self.channel3_sound_length.read_value(address - 65307u16)
        } else if address == 65308u16 {
            self.channel3_output_level.read_value(address - 65308u16)
        } else if address == 65309u16 {
            self.channel3_frequency_low.read_value(address - 65309u16)
        } else if address == 65310u16 {
            self.channel3_frequency_high.read_value(address - 65310u16)
        } else if address == 65312u16 {
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
        } else if address >= 65328u16 && address < 65344u16 {
            self.channel3_wave_pattern.read_value(address - 65328u16)
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
