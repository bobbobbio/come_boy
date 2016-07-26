use std::io::{self, Result};
use std::str;

pub mod opcode_gen;

use emulator8080::opcodes::opcode_gen::{OpcodePrinter, OpcodePrinterFactory8080};

struct Disassembler<'a, PF: for<'b> opcode_gen::OpcodePrinterFactory<'b>> {
    index: u64,
    rom: &'a [u8],
    opcode_printer_factory: PF,
    stream_out: &'a mut io::Write
}

impl<'a, PF: for<'b> opcode_gen::OpcodePrinterFactory<'b>> Disassembler<'a, PF> {
    fn new(
        rom: &'a [u8],
        opcode_printer_factory: PF,
        stream_out: &'a mut io::Write) -> Disassembler<'a, PF>
    {
        return Disassembler {
            index: 0,
            rom: rom,
            opcode_printer_factory: opcode_printer_factory,
            stream_out: stream_out
        }
    }
    fn disassemble(&mut self) -> Result<()>
    {
        while (self.index as usize) < self.rom.len() {
            let mut formatted_op_buf: Vec<u8> = vec![];
            let size: u8;
            {
                let mut d = self.opcode_printer_factory.new(&mut formatted_op_buf);
                size = d.print_opcode(&self.rom[self.index as usize..]);
            }
            let formatted_opcode = str::from_utf8(&formatted_op_buf).ok().expect("");

            let mut raw_assembly = String::new();
            for code in &self.rom[self.index as usize .. (self.index + size as u64) as usize] {
                raw_assembly.push_str(format!("{:02x} ", code).as_str());
            }

            try!(write!(self.stream_out, "{:07x} {:9}{}\n",
                self.index, raw_assembly, formatted_opcode));
            self.index += size as u64;
        }
        Ok(())
    }
}

pub fn disassemble(rom: &[u8]) -> Result<()>
{
    let stdout = &mut io::stdout();
    let mut disassembler = Disassembler::new(rom, OpcodePrinterFactory8080, stdout);
    disassembler.disassemble()
}

#[cfg(test)]
struct TestOpcodePrinter<'a> {
    stream_out: &'a mut io::Write
}

#[cfg(test)]
impl<'a> OpcodePrinter<'a> for TestOpcodePrinter<'a> {
    fn print_opcode(
        &mut self,
        stream: &[u8]) -> u8
    {
        let size;
        match stream[0] {
            0x1 => {
                write!(self.stream_out, "TEST1").ok().expect(""); size = 1;
            }
            0x2 => {
                write!(self.stream_out, "TEST2").ok().expect(""); size = 2;
            }
            0x3 => {
                write!(self.stream_out, "TEST3").ok().expect(""); size = 3;
            }
            _ => panic!("Unknown opcode")
        };
        size
    }
}

#[cfg(test)]
struct TestOpcodePrinterFactory;

#[cfg(test)]
impl<'a> opcode_gen::OpcodePrinterFactory<'a> for TestOpcodePrinterFactory {
    type Output = TestOpcodePrinter<'a>;
    fn new(&self,
        stream_out: &'a mut io::Write) -> TestOpcodePrinter<'a>
    {
        return TestOpcodePrinter {
            stream_out: stream_out
        };
    }
}

#[cfg(test)]
fn do_disassembler_test<PF: for<'b> opcode_gen::OpcodePrinterFactory<'b>>(
    opcode_printer_factory: PF,
    test_rom: &[u8],
    expected_str: &str)
{
    let mut output = vec![];
    {
        let mut disassembler = Disassembler::new(test_rom, opcode_printer_factory, &mut output);
        disassembler.disassemble().ok().expect("");
    }
    assert_eq!(str::from_utf8(&output).ok().expect(""), expected_str);
}

#[test]
fn disassembler_test_single_byte_instructions() {
    do_disassembler_test(
        TestOpcodePrinterFactory,
        &vec![0x1, 0x1, 0x1], "\
        0000000 01       TEST1\n\
        0000001 01       TEST1\n\
        0000002 01       TEST1\n\
    ");
}

#[test]
fn disassembler_test_multiple_byte_instructions() {
    do_disassembler_test(
        TestOpcodePrinterFactory,
        &vec![0x1, 0x2, 0x0, 0x3, 0x0, 0x0], "\
        0000000 01       TEST1\n\
        0000001 02 00    TEST2\n\
        0000003 03 00 00 TEST3\n\
    ");
}

#[test]
fn disassembler_test_instruction_arguments_are_printed() {
    do_disassembler_test(
        TestOpcodePrinterFactory,
        &vec![0x3, 0xff, 0xfe, 0x3, 0xfd, 0xfc], "\
        0000000 03 ff fe TEST3\n\
        0000003 03 fd fc TEST3\n\
    ");
}

#[test]
fn disassembler_8080_test() {
    do_disassembler_test(
        OpcodePrinterFactory8080,
        &vec![
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
