// Copyright 2018 Remi Bernotavicius

use std::io::{self, Result};
use std::ops::Range;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryDescription {
    Instruction,
    Data(u16),
    ASCII(u16),
}

pub trait MemoryAccessor {
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);

    fn read_memory_u16(&self, address: u16) -> u16 {
        if address == 0xFFFF {
            return self.read_memory(address) as u16;
        }

        return (self.read_memory(address + 1) as u16) << 8 | (self.read_memory(address) as u16);
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory(address, value as u8);

        if address == 0xFFFF {
            return;
        }

        self.set_memory(address + 1, (value >> 8) as u8);
    }

    fn describe_address(&self, address: u16) -> MemoryDescription;
}

pub struct MemoryStream<'a> {
    memory_accessor: &'a MemoryAccessor,
    index: u16,
}

impl<'a> MemoryStream<'a> {
    pub fn new(memory_accessor: &'a MemoryAccessor, index: u16) -> MemoryStream<'a> {
        return MemoryStream {
            memory_accessor: memory_accessor,
            index: index,
        };
    }
}

impl<'a> io::Read for MemoryStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            buf[i] = self.memory_accessor.read_memory(self.index + (i as u16));
        }
        self.index += buf.len() as u16;

        Ok(buf.len())
    }
}

pub struct MemoryIterator<'a> {
    memory_accessor: &'a MemoryAccessor,
    current_address: u16,
    ending_address: u16,
}

impl<'a> Iterator for MemoryIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current_address < self.ending_address {
            let mem = self.memory_accessor.read_memory(self.current_address);
            self.current_address += 1;
            return Some(mem);
        } else {
            return None;
        }
    }
}

impl<'a> MemoryIterator<'a> {
    pub fn new(memory_accessor: &'a MemoryAccessor, range: Range<u16>) -> MemoryIterator {
        assert!(range.start < range.end);
        return MemoryIterator {
            memory_accessor: memory_accessor,
            current_address: range.start,
            ending_address: range.end,
        };
    }
}

pub struct SimpleMemoryAccessor {
    pub memory: [u8; 0x10000],
}

impl SimpleMemoryAccessor {
    pub fn new() -> SimpleMemoryAccessor {
        return SimpleMemoryAccessor {
            memory: [0u8; 0x10000],
        };
    }
}

impl MemoryAccessor for SimpleMemoryAccessor {
    fn read_memory(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        return MemoryDescription::Instruction;
    }
}

/*   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

pub trait InstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8], address: u16) -> Result<()>;
    fn get_instruction<R: io::Read>(&self, stream: R) -> Option<Vec<u8>>;
}

pub trait InstructionPrinterFactory<'a> {
    type Output: InstructionPrinter<'a>;
    fn new(&self, &'a mut io::Write) -> Self::Output;
}

/*  ____  _                                  _     _
 * |  _ \(_)___  __ _ ___ ___  ___ _ __ ___ | |__ | | ___ _ __
 * | | | | / __|/ _` / __/ __|/ _ \ '_ ` _ \| '_ \| |/ _ \ '__|
 * | |_| | \__ \ (_| \__ \__ \  __/ | | | | | |_) | |  __/ |
 * |____/|_|___/\__,_|___/___/\___|_| |_| |_|_.__/|_|\___|_|
 *
 */

pub struct Disassembler<'a, PF: for<'b> InstructionPrinterFactory<'b> + Copy> {
    pub index: u16,
    memory_accessor: &'a MemoryAccessor,
    opcode_printer_factory: PF,
    stream_out: &'a mut io::Write,
}

