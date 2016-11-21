use std::io::{self, Result};

mod opcode_gen;

use emulator_common::{OpcodePrinter, OpcodePrinterFactory, Disassembler};
use emulator_lr35902::opcodes::opcode_gen::{
    dispatch_lr35902_instruction, get_lr35902_instruction};

/*
#[cfg(test)]
use emulator_common::do_disassembler_test;
*/

/*
 *   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

struct OpcodePrinterlr35902<'a> {
    stream_out: &'a mut io::Write
}

struct OpcodePrinterFactorylr35902;

impl<'a> OpcodePrinterFactory<'a> for OpcodePrinterFactorylr35902 {
    type Output = OpcodePrinterlr35902<'a>;
    fn new(&self,
        stream_out: &'a mut io::Write) -> OpcodePrinterlr35902<'a>
    {
        return OpcodePrinterlr35902 {
            stream_out: stream_out
        };
    }
}

impl<'a> OpcodePrinter<'a> for OpcodePrinterlr35902<'a> {
    fn print_opcode(&mut self, stream: &[u8])
    {
        dispatch_lr35902_instruction(stream, self)
    }
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>
    {
        get_lr35902_instruction(stream)
    }
}

pub fn disassemble_lr35902_rom(rom: &[u8]) -> Result<()>
{
    let stdout = &mut io::stdout();
    let mut disassembler = Disassembler::new(rom, OpcodePrinterFactorylr35902, stdout);
    disassembler.disassemble()
}

/*
#[test]
fn disassembler_lr35902_test() {
    do_disassembler_test(
        OpcodePrinterFactorylr35902,
        &vec![
            0xcd, 0xd6, 0x35, 0x21, 0x2d, 0xd7, 0xcb, 0xae, 0xcd, 0x29, 0x24, 0x21, 0x26, 0xd1,
            0xcb, 0xee, 0xcb, 0xf6, 0xaf, 0xea, 0x6b, 0xcd, 0xcd, 0xaf, 0x20, 0xcd, 0xaf, 0x20,
            0xcd, 0xba, 0x20, 0xfa, 0x36, 0xd7, 0xcb, 0x77, 0xc4, 0x9e, 0x03, 0xfa, 0xc5, 0xcf,
            0xa7, 0xc2, 0xb5, 0x05, 0xcd, 0x4d, 0x0f, 0x06, 0x07, 0x21, 0x88, 0x69, 0xcd, 0xd6,
            0x35
        ], "\
            0000000 cd d6 35 CALL $35d6\n\
            0000003 21 2d d7 LXI  H #$d72d\n\
            0000006 cb ae    RES  5 H \n\
            0000008 cd 29 24 CALL $2429\n\
            000000b 21 26 d1 LXI  H #$d126\n\
            000000e cb ee    SET  5 H\n\
            0000010 cb f6    SET  6 H\n\
            0000012 af       XOR  A\n\
            0000013 ea 6b cd LDM  $cd6b A\n\
            0000016 cd af 20 CALL $20af\n\
            0000019 cd af 20 CALL $20af\n\
            000001c cd ba 20 CALL $20ba\n\
            000001f fa 36 d7 LDD  A $d736\n\
            0000022 cb 77    BIT  6 A\n\
            0000024 c4 9e 03 CNZ  $39e\n\
            0000027 fa c5 cf LDD  A $cfc5\n\
            000002a a7       ANA  A\n\
            000002b c2 b5 05 JNZ  $5b5\n\
            000002e cd 4d 0f CALL $f4d\n\
            0000031 06 07    MVI  B #$07\n\
            0000033 21 88 69 LXI  H #$6988\n\
            0000036 cd d6 35 CALL $35d6\n\
    ");
}
*/
