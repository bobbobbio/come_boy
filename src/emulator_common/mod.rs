use std::io::Result;
use std::io;
use std::str;

/*
 *  ____            _     _            ___   ___   ___   ___
 * |  _ \ ___  __ _(_)___| |_ ___ _ __( _ ) / _ \ ( _ ) / _ \
 * | |_) / _ \/ _` | / __| __/ _ \ '__/ _ \| | | |/ _ \| | | |
 * |  _ <  __/ (_| | \__ \ ||  __/ | | (_) | |_| | (_) | |_| |
 * |_| \_\___|\__, |_|___/\__\___|_|  \___/ \___/ \___/ \___/
 *            |___/
 */

#[derive(Debug,Clone,Copy)]
pub enum Register8080 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 6,
    FLAGS = 7, // Conatins all of the condition bits.
    SP = 8,    // Stack Pointer (2 bytes)
    PSW = 10,  // Special fake register called 'Program Status Word'.
               // It refers to register pair, A and FLAGS.
    M = 11,    // Special fake register called 'Memory'.  Represents
               // the data stored at address contained in HL.
    Count = 12,
}

/*
 *   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

pub trait OpcodePrinter<'a> {
    fn print_opcode(&mut self, stream: &[u8]) -> Result<()>;
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>;
}

pub trait OpcodePrinterFactory<'a> {
    type Output: OpcodePrinter<'a>;
    fn new(&self, &'a mut io::Write) -> Self::Output;
}

/*
 *  ____  _                                  _     _
 * |  _ \(_)___  __ _ ___ ___  ___ _ __ ___ | |__ | | ___ _ __
 * | | | | / __|/ _` / __/ __|/ _ \ '_ ` _ \| '_ \| |/ _ \ '__|
 * | |_| | \__ \ (_| \__ \__ \  __/ | | | | | |_) | |  __/ |
 * |____/|_|___/\__,_|___/___/\___|_| |_| |_|_.__/|_|\___|_|
 *
 */

pub struct Disassembler<'a, PF: for<'b> OpcodePrinterFactory<'b>> {
    index: u64,
    rom: &'a [u8],
    opcode_printer_factory: PF,
    stream_out: &'a mut io::Write
}

impl<'a, PF: for<'b> OpcodePrinterFactory<'b>> Disassembler<'a, PF> {
    pub fn new(
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
    pub fn disassemble(&mut self) -> Result<()>
    {
        while (self.index as usize) < self.rom.len() {
            let mut printed_instr: Vec<u8> = vec![];
            let instr: Vec<u8>;
            let printed;
            {
                let mut opcode_printer = self.opcode_printer_factory.new(&mut printed_instr);
                printed = match opcode_printer.get_instruction(&self.rom[self.index as usize..]) {
                    Some(res) => {
                        try!(opcode_printer.print_opcode(&res));
                        instr = res;
                        true
                    },
                    None => {
                        instr = vec![self.rom[self.index as usize]];
                        false
                    }
                };
            }

            let str_instr = match printed {
                true => str::from_utf8(&printed_instr).unwrap(),
                false => "-   "
            };

            let mut raw_assembly = String::new();
            for code in &instr {
                raw_assembly.push_str(format!("{:02x} ", code).as_str());
            }

            try!(write!(self.stream_out, "{:07x} {:9}{}\n", self.index, raw_assembly, str_instr));

            self.index += instr.len() as u64;
        }
        Ok(())
    }
}

#[cfg(test)]
struct TestOpcodePrinter<'a> {
    stream_out: &'a mut io::Write
}

#[cfg(test)]
impl<'a> OpcodePrinter<'a> for TestOpcodePrinter<'a> {
    fn print_opcode(&mut self, stream: &[u8]) -> Result<()>
    {
        match stream[0] {
            0x1 => write!(self.stream_out, "TEST1").unwrap(),
            0x2 => write!(self.stream_out, "TEST2").unwrap(),
            0x3 => write!(self.stream_out, "TEST3").unwrap(),
            _ => panic!("Unkown Opcode {}", stream[0])
        };
        Ok(())
    }
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>
    {
        let size = match stream[0] {
            0x1 => 1,
            0x2 => 2,
            0x3 => 3,
            _ => return None
        };
        let mut instruction = vec![];
        instruction.resize(size, 0);
        instruction.clone_from_slice(&stream[0..size]);
        return Some(instruction);
    }
}

#[cfg(test)]
struct TestOpcodePrinterFactory;

#[cfg(test)]
impl<'a> OpcodePrinterFactory<'a> for TestOpcodePrinterFactory {
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
pub fn do_disassembler_test<PF: for<'b> OpcodePrinterFactory<'b>>(
    opcode_printer_factory: PF,
    test_rom: &[u8],
    expected_str: &str)
{
    let mut output = vec![];
    {
        let mut disassembler = Disassembler::new(test_rom, opcode_printer_factory, &mut output);
        disassembler.disassemble().unwrap();
    }
    assert_eq!(str::from_utf8(&output).unwrap(), expected_str);
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
