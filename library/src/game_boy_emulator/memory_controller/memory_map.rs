use super::GameBoyMemoryMap;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl<'a> crate::game_boy_emulator::memory_controller::MemoryAccessor for GameBoyMemoryMap<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        if address == 65280u16 {
            MemoryMappedHardware::read_value(&self.joypad, address - 65280u16)
        } else if address == 65281u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.registers.serial_transfer_data,
                address - 65281u16,
            )
        } else if address == 65282u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.registers.serial_transfer_control,
                address - 65282u16,
            )
        } else if address == 65284u16 {
            MemoryMappedHardware::read_value(&self.bridge.registers.divider, address - 65284u16)
        } else if address == 65285u16 {
            MemoryMappedHardware::read_value(&self.bridge.timer.counter, address - 65285u16)
        } else if address == 65286u16 {
            MemoryMappedHardware::read_value(&self.bridge.timer.modulo, address - 65286u16)
        } else if address == 65287u16 {
            MemoryMappedHardware::read_value(&self.bridge.timer.control, address - 65287u16)
        } else if address == 65295u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.registers.interrupt_flag,
                address - 65295u16,
            )
        } else if address == 65344u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.lcdc,
                address - 65344u16,
            )
        } else if address == 65345u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.stat,
                address - 65345u16,
            )
        } else if address == 65346u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.scy,
                address - 65346u16,
            )
        } else if address == 65347u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.scx,
                address - 65347u16,
            )
        } else if address == 65348u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.ly,
                address - 65348u16,
            )
        } else if address == 65349u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.lyc,
                address - 65349u16,
            )
        } else if address == 65350u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.dma,
                address - 65350u16,
            )
        } else if address == 65351u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.bgp,
                address - 65351u16,
            )
        } else if address == 65352u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.obp0,
                address - 65352u16,
            )
        } else if address == 65353u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.obp1,
                address - 65353u16,
            )
        } else if address == 65354u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.wy,
                address - 65354u16,
            )
        } else if address == 65355u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.registers.wx,
                address - 65355u16,
            )
        } else if address == 65535u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.registers.interrupt_enable,
                address - 65535u16,
            )
        } else if address < 32768u16 {
            MemoryMappedHardware::read_value(&self.game_pak, address - 0u16)
        } else if address >= 32768u16 && address < 38912u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.character_data,
                address - 32768u16,
            )
        } else if address >= 38912u16 && address < 39936u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.background_display_data_1,
                address - 38912u16,
            )
        } else if address >= 39936u16 && address < 40960u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.background_display_data_2,
                address - 39936u16,
            )
        } else if address >= 40960u16 && address < 49152u16 {
            MemoryMappedHardware::read_value(&self.game_pak, address - 0u16)
        } else if address >= 49152u16 && address < 56832u16 {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_a, address - 49152u16)
        } else if address >= 56832u16 && address < 57344u16 {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_b, address - 56832u16)
        } else if address >= 57344u16 && address < 65024u16 {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_a, address - 57344u16)
        } else if address >= 65024u16 && address < 65184u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.oam_data,
                address - 65024u16,
            )
        } else if address >= 65184u16 && address < 65280u16 {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.unusable_memory,
                address - 65184u16,
            )
        } else if address >= 65296u16 && address < 65344u16 {
            MemoryMappedHardware::read_value(&self.bridge.sound_controller, address - 0u16)
        } else if address >= 65408u16 && address < 65535u16 {
            MemoryMappedHardware::read_value(&self.bridge.high_ram, address - 65408u16)
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
