// Copyright 2017 Remi Bernotavicius

use crate::io::{self, Result};
use core::mem;

use crate::emulator_common::disassembler::{
    Disassembler, Instruction, InstructionPrinter, InstructionPrinterFactory, SimpleMemoryAccessor,
};
pub use crate::intel_8080_emulator::opcodes::opcode_gen::{
    Intel8080Instruction, Intel8080InstructionSet,
};

#[cfg(test)]
use crate::emulator_common::disassembler::do_disassembler_test;

mod opcode_gen;

pub struct Intel8080InstructionPrinter<'a> {
    stream_out: &'a mut dyn io::Write,
    error: Result<()>,
}

#[derive(Copy, Clone)]
pub struct Intel8080InstructionPrinterFactory;

impl<'a> InstructionPrinterFactory<'a> for Intel8080InstructionPrinterFactory {
    type Output = Intel8080InstructionPrinter<'a>;
    fn new(&self, stream_out: &'a mut dyn io::Write) -> Intel8080InstructionPrinter<'a> {
        Intel8080InstructionPrinter {
            stream_out,
            error: Ok(()),
        }
    }
}

impl Instruction for Intel8080Instruction {
    fn size(&self) -> u8 {
        Intel8080Instruction::size(self)
    }
}

impl<'a> InstructionPrinter<'a> for Intel8080InstructionPrinter<'a> {
    type Instruction = Intel8080Instruction;

    fn print_instruction(&mut self, instr: Intel8080Instruction, _address: u16) -> Result<()> {
        instr.dispatch(self);
        mem::replace(&mut self.error, Ok(()))
    }
    fn get_instruction<R: io::Read>(&self, stream: R) -> Result<Option<Intel8080Instruction>> {
        Intel8080Instruction::from_reader(stream)
    }
}

pub fn disassemble_8080_rom(
    rom: &[u8],
    include_opcodes: bool,
    mut output: impl io::Write,
) -> Result<()> {
    let mut ma = SimpleMemoryAccessor::new();
    ma.memory[0..rom.len()].clone_from_slice(rom);
    let mut disassembler = Disassembler::new(&ma, Intel8080InstructionPrinterFactory, &mut output);
    disassembler.disassemble(0u16..rom.len() as u16, include_opcodes)
}

#[test]
fn disassembler_8080_test() {
    do_disassembler_test(
        Intel8080InstructionPrinterFactory,
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
         0000006 cb       -\n\
         0000007 ae       XRA  M\n\
         0000008 cd 29 24 CALL $2429\n\
         000000b 21 26 d1 LXI  H #$d126\n\
         000000e cb       -\n\
         000000f ee cb    XRI  #$cb\n\
         0000011 f6 af    ORI  #$af\n\
         0000013 ea 6b cd JPE  $cd6b\n\
         0000016 cd af 20 CALL $20af\n\
         0000019 cd af 20 CALL $20af\n\
         000001c cd ba 20 CALL $20ba\n\
         000001f fa 36 d7 JM   $d736\n\
         0000022 cb       -\n\
         0000023 77       MOV  M A\n\
         0000024 c4 9e 03 CNZ  $39e\n\
         0000027 fa c5 cf JM   $cfc5\n\
         000002a a7       ANA  A\n\
         000002b c2 b5 05 JNZ  $5b5\n\
         000002e cd 4d 0f CALL $f4d\n\
         0000031 06 07    MVI  B #$07\n\
         0000033 21 88 69 LXI  H #$6988\n\
         0000036 cd d6 35 CALL $35d6\n\
         ",
    );
}
