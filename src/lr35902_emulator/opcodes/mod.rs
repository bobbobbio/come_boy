// Copyright 2017 Remi Bernotavicius

use std::io::{self, Result};
use std::mem;

mod opcode_gen;

use emulator_common::{
    Disassembler, InstructionPrinter, InstructionPrinterFactory, MemoryAccessor,
    SimpleMemoryAccessor,
};
pub use lr35902_emulator::opcodes::opcode_gen::{
    dispatch_lr35902_instruction, get_lr35902_instruction, LR35902InstructionSet,
};

#[cfg(test)]
use emulator_common::do_disassembler_test;

pub struct LR35902InstructionPrinter<'a> {
    stream_out: &'a mut io::Write,
    error: Result<()>,
}

#[derive(Copy, Clone)]
pub struct LR35902InstructionPrinterFactory;

impl<'a> InstructionPrinterFactory<'a> for LR35902InstructionPrinterFactory {
    type Output = LR35902InstructionPrinter<'a>;
    fn new(&self, stream_out: &'a mut io::Write) -> LR35902InstructionPrinter<'a> {
        return LR35902InstructionPrinter {
            stream_out: stream_out,
            error: Ok(()),
        };
    }
}

impl<'a> InstructionPrinter<'a> for LR35902InstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8], _address: u16) -> Result<()> {
        dispatch_lr35902_instruction(stream, self);
        mem::replace(&mut self.error, Ok(()))
    }
    fn get_instruction<R: io::Read>(&self, stream: R) -> Option<Vec<u8>> {
        get_lr35902_instruction(stream)
    }
}

pub fn create_disassembler<'a>(
    ma: &'a MemoryAccessor,
    stream_out: &'a mut io::Write,
) -> Disassembler<'a, LR35902InstructionPrinterFactory> {
    Disassembler::new(ma, LR35902InstructionPrinterFactory, stream_out)
}

pub fn disassemble_lr35902_rom(rom: &[u8], include_opcodes: bool) -> Result<()> {
    let stdout = &mut io::stdout();
    let mut ma = SimpleMemoryAccessor::new();
    ma.memory[0..rom.len()].clone_from_slice(rom);
    let mut disassembler = create_disassembler(&ma, stdout);
    disassembler.disassemble(0u16..rom.len() as u16, include_opcodes)
}

#[test]
fn disassembler_lr35902_test() {
    do_disassembler_test(
        LR35902InstructionPrinterFactory,
        &[
            0xcd, 0xd6, 0x35, 0x21, 0x2d, 0xd7, 0xcb, 0xae, 0xcd, 0x29, 0x24, 0x21, 0x26, 0xd1,
            0xcb, 0xee, 0xcb, 0xf6, 0xaf, 0xea, 0x6b, 0xcd, 0xcd, 0xaf, 0x20, 0xcd, 0xaf, 0x20,
            0xcd, 0xba, 0x20, 0xfa, 0x36, 0xd7, 0xcb, 0x77, 0xc4, 0x9e, 0x03, 0xfa, 0xc5, 0xcf,
            0xa7, 0xc2, 0xb5, 0x05, 0xcd, 0x4d, 0x0f, 0x06, 0x07, 0x21, 0x88, 0x69, 0xcd, 0xd6,
            0x35,
        ],
        "\
         0000000 cd d6 35 CALL $35d6\n\
         0000003 21 2d d7 LXI  H #$d72d\n\
         0000006 cb ae    RES  5 M\n\
         0000008 cd 29 24 CALL $2429\n\
         000000b 21 26 d1 LXI  H #$d126\n\
         000000e cb ee    SET  5 M\n\
         0000010 cb f6    SET  6 M\n\
         0000012 af       XRA  A\n\
         0000013 ea 6b cd STA  $cd6b\n\
         0000016 cd af 20 CALL $20af\n\
         0000019 cd af 20 CALL $20af\n\
         000001c cd ba 20 CALL $20ba\n\
         000001f fa 36 d7 LDAD $d736\n\
         0000022 cb 77    BIT  6 A\n\
         0000024 c4 9e 03 CNZ  $39e\n\
         0000027 fa c5 cf LDAD $cfc5\n\
         000002a a7       ANA  A\n\
         000002b c2 b5 05 JNZ  $5b5\n\
         000002e cd 4d 0f CALL $f4d\n\
         0000031 06 07    MVI  B #$07\n\
         0000033 21 88 69 LXI  H #$6988\n\
         0000036 cd d6 35 CALL $35d6\n\
         ",
    );
}

#[test]
fn disassembler_lr35902_prints_not_implemented_instructions_correctly() {
    do_disassembler_test(
        LR35902InstructionPrinterFactory,
        &[0xd3, 0xe3, 0xe4, 0xf4],
        "\
         0000000 d3       -\n\
         0000001 e3       -\n\
         0000002 e4       -\n\
         0000003 f4       -\n\
         ",
    );
}
