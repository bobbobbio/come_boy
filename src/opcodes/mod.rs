use std::io::{self, Result};
use std::str;

mod opcode_gen;

use opcodes::opcode_gen::OpcodePrinter;

struct Disassembler<'a> {
    index: u64,
    rom: &'a [u8],
    stream_out: &'a mut io::Write
}

impl<'a> Disassembler<'a> {
    fn new(rom: &'a [u8], stream_out: &'a mut io::Write) -> Disassembler<'a>
    {
        return Disassembler {
            index: 0,
            rom: rom,
            stream_out: stream_out
        }
    }
    fn disassemble<PF: for<'b> opcode_gen::OpcodePrinterFactory<'b>>(
        &mut self,
        opcode_printer_factory: PF) -> Result<()>
    {
        while (self.index as usize) < self.rom.len() {

            let mut formatted_op_buf: Vec<u8> = vec![];
            let size: u8;
            {
                let mut d = opcode_printer_factory.new(&mut formatted_op_buf);
                size = try!(d.print_opcode(&self.rom[self.index as usize..]));
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
    let mut disassembler = Disassembler::new(rom, stdout);
    disassembler.disassemble(opcode_gen::OpcodePrinterFactory8080)
}
