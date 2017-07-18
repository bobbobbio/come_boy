// Copyright 2017 Remi Bernotavicius

use std::io::{self, Result};
use std::mem;

mod opcode_gen;

use emulator_common::{InstructionPrinter, InstructionPrinterFactory, Disassembler};
pub use intel_8080_emulator::opcodes::opcode_gen::{
    Intel8080InstructionSet, dispatch_intel8080_instruction, get_intel8080_instruction};

#[cfg(test)]
use emulator_common::do_disassembler_test;

pub struct Intel8080InstructionPrinter<'a> {
    stream_out: &'a mut io::Write,
    error: Result<()>
}

pub struct Intel8080InstructionPrinterFactory;

impl<'a> InstructionPrinterFactory<'a> for Intel8080InstructionPrinterFactory {
    type Output = Intel8080InstructionPrinter<'a>;
    fn new(&self,
        stream_out: &'a mut io::Write) -> Intel8080InstructionPrinter<'a>
    {
        return Intel8080InstructionPrinter {
            stream_out: stream_out,
            error: Ok(())
        };
    }
}

impl<'a> InstructionPrinter<'a> for Intel8080InstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8]) -> Result<()>
    {
        dispatch_intel8080_instruction(stream, self);
        return mem::replace(&mut self.error, Ok(()));
    }
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>
    {
        get_intel8080_instruction(stream)
    }
}

pub fn disassemble_8080_rom(rom: &[u8]) -> Result<()>
{
    let stdout = &mut io::stdout();
    let mut disassembler = Disassembler::new(rom, Intel8080InstructionPrinterFactory, stdout);
    disassembler.disassemble()
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
            0x35
        ], "\
            0000000 cd d6 35 CALL $35d6\n\
            0000003 21 2d d7 LXI  H #$d72d\n\
            0000006 cb       -   \n\
            0000007 ae       XRA  M\n\
            0000008 cd 29 24 CALL $2429\n\
            000000b 21 26 d1 LXI  H #$d126\n\
            000000e cb       -   \n\
            000000f ee cb    XRI  #$cb\n\
            0000011 f6 af    ORI  #$af\n\
            0000013 ea 6b cd JPE  $cd6b\n\
            0000016 cd af 20 CALL $20af\n\
            0000019 cd af 20 CALL $20af\n\
            000001c cd ba 20 CALL $20ba\n\
            000001f fa 36 d7 JM   $d736\n\
            0000022 cb       -   \n\
            0000023 77       MOV  M A\n\
            0000024 c4 9e 03 CNZ  $39e\n\
            0000027 fa c5 cf JM   $cfc5\n\
            000002a a7       ANA  A\n\
            000002b c2 b5 05 JNZ  $5b5\n\
            000002e cd 4d 0f CALL $f4d\n\
            0000031 06 07    MVI  B #$07\n\
            0000033 21 88 69 LXI  H #$6988\n\
            0000036 cd d6 35 CALL $35d6\n\
    ");
}
