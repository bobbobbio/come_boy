use super::GameBoyMemoryMapMut;
use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
impl<'a, Storage> crate::game_boy_emulator::memory_controller::MemoryAccessor
    for GameBoyMemoryMapMut<'a, Storage>
where
    Storage: crate::storage::PersistentStorage,
{
    #[allow(clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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
            MemoryMappedHardware::read_value(
                &(&self.bridge.timer, &self.bridge.scheduler),
                address - 65287u16,
            )
        } else if address == 65295u16 {
            MemoryMappedHardware::read_value(
                &(
                    &self.bridge.registers.interrupt_flag,
                    &self.bridge.scheduler,
                ),
                address - 65295u16,
            )
        } else if address == 65344u16 {
            MemoryMappedHardware::read_value(
                &(&self.bridge.lcd_controller, &self.bridge.scheduler),
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
                &(
                    &self.bridge.lcd_controller.registers.dma,
                    &self.bridge.scheduler,
                ),
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
                &(
                    &self.bridge.registers.interrupt_enable_mask,
                    &self.bridge.scheduler,
                ),
                address - 65535u16,
            )
        } else if address < 32768u16 {
            MemoryMappedHardware::read_value(&self.game_pak, address - 0u16)
        } else if (32768u16..38912u16).contains(&address) {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.character_data,
                address - 32768u16,
            )
        } else if (38912u16..39936u16).contains(&address) {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.background_display_data_1,
                address - 38912u16,
            )
        } else if (39936u16..40960u16).contains(&address) {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.background_display_data_2,
                address - 39936u16,
            )
        } else if (40960u16..49152u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.game_pak, address - 0u16)
        } else if (49152u16..56832u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_a, address - 49152u16)
        } else if (56832u16..57344u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_b, address - 56832u16)
        } else if (57344u16..65024u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.bridge.internal_ram_a, address - 57344u16)
        } else if (65024u16..65184u16).contains(&address) {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.oam_data,
                address - 65024u16,
            )
        } else if (65184u16..65280u16).contains(&address) {
            MemoryMappedHardware::read_value(
                &self.bridge.lcd_controller.unusable_memory,
                address - 65184u16,
            )
        } else if (65296u16..65344u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.bridge.sound_controller, address - 0u16)
        } else if (65408u16..65535u16).contains(&address) {
            MemoryMappedHardware::read_value(&self.bridge.high_ram, address - 65408u16)
        } else {
            0xFF
        }
    }
    #[allow(unused_variables, clippy::identity_op, clippy::if_same_then_else)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        if address == 65280u16 {
            MemoryMappedHardware::set_value(&mut self.joypad, address - 65280u16, value)
        } else if address == 65281u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.registers.serial_transfer_data,
                address - 65281u16,
                value,
            )
        } else if address == 65284u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.registers.divider,
                address - 65284u16,
                value,
            )
        } else if address == 65285u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.timer.counter,
                address - 65285u16,
                value,
            )
        } else if address == 65286u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.timer.modulo,
                address - 65286u16,
                value,
            )
        } else if address == 65287u16 {
            MemoryMappedHardware::set_value(
                &mut (&mut self.bridge.timer, &mut self.bridge.scheduler),
                address - 65287u16,
                value,
            )
        } else if address == 65295u16 {
            MemoryMappedHardware::set_value(
                &mut (
                    &mut self.bridge.registers.interrupt_flag,
                    &mut self.bridge.scheduler,
                ),
                address - 65295u16,
                value,
            )
        } else if address == 65344u16 {
            MemoryMappedHardware::set_value(
                &mut (&mut self.bridge.lcd_controller, &mut self.bridge.scheduler),
                address - 65344u16,
                value,
            )
        } else if address == 65345u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.stat,
                address - 65345u16,
                value,
            )
        } else if address == 65346u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.scy,
                address - 65346u16,
                value,
            )
        } else if address == 65347u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.scx,
                address - 65347u16,
                value,
            )
        } else if address == 65348u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.ly,
                address - 65348u16,
                value,
            )
        } else if address == 65349u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.lyc,
                address - 65349u16,
                value,
            )
        } else if address == 65350u16 {
            MemoryMappedHardware::set_value(
                &mut (
                    &mut self.bridge.lcd_controller.registers.dma,
                    &mut self.bridge.scheduler,
                ),
                address - 65350u16,
                value,
            )
        } else if address == 65351u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.bgp,
                address - 65351u16,
                value,
            )
        } else if address == 65352u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.obp0,
                address - 65352u16,
                value,
            )
        } else if address == 65353u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.obp1,
                address - 65353u16,
                value,
            )
        } else if address == 65354u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.wy,
                address - 65354u16,
                value,
            )
        } else if address == 65355u16 {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.registers.wx,
                address - 65355u16,
                value,
            )
        } else if address == 65535u16 {
            MemoryMappedHardware::set_value(
                &mut (
                    &mut self.bridge.registers.interrupt_enable_mask,
                    &mut self.bridge.scheduler,
                ),
                address - 65535u16,
                value,
            )
        } else if address < 32768u16 {
            MemoryMappedHardware::set_value(&mut self.game_pak, address - 0u16, value)
        } else if (32768u16..38912u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.character_data,
                address - 32768u16,
                value,
            )
        } else if (38912u16..39936u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.background_display_data_1,
                address - 38912u16,
                value,
            )
        } else if (39936u16..40960u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.background_display_data_2,
                address - 39936u16,
                value,
            )
        } else if (40960u16..49152u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.game_pak, address - 0u16, value)
        } else if (49152u16..56832u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.internal_ram_a,
                address - 49152u16,
                value,
            )
        } else if (56832u16..57344u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.internal_ram_b,
                address - 56832u16,
                value,
            )
        } else if (57344u16..65024u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.internal_ram_a,
                address - 57344u16,
                value,
            )
        } else if (65024u16..65184u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.oam_data,
                address - 65024u16,
                value,
            )
        } else if (65184u16..65280u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.lcd_controller.unusable_memory,
                address - 65184u16,
                value,
            )
        } else if (65296u16..65344u16).contains(&address) {
            MemoryMappedHardware::set_value(
                &mut self.bridge.sound_controller,
                address - 0u16,
                value,
            )
        } else if (65408u16..65535u16).contains(&address) {
            MemoryMappedHardware::set_value(&mut self.bridge.high_ram, address - 65408u16, value)
        }
    }
    #[allow(unused_variables)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_interrupts_enabled(&mut self, enabled: bool) {
        MemoryMappedHardware::set_interrupts_enabled(
            &mut (
                &mut self.bridge.registers.interrupts_enabled,
                &mut self.bridge.scheduler,
            ),
            enabled,
        )
    }
    fn describe_address(
        &self,
        _address: u16,
    ) -> crate::game_boy_emulator::memory_controller::MemoryDescription {
        crate::game_boy_emulator::memory_controller::MemoryDescription::Instruction
    }
}
