// Copyright 2018 Remi Bernotavicius

use crate::io::{self, Result};
use alloc::{format, string::String, vec, vec::Vec};
use core::ops::Range;
use core::str;

#[cfg(test)]
use crate::bytes::ReadBytesExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryDescription {
    Instruction,
    Data(u16),
    Ascii(u16),
}

pub trait MemoryAccessor {
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory_u16(&self, address: u16) -> u16 {
        if address == 0xFFFF {
            return self.read_memory(address) as u16;
        }

        (self.read_memory(address + 1) as u16) << 8 | (self.read_memory(address) as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory(address, value as u8);

        if address == 0xFFFF {
            return;
        }

        self.set_memory(address + 1, (value >> 8) as u8);
    }

    fn describe_address(&self, address: u16) -> MemoryDescription;
}

impl MemoryAccessor for &dyn MemoryAccessor {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        (*self).read_memory(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory(&mut self, _address: u16, _value: u8) {
        panic!()
    }

    fn describe_address(&self, address: u16) -> MemoryDescription {
        (*self).describe_address(address)
    }
}

pub struct MemoryStream<'a, M> {
    memory_accessor: &'a M,
    index: u16,
}

impl<'a, M> MemoryStream<'a, M> {
    pub fn new(memory_accessor: &'a M, index: u16) -> Self {
        Self {
            memory_accessor,
            index,
        }
    }
}

impl<'a, M: MemoryAccessor> io::Read for MemoryStream<'a, M> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_exact(buf)?;
        Ok(buf.len())
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        for (i, item) in buf.iter_mut().enumerate() {
            *item = self.memory_accessor.read_memory(self.index + (i as u16));
        }
        self.index += buf.len() as u16;
        Ok(())
    }
}

pub struct MemoryIterator<'a> {
    memory_accessor: &'a dyn MemoryAccessor,
    current_address: u16,
    ending_address: u16,
}

impl<'a> Iterator for MemoryIterator<'a> {
    type Item = u8;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn next(&mut self) -> Option<u8> {
        if self.current_address < self.ending_address {
            let mem = self.memory_accessor.read_memory(self.current_address);
            self.current_address += 1;
            Some(mem)
        } else {
            None
        }
    }
}

impl<'a> MemoryIterator<'a> {
    pub fn new(memory_accessor: &'a dyn MemoryAccessor, range: Range<u16>) -> MemoryIterator {
        assert!(range.start <= range.end);
        Self {
            memory_accessor,
            current_address: range.start,
            ending_address: range.end,
        }
    }
}

pub struct SimpleMemoryAccessor {
    pub memory: [u8; 0x10000],
}

impl Default for SimpleMemoryAccessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleMemoryAccessor {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }
}

impl MemoryAccessor for SimpleMemoryAccessor {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        MemoryDescription::Instruction
    }
}

