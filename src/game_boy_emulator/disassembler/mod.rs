// Copyright 2017 Remi Bernotavicius

use std::io::{self, Result};
use emulator_common::Disassembler;

mod rgbds_assembly;

use game_boy_emulator::disassembler::rgbds_assembly::RGBDSInstructionPrinterFactory;

#[cfg(test)]
use emulator_common::do_disassembler_test;

pub fn create_disassembler<'a>(rom: &'a [u8], stream_out: &'a mut io::Write)
    -> Disassembler<'a, RGBDSInstructionPrinterFactory>
{
    Disassembler::new(rom, RGBDSInstructionPrinterFactory, stream_out)
}

pub fn disassemble_game_boy_rom(rom: &[u8], include_opcodes: bool) -> Result<()>
{
    let stdout = &mut io::stdout();
    let mut disassembler = create_disassembler(rom, stdout);
    disassembler.disassemble(include_opcodes)
}

#[test]
fn disassembler_rgbds_test() {
    do_disassembler_test(
        RGBDSInstructionPrinterFactory,
        &[
            0xcd, 0xd6, 0x35, 0x21, 0x2d, 0xd7, 0xcb, 0xae, 0xcd, 0x29, 0x24, 0x21, 0x26, 0xd1,
            0xcb, 0xee, 0xcb, 0xf6, 0xaf, 0xea, 0x6b, 0xcd, 0xcd, 0xaf, 0x20, 0xcd, 0xaf, 0x20,
            0xcd, 0xba, 0x20, 0xfa, 0x36, 0xd7, 0xcb, 0x77, 0xc4, 0x9e, 0x03, 0xfa, 0xc5, 0xcf,
            0xa7, 0xc2, 0xb5, 0x05, 0xcd, 0x4d, 0x0f, 0x06, 0x07, 0x21, 0x88, 0x69, 0xcd, 0xd6,
            0x35
        ], "\
            0000000 cd d6 35 call $35d6\n\
            0000003 21 2d d7 ld   hl,$d72d\n\
            0000006 cb ae    res  5,[hl]\n\
            0000008 cd 29 24 call $2429\n\
            000000b 21 26 d1 ld   hl,$d126\n\
            000000e cb ee    set  5,[hl]\n\
            0000010 cb f6    set  6,[hl]\n\
            0000012 af       xor  a\n\
            0000013 ea 6b cd ld   [$cd6b],a\n\
            0000016 cd af 20 call $20af\n\
            0000019 cd af 20 call $20af\n\
            000001c cd ba 20 call $20ba\n\
            000001f fa 36 d7 ld   a,[$d736]\n\
            0000022 cb 77    bit  $06 a\n\
            0000024 c4 9e 03 cnz  $039e\n\
            0000027 fa c5 cf ld   a,[$cfc5]\n\
            000002a a7       and  a\n\
            000002b c2 b5 05 jnz  $05b5\n\
            000002e cd 4d 0f call $0f4d\n\
            0000031 06 07    ld   b,$07\n\
            0000033 21 88 69 ld   hl,$6988\n\
            0000036 cd d6 35 call $35d6\n\
    ");
}

#[test]
fn disassembler_rgbds_prints_not_implemented_instructions_correctly() {
    do_disassembler_test(
        RGBDSInstructionPrinterFactory,
        &[0xd3, 0xe3, 0xe4, 0xf4], "\
            0000000 d3       -   \n\
            0000001 e3       -   \n\
            0000002 e4       -   \n\
            0000003 f4       -   \n\
    ");
}
