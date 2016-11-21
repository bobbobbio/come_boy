use emulator_common::Register8080;
use emulator_lr35902::opcodes::OpcodePrinterlr35902;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait InstructionSetlr35902 {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Register8080);
    fn test(&mut self, data1: u8, data2: u16);
}

pub fn dispatch_lr35902_instruction<I: InstructionSetlr35902>(
    mut stream: &[u8],
    machine: &mut I)
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x01 => machine.test(read_u8(&mut stream).unwrap(), read_u16(&mut stream).unwrap()),
        0xCB => match (0xCB as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0xCB80 => machine.reset_bit(0 as u8, Register8080::B),
            0xCB81 => machine.reset_bit(0 as u8, Register8080::C),
            0xCB82 => machine.reset_bit(0 as u8, Register8080::D),
            0xCB83 => machine.reset_bit(0 as u8, Register8080::E),
            0xCB84 => machine.reset_bit(0 as u8, Register8080::H),
            0xCB85 => machine.reset_bit(0 as u8, Register8080::L),
            0xCB86 => machine.reset_bit(0 as u8, Register8080::M),
            0xCB87 => machine.reset_bit(0 as u8, Register8080::A),
            0xCB88 => machine.reset_bit(1 as u8, Register8080::B),
            0xCB89 => machine.reset_bit(1 as u8, Register8080::C),
            0xCB8A => machine.reset_bit(1 as u8, Register8080::D),
            0xCB8B => machine.reset_bit(1 as u8, Register8080::E),
            0xCB8C => machine.reset_bit(1 as u8, Register8080::H),
            0xCB8D => machine.reset_bit(1 as u8, Register8080::L),
            0xCB8E => machine.reset_bit(1 as u8, Register8080::M),
            0xCB8F => machine.reset_bit(1 as u8, Register8080::A),
            v => panic!("Unknown opcode {}", v)
        },
        v => panic!("Unknown opcode {}", v)
    };
}

pub fn get_lr35902_instruction(original_stream: &[u8]) -> Option<Vec<u8>>
{
    let mut stream = original_stream;
    let size = match read_u8(&mut stream).unwrap() {
        0x01 => 4,
        0xCB => match (0xCB as u16) << 8 |
            match read_u8(&mut stream) { Ok(x) => x, _ => return None } as u16{
            0xCB80 => 2,
            0xCB81 => 2,
            0xCB82 => 2,
            0xCB83 => 2,
            0xCB84 => 2,
            0xCB85 => 2,
            0xCB86 => 2,
            0xCB87 => 2,
            0xCB88 => 2,
            0xCB89 => 2,
            0xCB8A => 2,
            0xCB8B => 2,
            0xCB8C => 2,
            0xCB8D => 2,
            0xCB8E => 2,
            0xCB8F => 2,
            _ => return None
        },
        _ => return None
    };

    let mut instruction = vec![];
    instruction.resize(size, 0);
    instruction.clone_from_slice(&original_stream[0..size]);
    return Some(instruction);
}

impl<'a> InstructionSetlr35902 for OpcodePrinterlr35902<'a> {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Register8080)
    {
        write!(self.stream_out, "{:04}", "RES").unwrap();
        write!(self.stream_out, " {}", implicit_data1).unwrap();
        write!(self.stream_out, " {:?}", register2).unwrap();
    }
    fn test(&mut self, data1: u8, data2: u16)
    {
        write!(self.stream_out, "{:04}", "TEST").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
        write!(self.stream_out, " #${:02x}", data2).unwrap();
    }
}