impl<'a, PF: for<'b> InstructionPrinterFactory<'b> + Copy> Disassembler<'a, PF> {
    pub fn new(
        memory_accessor: &'a MemoryAccessor,
        opcode_printer_factory: PF,
        stream_out: &'a mut io::Write,
    ) -> Disassembler<'a, PF> {
        return Disassembler {
            index: 0,
            memory_accessor: memory_accessor,
            opcode_printer_factory: opcode_printer_factory,
            stream_out: stream_out,
        };
    }

    fn display_data(&mut self, data: &[u8], include_opcodes: bool, mut index: u16) -> Result<()> {
        let iter = &mut data.iter().peekable();
        while iter.peek().is_some() {
            if include_opcodes {
                write!(self.stream_out, "{:07x}          ", index)?;
            }
            write!(self.stream_out, "{:04} ${:02X}", "db", iter.next().unwrap())?;
            index += 1;
            for d in iter.take(15) {
                write!(self.stream_out, ",${:02X}", d)?;
                index += 1;
            }
            if iter.peek().is_some() {
                writeln!(self.stream_out)?;
            }
        }
        Ok(())
    }

    fn disassemble_data(&mut self, len: u16, include_opcodes: bool) -> Result<()> {
        let start = self.index;
        let mut data = vec![];
        for _ in 0..len {
            data.push(self.memory_accessor.read_memory(self.index));
            self.index += 1;
        }

        self.display_data(&data, include_opcodes, start)
    }

    fn disassemble_ascii(&mut self, len: u16, include_opcodes: bool) -> Result<()> {
        let start = self.index;
        let mut data = vec![];
        for _ in 0..len {
            data.push(self.memory_accessor.read_memory(self.index));
            self.index += 1;
        }

        match str::from_utf8(&data) {
            Ok(s) => {
                if include_opcodes {
                    write!(self.stream_out, "{:07x}          ", start)?;
                }
                write!(self.stream_out, "{:04} \"{}\"", "db", s)
            }
            Err(_) => self.display_data(&data, include_opcodes, start),
        }
    }

    pub fn disassemble_one(&mut self, include_opcodes: bool) -> Result<()> {
        match self.memory_accessor.describe_address(self.index) {
            MemoryDescription::Instruction => self.disassemble_one_instruction(include_opcodes),
            MemoryDescription::Data(len) => self.disassemble_data(len, include_opcodes),
            MemoryDescription::ASCII(len) => self.disassemble_ascii(len, include_opcodes),
        }
    }

    pub fn disassemble_multiple(&mut self) -> Result<()> {
        let context = 10;
        let mut previous = vec![];
        let current = self.index;

        let start = current.saturating_sub((context + 5) * 3);

        let mut dis = Disassembler::new(
            self.memory_accessor,
            self.opcode_printer_factory,
            &mut previous,
        );
        dis.disassemble(start..current, true).unwrap();

        let lines = str::from_utf8(&previous).unwrap();
        let skip = lines.lines().count().saturating_sub(context as usize);

        for line in lines.lines().skip(skip) {
            writeln!(self.stream_out, "{}", line)?;
        }

        self.disassemble_one(true)?;
        writeln!(self.stream_out, " <---")?;

        for _ in 0..(context - 1) {
            self.disassemble_one(true)?;
            writeln!(self.stream_out)?;
        }
        self.disassemble_one(true)?;

        self.index = current;

        Ok(())
    }

    fn disassemble_one_instruction(&mut self, include_opcodes: bool) -> Result<()> {
        let mut printed_instr: Vec<u8> = vec![];
        let instr: Vec<u8>;
        let printed;
        {
            let mut opcode_printer = self.opcode_printer_factory.new(&mut printed_instr);
            let stream = MemoryStream::new(self.memory_accessor, self.index);
            printed = match opcode_printer.get_instruction(stream) {
                Some(res) => {
                    opcode_printer.print_instruction(&res, self.index)?;
                    instr = res;
                    true
                }
                None => {
                    instr = vec![self.memory_accessor.read_memory(self.index)];
                    false
                }
            };
        }

        let str_instr = match printed {
            true => str::from_utf8(&printed_instr).unwrap(),
            false => "-",
        };

        if include_opcodes {
            let mut raw_assembly = String::new();
            for code in &instr {
                raw_assembly.push_str(format!("{:02x} ", code).as_str());
            }

            write!(
                self.stream_out,
                "{:07x} {:9}{}",
                self.index, raw_assembly, str_instr
            )?;
        } else {
            write!(self.stream_out, "{}", str_instr)?;
        }

        self.index += instr.len() as u16;

        Ok(())
    }

    pub fn disassemble(&mut self, range: Range<u16>, include_opcodes: bool) -> Result<()> {
        self.index = range.start;
        while self.index < range.end {
            self.disassemble_one(include_opcodes)?;
            writeln!(self.stream_out, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
struct TestInstructionPrinter<'a> {
    stream_out: &'a mut io::Write,
}

#[cfg(test)]
impl<'a> InstructionPrinter<'a> for TestInstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8], _address: u16) -> Result<()> {
        match stream[0] {
            0x1 => write!(self.stream_out, "TEST1").unwrap(),
            0x2 => write!(self.stream_out, "TEST2").unwrap(),
            0x3 => write!(self.stream_out, "TEST3").unwrap(),
            _ => panic!("Unknown Opcode {}", stream[0]),
        };
        Ok(())
    }
    fn get_instruction<R: io::Read>(&self, mut stream: R) -> Option<Vec<u8>> {
        let mut instr = vec![0];
        stream.read(&mut instr).unwrap();
        let size = match instr[0] {
            0x1 => 1,
            0x2 => 2,
            0x3 => 3,
            _ => return None,
        };
        instr.resize(size, 0);
        stream.read(&mut instr[1..]).unwrap();
        return Some(instr);
    }
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct TestInstructionPrinterFactory;

#[cfg(test)]
impl<'a> InstructionPrinterFactory<'a> for TestInstructionPrinterFactory {
    type Output = TestInstructionPrinter<'a>;
    fn new(&self, stream_out: &'a mut io::Write) -> TestInstructionPrinter<'a> {
        return TestInstructionPrinter {
            stream_out: stream_out,
        };
    }
}

#[cfg(test)]
pub fn do_disassembler_test<PF: for<'b> InstructionPrinterFactory<'b> + Copy>(
    opcode_printer_factory: PF,
    test_rom: &[u8],
    expected_str: &str,
) {
    let mut output = vec![];
    {
        let mut ma = SimpleMemoryAccessor::new();
        ma.memory[0..test_rom.len()].clone_from_slice(test_rom);
        let mut disassembler = Disassembler::new(&ma, opcode_printer_factory, &mut output);
        disassembler
            .disassemble(0u16..test_rom.len() as u16, true)
            .unwrap();
    }
    assert_eq!(str::from_utf8(&output).unwrap(), expected_str);
}

#[test]
fn disassembler_test_single_byte_instructions() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x1, 0x1, 0x1],
        "\
         0000000 01       TEST1\n\
         0000001 01       TEST1\n\
         0000002 01       TEST1\n\
         ",
    );
}

#[test]
fn disassembler_test_multiple_byte_instructions() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x1, 0x2, 0x0, 0x3, 0x0, 0x0],
        "\
         0000000 01       TEST1\n\
         0000001 02 00    TEST2\n\
         0000003 03 00 00 TEST3\n\
         ",
    );
}

#[test]
fn disassembler_test_instruction_arguments_are_printed() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x3, 0xff, 0xfe, 0x3, 0xfd, 0xfc],
        "\
         0000000 03 ff fe TEST3\n\
         0000003 03 fd fc TEST3\n\
         ",
    );
}
