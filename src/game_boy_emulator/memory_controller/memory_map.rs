pub struct GameBoyMemoryMapMut<'a> {
    pub game_pak: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub high_ram: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub internal_ram_a:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub internal_ram_b:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub joypad_register:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_background_display_data_1:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_background_display_data_2:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_character_data:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_oam_data:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_bgp:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_dma:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_lcdc:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_ly:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_lyc:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_obp0:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_obp1:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_scx:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_scy:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_stat:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_wx:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_wy:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_unusable_memory:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_divider:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_interrupt_enable:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_interrupt_flag:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_serial_transfer_control:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_serial_transfer_data:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sound_controller:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_control:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_counter:
        &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_modulo: &'a mut dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! game_boy_memory_map_mut {
    ( $ f : expr ) => {
        GameBoyMemoryMapMut {
            game_pak: &mut $f.game_pak,
            high_ram: &mut $f.high_ram,
            internal_ram_a: &mut $f.internal_ram_a,
            internal_ram_b: &mut $f.internal_ram_b,
            joypad_register: &mut $f.joypad_register,
            lcd_controller_background_display_data_1: &mut $f
                .lcd_controller
                .background_display_data_1,
            lcd_controller_background_display_data_2: &mut $f
                .lcd_controller
                .background_display_data_2,
            lcd_controller_character_data: &mut $f.lcd_controller.character_data,
            lcd_controller_oam_data: &mut $f.lcd_controller.oam_data,
            lcd_controller_registers_bgp: &mut $f.lcd_controller.registers.bgp,
            lcd_controller_registers_dma: &mut $f.lcd_controller.registers.dma,
            lcd_controller_registers_lcdc: &mut $f.lcd_controller.registers.lcdc,
            lcd_controller_registers_ly: &mut $f.lcd_controller.registers.ly,
            lcd_controller_registers_lyc: &mut $f.lcd_controller.registers.lyc,
            lcd_controller_registers_obp0: &mut $f.lcd_controller.registers.obp0,
            lcd_controller_registers_obp1: &mut $f.lcd_controller.registers.obp1,
            lcd_controller_registers_scx: &mut $f.lcd_controller.registers.scx,
            lcd_controller_registers_scy: &mut $f.lcd_controller.registers.scy,
            lcd_controller_registers_stat: &mut $f.lcd_controller.registers.stat,
            lcd_controller_registers_wx: &mut $f.lcd_controller.registers.wx,
            lcd_controller_registers_wy: &mut $f.lcd_controller.registers.wy,
            lcd_controller_unusable_memory: &mut $f.lcd_controller.unusable_memory,
            registers_divider: &mut $f.registers.divider,
            registers_interrupt_enable: &mut $f.registers.interrupt_enable,
            registers_interrupt_flag: &mut $f.registers.interrupt_flag,
            registers_serial_transfer_control: &mut $f.registers.serial_transfer_control,
            registers_serial_transfer_data: &mut $f.registers.serial_transfer_data,
            sound_controller: &mut $f.sound_controller,
            timer_control: &mut $f.timer.control,
            timer_counter: &mut $f.timer.counter,
            timer_modulo: &mut $f.timer.modulo,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for GameBoyMemoryMapMut<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65280u16 {
            self.joypad_register.read_value(address - 65280u16)
        } else if address == 65281u16 {
            self.registers_serial_transfer_data
                .read_value(address - 65281u16)
        } else if address == 65282u16 {
            self.registers_serial_transfer_control
                .read_value(address - 65282u16)
        } else if address == 65284u16 {
            self.registers_divider.read_value(address - 65284u16)
        } else if address == 65285u16 {
            self.timer_counter.read_value(address - 65285u16)
        } else if address == 65286u16 {
            self.timer_modulo.read_value(address - 65286u16)
        } else if address == 65287u16 {
            self.timer_control.read_value(address - 65287u16)
        } else if address == 65295u16 {
            self.registers_interrupt_flag.read_value(address - 65295u16)
        } else if address == 65344u16 {
            self.lcd_controller_registers_lcdc
                .read_value(address - 65344u16)
        } else if address == 65345u16 {
            self.lcd_controller_registers_stat
                .read_value(address - 65345u16)
        } else if address == 65346u16 {
            self.lcd_controller_registers_scy
                .read_value(address - 65346u16)
        } else if address == 65347u16 {
            self.lcd_controller_registers_scx
                .read_value(address - 65347u16)
        } else if address == 65348u16 {
            self.lcd_controller_registers_ly
                .read_value(address - 65348u16)
        } else if address == 65349u16 {
            self.lcd_controller_registers_lyc
                .read_value(address - 65349u16)
        } else if address == 65350u16 {
            self.lcd_controller_registers_dma
                .read_value(address - 65350u16)
        } else if address == 65351u16 {
            self.lcd_controller_registers_bgp
                .read_value(address - 65351u16)
        } else if address == 65352u16 {
            self.lcd_controller_registers_obp0
                .read_value(address - 65352u16)
        } else if address == 65353u16 {
            self.lcd_controller_registers_obp1
                .read_value(address - 65353u16)
        } else if address == 65354u16 {
            self.lcd_controller_registers_wy
                .read_value(address - 65354u16)
        } else if address == 65355u16 {
            self.lcd_controller_registers_wx
                .read_value(address - 65355u16)
        } else if address == 65535u16 {
            self.registers_interrupt_enable
                .read_value(address - 65535u16)
        } else if address < 32768u16 {
            self.game_pak.read_value(address - 0u16)
        } else if address >= 32768u16 && address < 38912u16 {
            self.lcd_controller_character_data
                .read_value(address - 32768u16)
        } else if address >= 38912u16 && address < 39936u16 {
            self.lcd_controller_background_display_data_1
                .read_value(address - 38912u16)
        } else if address >= 39936u16 && address < 40960u16 {
            self.lcd_controller_background_display_data_2
                .read_value(address - 39936u16)
        } else if address >= 40960u16 && address < 49152u16 {
            self.game_pak.read_value(address - 0u16)
        } else if address >= 49152u16 && address < 56832u16 {
            self.internal_ram_a.read_value(address - 49152u16)
        } else if address >= 56832u16 && address < 57344u16 {
            self.internal_ram_b.read_value(address - 56832u16)
        } else if address >= 57344u16 && address < 65024u16 {
            self.internal_ram_a.read_value(address - 57344u16)
        } else if address >= 65024u16 && address < 65184u16 {
            self.lcd_controller_oam_data.read_value(address - 65024u16)
        } else if address >= 65184u16 && address < 65280u16 {
            self.lcd_controller_unusable_memory
                .read_value(address - 65184u16)
        } else if address >= 65296u16 && address < 65344u16 {
            self.sound_controller.read_value(address - 0u16)
        } else if address >= 65408u16 && address < 65535u16 {
            self.high_ram.read_value(address - 65408u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables)]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65280u16 {
            self.joypad_register.set_value(address - 65280u16, value)
        } else if address == 65281u16 {
            self.registers_serial_transfer_data
                .set_value(address - 65281u16, value)
        } else if address == 65284u16 {
            self.registers_divider.set_value(address - 65284u16, value)
        } else if address == 65285u16 {
            self.timer_counter.set_value(address - 65285u16, value)
        } else if address == 65286u16 {
            self.timer_modulo.set_value(address - 65286u16, value)
        } else if address == 65287u16 {
            self.timer_control.set_value(address - 65287u16, value)
        } else if address == 65295u16 {
            self.registers_interrupt_flag
                .set_value(address - 65295u16, value)
        } else if address == 65344u16 {
            self.lcd_controller_registers_lcdc
                .set_value(address - 65344u16, value)
        } else if address == 65346u16 {
            self.lcd_controller_registers_scy
                .set_value(address - 65346u16, value)
        } else if address == 65347u16 {
            self.lcd_controller_registers_scx
                .set_value(address - 65347u16, value)
        } else if address == 65348u16 {
            self.lcd_controller_registers_ly
                .set_value(address - 65348u16, value)
        } else if address == 65349u16 {
            self.lcd_controller_registers_lyc
                .set_value(address - 65349u16, value)
        } else if address == 65350u16 {
            self.lcd_controller_registers_dma
                .set_value(address - 65350u16, value)
        } else if address == 65351u16 {
            self.lcd_controller_registers_bgp
                .set_value(address - 65351u16, value)
        } else if address == 65352u16 {
            self.lcd_controller_registers_obp0
                .set_value(address - 65352u16, value)
        } else if address == 65353u16 {
            self.lcd_controller_registers_obp1
                .set_value(address - 65353u16, value)
        } else if address == 65354u16 {
            self.lcd_controller_registers_wy
                .set_value(address - 65354u16, value)
        } else if address == 65355u16 {
            self.lcd_controller_registers_wx
                .set_value(address - 65355u16, value)
        } else if address == 65535u16 {
            self.registers_interrupt_enable
                .set_value(address - 65535u16, value)
        } else if address < 32768u16 {
            self.game_pak.set_value(address - 0u16, value)
        } else if address >= 32768u16 && address < 38912u16 {
            self.lcd_controller_character_data
                .set_value(address - 32768u16, value)
        } else if address >= 38912u16 && address < 39936u16 {
            self.lcd_controller_background_display_data_1
                .set_value(address - 38912u16, value)
        } else if address >= 39936u16 && address < 40960u16 {
            self.lcd_controller_background_display_data_2
                .set_value(address - 39936u16, value)
        } else if address >= 40960u16 && address < 49152u16 {
            self.game_pak.set_value(address - 0u16, value)
        } else if address >= 49152u16 && address < 56832u16 {
            self.internal_ram_a.set_value(address - 49152u16, value)
        } else if address >= 56832u16 && address < 57344u16 {
            self.internal_ram_b.set_value(address - 56832u16, value)
        } else if address >= 57344u16 && address < 65024u16 {
            self.internal_ram_a.set_value(address - 57344u16, value)
        } else if address >= 65024u16 && address < 65184u16 {
            self.lcd_controller_oam_data
                .set_value(address - 65024u16, value)
        } else if address >= 65184u16 && address < 65280u16 {
            self.lcd_controller_unusable_memory
                .set_value(address - 65184u16, value)
        } else if address >= 65296u16 && address < 65344u16 {
            self.sound_controller.set_value(address - 0u16, value)
        } else if address >= 65408u16 && address < 65535u16 {
            self.high_ram.set_value(address - 65408u16, value)
        }
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
pub struct GameBoyMemoryMap<'a> {
    pub game_pak: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub high_ram: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub internal_ram_a: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub internal_ram_b: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub joypad_register: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_background_display_data_1:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_background_display_data_2:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_character_data:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_oam_data:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_bgp:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_dma:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_lcdc:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_ly:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_lyc:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_obp0:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_obp1:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_scx:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_scy:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_stat:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_wx:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_registers_wy:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub lcd_controller_unusable_memory:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_divider:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_interrupt_enable:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_interrupt_flag:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_serial_transfer_control:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub registers_serial_transfer_data:
        &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub sound_controller: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_control: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_counter: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
    pub timer_modulo: &'a dyn crate::game_boy_emulator::memory_controller::MemoryMappedHardware,
}
#[macro_export]
macro_rules! game_boy_memory_map {
    ( $ f : expr ) => {
        GameBoyMemoryMap {
            game_pak: &$f.game_pak,
            high_ram: &$f.high_ram,
            internal_ram_a: &$f.internal_ram_a,
            internal_ram_b: &$f.internal_ram_b,
            joypad_register: &$f.joypad_register,
            lcd_controller_background_display_data_1: &$f.lcd_controller.background_display_data_1,
            lcd_controller_background_display_data_2: &$f.lcd_controller.background_display_data_2,
            lcd_controller_character_data: &$f.lcd_controller.character_data,
            lcd_controller_oam_data: &$f.lcd_controller.oam_data,
            lcd_controller_registers_bgp: &$f.lcd_controller.registers.bgp,
            lcd_controller_registers_dma: &$f.lcd_controller.registers.dma,
            lcd_controller_registers_lcdc: &$f.lcd_controller.registers.lcdc,
            lcd_controller_registers_ly: &$f.lcd_controller.registers.ly,
            lcd_controller_registers_lyc: &$f.lcd_controller.registers.lyc,
            lcd_controller_registers_obp0: &$f.lcd_controller.registers.obp0,
            lcd_controller_registers_obp1: &$f.lcd_controller.registers.obp1,
            lcd_controller_registers_scx: &$f.lcd_controller.registers.scx,
            lcd_controller_registers_scy: &$f.lcd_controller.registers.scy,
            lcd_controller_registers_stat: &$f.lcd_controller.registers.stat,
            lcd_controller_registers_wx: &$f.lcd_controller.registers.wx,
            lcd_controller_registers_wy: &$f.lcd_controller.registers.wy,
            lcd_controller_unusable_memory: &$f.lcd_controller.unusable_memory,
            registers_divider: &$f.registers.divider,
            registers_interrupt_enable: &$f.registers.interrupt_enable,
            registers_interrupt_flag: &$f.registers.interrupt_flag,
            registers_serial_transfer_control: &$f.registers.serial_transfer_control,
            registers_serial_transfer_data: &$f.registers.serial_transfer_data,
            sound_controller: &$f.sound_controller,
            timer_control: &$f.timer.control,
            timer_counter: &$f.timer.counter,
            timer_modulo: &$f.timer.modulo,
        }
    };
}
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for GameBoyMemoryMap<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65280u16 {
            self.joypad_register.read_value(address - 65280u16)
        } else if address == 65281u16 {
            self.registers_serial_transfer_data
                .read_value(address - 65281u16)
        } else if address == 65282u16 {
            self.registers_serial_transfer_control
                .read_value(address - 65282u16)
        } else if address == 65284u16 {
            self.registers_divider.read_value(address - 65284u16)
        } else if address == 65285u16 {
            self.timer_counter.read_value(address - 65285u16)
        } else if address == 65286u16 {
            self.timer_modulo.read_value(address - 65286u16)
        } else if address == 65287u16 {
            self.timer_control.read_value(address - 65287u16)
        } else if address == 65295u16 {
            self.registers_interrupt_flag.read_value(address - 65295u16)
        } else if address == 65344u16 {
            self.lcd_controller_registers_lcdc
                .read_value(address - 65344u16)
        } else if address == 65345u16 {
            self.lcd_controller_registers_stat
                .read_value(address - 65345u16)
        } else if address == 65346u16 {
            self.lcd_controller_registers_scy
                .read_value(address - 65346u16)
        } else if address == 65347u16 {
            self.lcd_controller_registers_scx
                .read_value(address - 65347u16)
        } else if address == 65348u16 {
            self.lcd_controller_registers_ly
                .read_value(address - 65348u16)
        } else if address == 65349u16 {
            self.lcd_controller_registers_lyc
                .read_value(address - 65349u16)
        } else if address == 65350u16 {
            self.lcd_controller_registers_dma
                .read_value(address - 65350u16)
        } else if address == 65351u16 {
            self.lcd_controller_registers_bgp
                .read_value(address - 65351u16)
        } else if address == 65352u16 {
            self.lcd_controller_registers_obp0
                .read_value(address - 65352u16)
        } else if address == 65353u16 {
            self.lcd_controller_registers_obp1
                .read_value(address - 65353u16)
        } else if address == 65354u16 {
            self.lcd_controller_registers_wy
                .read_value(address - 65354u16)
        } else if address == 65355u16 {
            self.lcd_controller_registers_wx
                .read_value(address - 65355u16)
        } else if address == 65535u16 {
            self.registers_interrupt_enable
                .read_value(address - 65535u16)
        } else if address < 32768u16 {
            self.game_pak.read_value(address - 0u16)
        } else if address >= 32768u16 && address < 38912u16 {
            self.lcd_controller_character_data
                .read_value(address - 32768u16)
        } else if address >= 38912u16 && address < 39936u16 {
            self.lcd_controller_background_display_data_1
                .read_value(address - 38912u16)
        } else if address >= 39936u16 && address < 40960u16 {
            self.lcd_controller_background_display_data_2
                .read_value(address - 39936u16)
        } else if address >= 40960u16 && address < 49152u16 {
            self.game_pak.read_value(address - 0u16)
        } else if address >= 49152u16 && address < 56832u16 {
            self.internal_ram_a.read_value(address - 49152u16)
        } else if address >= 56832u16 && address < 57344u16 {
            self.internal_ram_b.read_value(address - 56832u16)
        } else if address >= 57344u16 && address < 65024u16 {
            self.internal_ram_a.read_value(address - 57344u16)
        } else if address >= 65024u16 && address < 65184u16 {
            self.lcd_controller_oam_data.read_value(address - 65024u16)
        } else if address >= 65184u16 && address < 65280u16 {
            self.lcd_controller_unusable_memory
                .read_value(address - 65184u16)
        } else if address >= 65296u16 && address < 65344u16 {
            self.sound_controller.read_value(address - 0u16)
        } else if address >= 65408u16 && address < 65535u16 {
            self.high_ram.read_value(address - 65408u16)
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
