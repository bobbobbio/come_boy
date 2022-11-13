// Copyright 2017 Remi Bernotavicius

use crate::emulator_common::disassembler::{Disassembler, MemoryAccessor, MemoryDescription};
use crate::io::{self, Result};

pub use crate::game_boy_emulator::disassembler::rgbds_assembly::RGBDSInstructionPrinterFactory;

mod rgbds_assembly;

pub fn create_disassembler<'a>(
    memory_accessor: &'a dyn MemoryAccessor,
    stream_out: &'a mut dyn io::Write,
) -> Disassembler<'a, RGBDSInstructionPrinterFactory> {
    Disassembler::new(memory_accessor, RGBDSInstructionPrinterFactory, stream_out)
}

pub struct ROMAccessor<'a> {
    rom: &'a [u8],
}

impl<'a> ROMAccessor<'a> {
    pub fn new(rom: &'a [u8]) -> ROMAccessor<'a> {
        // XXX: Can't yet disassemble things bigger than this.
        assert!(rom.len() < 0xFFFF);

        Self { rom }
    }
}

impl<'a> MemoryAccessor for ROMAccessor<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn set_memory(&mut self, _address: u16, _value: u8) {}

    fn set_interrupts_enabled(&mut self, _enabled: bool) {}

    fn describe_address(&self, address: u16) -> MemoryDescription {
        // This should probably live somewhere else, possibly in some GamePak object.
        match address {
            // Nintendo Logo
            0x0104 => MemoryDescription::Data(48),
            // Game Name / Designation
            0x0134 => MemoryDescription::Ascii(15),
            // Color Compatibility Byte
            0x0143 => MemoryDescription::Data(1),
            // New Licensee Code
            0x0144 => MemoryDescription::Data(2),
            // SGB Compatibility Byte
            0x0146 => MemoryDescription::Data(1),
            // Cart Type
            0x0147 => MemoryDescription::Data(1),
            // ROM Size
            0x0148 => MemoryDescription::Data(1),
            // RAM Size
            0x0149 => MemoryDescription::Data(1),
            // Destination Code
            0x014A => MemoryDescription::Data(1),
            // Old Licensee Code
            0x014B => MemoryDescription::Data(1),
            // Mask ROM version
            0x014C => MemoryDescription::Data(1),
            // Complement checksum
            0x014D => MemoryDescription::Data(1),
            // Checksum
            0x014E => MemoryDescription::Data(2),
            _ => MemoryDescription::Instruction,
        }
    }
}

pub fn disassemble_game_boy_rom(
    rom: &[u8],
    include_opcodes: bool,
    mut output: impl io::Write,
) -> Result<()> {
    let ma = ROMAccessor::new(rom);
    let mut disassembler = create_disassembler(&ma, &mut output);
    disassembler.disassemble(0..rom.len() as u16, include_opcodes)
}

#[cfg(test)]
mod tests;
