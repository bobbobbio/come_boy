use emulator_common::Register8080;
use emulator_lr35902::opcodes::OpcodePrinterlr35902;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait InstructionSetlr35902 {
    fn derping(&mut self, register1: Register8080, data2: u8, data3: u16);
}

pub fn dispatch_lr35902_instruction<I: InstructionSetlr35902>(
    mut stream: &[u8],
    machine: &mut I)
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x13 => machine.derping(Register8080::A, read_u8(&mut stream).unwrap(), read_u16(&mut stream).unwrap()),

        _ => panic!("Unknown opcode {}", opcode)
   };
}

pub fn get_lr35902_instruction(stream: &[u8]) -> Option<Vec<u8>>
{
    let size = match stream[0] {
        0x13 => 4,

        _ => return None
    };
    let mut instruction = vec![];
    instruction.resize(size, 0);
    instruction.clone_from_slice(&stream[0..size]);
    return Some(instruction);
}

impl<'a> InstructionSetlr35902 for OpcodePrinterlr35902<'a> {
    fn derping(&mut self, register1: Register8080, data2: u8, data3: u16)
    {
        write!(self.stream_out, "{:04}", "DERP").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
        write!(self.stream_out, " #${:02x}", data2).unwrap();
        write!(self.stream_out, " #${:02x}", data3).unwrap();
    }
}