/*   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

pub trait Instruction {
    fn size(&self) -> u8;
}

pub trait InstructionPrinter<'a> {
    type Instruction: Instruction;

    fn print_instruction(&mut self, instr: Self::Instruction, address: u16) -> Result<()>;

    fn get_instruction<R: io::Read>(&self, stream: R) -> Result<Option<Self::Instruction>>;
}

pub trait InstructionPrinterFactory<'a> {
    type Output: InstructionPrinter<'a>;
    fn create(&self, _: &'a mut dyn io::Write) -> Self::Output;
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
    memory_accessor: &'a dyn MemoryAccessor,
    opcode_printer_factory: PF,
    stream_out: &'a mut dyn io::Write,
}

impl<'a, PF: for<'b> InstructionPrinterFactory<'b> + Copy> Disassembler<'a, PF> {
    pub fn new(
        memory_accessor: &'a dyn MemoryAccessor,
        opcode_printer_factory: PF,
        stream_out: &'a mut dyn io::Write,
    ) -> Disassembler<'a, PF> {
        Self {
            index: 0,
            memory_accessor,
            opcode_printer_factory,
            stream_out,
        }
    }

    fn display_data(&mut self, data: &[u8], include_opcodes: bool, mut index: u16) -> Result<()> {
        let iter = &mut data.iter().peekable();
        while iter.peek().is_some() {
            if include_opcodes {
                write!(self.stream_out, "{index:07x}          ")?;
            }
            write!(self.stream_out, "{:04} ${:02X}", "db", iter.next().unwrap())?;
            index += 1;
            for d in iter.take(15) {
                write!(self.stream_out, ",${d:02X}")?;
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
                    write!(self.stream_out, "{start:07x}          ")?;
                }
                write!(self.stream_out, "{:04} \"{s}\"", "db")
            }
            Err(_) => self.display_data(&data, include_opcodes, start),
        }
    }

    pub fn disassemble_one(&mut self, include_opcodes: bool) -> Result<()> {
        match self.memory_accessor.describe_address(self.index) {
            MemoryDescription::Instruction => self.disassemble_one_instruction(include_opcodes),
            MemoryDescription::Data(len) => self.disassemble_data(len, include_opcodes),
            MemoryDescription::Ascii(len) => self.disassemble_ascii(len, include_opcodes),
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
            writeln!(self.stream_out, "{line}")?;
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
        let mut instr = vec![];
        let printed;
        {
            let mut opcode_printer = self.opcode_printer_factory.create(&mut printed_instr);
            let stream = MemoryStream::new(&self.memory_accessor, self.index);
            printed = match opcode_printer.get_instruction(stream)? {
                Some(res) => {
                    for i in 0..res.size() {
                        instr.push(self.memory_accessor.read_memory(self.index + i as u16));
                    }
                    opcode_printer.print_instruction(res, self.index)?;
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
                raw_assembly.push_str(format!("{code:02x} ").as_str());
            }

            write!(
                self.stream_out,
                "{:07x} {raw_assembly:9}{str_instr}",
                self.index
            )?;
        } else {
            write!(self.stream_out, "{str_instr}")?;
        }

        self.index += instr.len() as u16;

        Ok(())
    }

    pub fn disassemble(&mut self, range: Range<u16>, include_opcodes: bool) -> Result<()> {
        self.index = range.start;
        while self.index < range.end {
            self.disassemble_one(include_opcodes)?;
            writeln!(self.stream_out)?;
        }
        Ok(())
    }
}

#[cfg(test)]
struct TestInstructionPrinter<'a> {
    stream_out: &'a mut dyn io::Write,
}

#[cfg(test)]
enum TestInstruction {
    One,
    Two(u8),
    Three(u8, u8),
}

#[cfg(test)]
impl Instruction for TestInstruction {
    fn size(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two(_) => 2,
            Self::Three(_, _) => 3,
        }
    }
}

#[cfg(test)]
impl<'a> InstructionPrinter<'a> for TestInstructionPrinter<'a> {
    type Instruction = TestInstruction;

    fn print_instruction(&mut self, instr: TestInstruction, _address: u16) -> Result<()> {
        match instr {
            TestInstruction::One => write!(self.stream_out, "TEST1")?,
            TestInstruction::Two(v1) => write!(self.stream_out, "TEST2 {v1}")?,
            TestInstruction::Three(v1, v2) => write!(self.stream_out, "TEST3 {v1} {v2}")?,
        };
        Ok(())
    }

    fn get_instruction<R: io::Read>(&self, mut stream: R) -> Result<Option<TestInstruction>> {
        Ok(match stream.read_u8()? {
            0x1 => Some(TestInstruction::One),
            0x2 => Some(TestInstruction::Two(stream.read_u8()?)),
            0x3 => Some(TestInstruction::Three(stream.read_u8()?, stream.read_u8()?)),
            _ => None,
        })
    }
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct TestInstructionPrinterFactory;

#[cfg(test)]
impl<'a> InstructionPrinterFactory<'a> for TestInstructionPrinterFactory {
    type Output = TestInstructionPrinter<'a>;
    fn create(&self, stream_out: &'a mut dyn io::Write) -> TestInstructionPrinter<'a> {
        TestInstructionPrinter { stream_out }
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
         0000001 02 00    TEST2 0\n\
         0000003 03 00 00 TEST3 0 0\n\
         ",
    );
}

#[test]
fn disassembler_test_instruction_arguments_are_printed() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x3, 0xff, 0xfe, 0x3, 0xfd, 0xfc],
        "\
         0000000 03 ff fe TEST3 255 254\n\
         0000003 03 fd fc TEST3 253 252\n\
         ",
    );
}
