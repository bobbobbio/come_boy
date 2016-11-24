use std::io::{self, Result};
use std::mem;

mod opcode_gen;

use emulator_common::{OpcodePrinter, OpcodePrinterFactory, Disassembler};
pub use emulator_lr35902::opcodes::opcode_gen::{
    dispatch_lr35902_instruction, get_lr35902_instruction, InstructionSetLR35902};
use emulator_8080::{get_8080_instruction, OpcodePrinterFactory8080};

#[cfg(test)]
use emulator_common::do_disassembler_test;

/*
 *   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

struct OpcodePrinterLR35902<'a> {
    stream_out: &'a mut io::Write,
    error: Result<()>
}

struct OpcodePrinterFactoryLR35902;

impl<'a> OpcodePrinterFactory<'a> for OpcodePrinterFactoryLR35902 {
    type Output = OpcodePrinterLR35902<'a>;
    fn new(&self, stream_out: &'a mut io::Write) -> OpcodePrinterLR35902<'a>
    {
        return OpcodePrinterLR35902 {
            stream_out: stream_out,
            error: Ok(())
        };
    }
}

impl<'a> OpcodePrinter<'a> for OpcodePrinterLR35902<'a> {
    fn print_opcode(&mut self, stream: &[u8]) -> Result<()>
    {
        match get_lr35902_instruction(stream) {
            Some(_) => {
                dispatch_lr35902_instruction(stream, self);
                mem::replace(&mut self.error, Ok(()))
            },
            None => {
                let mut op = OpcodePrinterFactory8080.new(self.stream_out);
                op.print_opcode(stream)
            }
        }
    }
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>
    {
        match get_lr35902_instruction(stream) {
            Some(x) => Some(x),
            None => get_8080_instruction(stream)
        }
    }
}

pub fn disassemble_lr35902_rom(rom: &[u8]) -> Result<()>
{
    let stdout = &mut io::stdout();
    let mut disassembler = Disassembler::new(rom, OpcodePrinterFactoryLR35902, stdout);
    disassembler.disassemble()
}

#[test]
fn disassembler_lr35902_test() {
    do_disassembler_test(
        OpcodePrinterFactoryLR35902,
        &vec![
            0xcd, 0xd6, 0x35, 0x21, 0x2d, 0xd7, 0xcb, 0xae, 0xcd, 0x29, 0x24, 0x21, 0x26, 0xd1,
            0xcb, 0xee, 0xcb, 0xf6, 0xaf, 0xea, 0x6b, 0xcd, 0xcd, 0xaf, 0x20, 0xcd, 0xaf, 0x20,
            0xcd, 0xba, 0x20, 0xfa, 0x36, 0xd7, 0xcb, 0x77, 0xc4, 0x9e, 0x03, 0xfa, 0xc5, 0xcf,
            0xa7, 0xc2, 0xb5, 0x05, 0xcd, 0x4d, 0x0f, 0x06, 0x07, 0x21, 0x88, 0x69, 0xcd, 0xd6,
            0x35
        ], "\
            0000000 cd d6 35 CALL $35d6\n\
            0000003 21 2d d7 LXI  H #$d72d\n\
            0000006 cb ae    RES  5 M\n\
            0000008 cd 29 24 CALL $2429\n\
            000000b 21 26 d1 LXI  H #$d126\n\
            000000e cb ee    SET  5 M\n\
            0000010 cb f6    SET  6 M\n\
            0000012 af       XRA  A\n\
            0000013 ea 6b cd LDMD $cd6b\n\
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
    ");
}
