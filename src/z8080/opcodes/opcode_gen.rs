
use std::io::{self, Result};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcodes/opcode_gen.py
 */

fn read_u16<T: io::Read>(
    mut stream: T) -> Result<u16>
{
    let mut narg : u16;
    let mut arg_buffer = [0; 1];
    try!(stream.read_exact(&mut arg_buffer));
    narg = arg_buffer[0] as u16;
    try!(stream.read_exact(&mut arg_buffer));
    narg |= (arg_buffer[0] as u16) << 8;
    Ok(narg)
}

fn read_u8<T: io::Read>(
    mut stream: T) -> Result<u8>
{
    let mut arg_buffer = [0; 1];
    try!(stream.read_exact(&mut arg_buffer));
    Ok(arg_buffer[0])
}

#[derive(Debug,Clone,Copy)]
pub enum Register8080 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 6,
    PSW = 7, // Program Status Word
    PC = 8, // Program Counter
    SP = 9, // Stack Pointer
    M = 10, // Special fake register called 'Memory'.  Represents
            // the data stored at address contained in HL.
    Count = 11,
}

pub trait InstructionSet8080 {
    fn subtract_from_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn return_if_not_zero(&mut self) -> Result<()>;
    fn add_immediate_to_accumulator(&mut self, data1: u8) -> Result<()>;
    fn pop_data_off_stack(&mut self, register1: Register8080) -> Result<()>;
    fn add_to_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn jump_if_parity_even(&mut self, address1: u16) -> Result<()>;
    fn call_if_zero(&mut self, address1: u16) -> Result<()>;
    fn double_add(&mut self, register1: Register8080) -> Result<()>;
    fn or_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>;
    fn call_if_carry(&mut self, address1: u16) -> Result<()>;
    fn jump(&mut self, address1: u16) -> Result<()>;
    fn logical_or(&mut self, register1: Register8080) -> Result<()>;
    fn rim(&mut self) -> Result<()>;
    fn call_if_parity_even(&mut self, address1: u16) -> Result<()>;
    fn jump_if_positive(&mut self, address1: u16) -> Result<()>;
    fn move_data(&mut self, register1: Register8080, register2: Register8080) -> Result<()>;
    fn no_instruction(&mut self) -> Result<()>;
    fn disable_interrupts(&mut self) -> Result<()>;
    fn set_carry(&mut self) -> Result<()>;
    fn compare_with_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn call_if_not_zero(&mut self, address1: u16) -> Result<()>;
    fn call_if_parity_odd(&mut self, address1: u16) -> Result<()>;
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) -> Result<()>;
    fn rotate_accumulator_left_through_carry(&mut self) -> Result<()>;
    fn load_sp_from_h_and_l(&mut self) -> Result<()>;
    fn logical_and_with_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn load_h_and_l_direct(&mut self, address1: u16) -> Result<()>;
    fn add_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>;
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>;
    fn call(&mut self, address1: u16) -> Result<()>;
    fn enable_interrupts(&mut self) -> Result<()>;
    fn load_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn input(&mut self, data1: u8) -> Result<()>;
    fn jump_if_parity_odd(&mut self, address1: u16) -> Result<()>;
    fn increment_register_pair(&mut self, register1: Register8080) -> Result<()>;
    fn logical_exclusive_or(&mut self, register1: Register8080) -> Result<()>;
    fn exchange_registers(&mut self) -> Result<()>;
    fn rotate_accumulator_right(&mut self) -> Result<()>;
    fn call_if_no_carry(&mut self, address1: u16) -> Result<()>;
    fn return_if_parity_even(&mut self) -> Result<()>;
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) -> Result<()>;
    fn halt(&mut self) -> Result<()>;
    fn call_if_plus(&mut self, address1: u16) -> Result<()>;
    fn increment_register_or_memory(&mut self, register1: Register8080) -> Result<()>;
    fn compare_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>;
    fn load_program_counter(&mut self) -> Result<()>;
    fn return_if_minus(&mut self) -> Result<()>;
    fn jump_if_carry(&mut self, address1: u16) -> Result<()>;
    fn call_if_minus(&mut self, address1: u16) -> Result<()>;
    fn decimal_adjust_accumulator(&mut self) -> Result<()>;
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16) -> Result<()>;
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8) -> Result<()>;
    fn return_if_plus(&mut self) -> Result<()>;
    fn restart(&mut self, implicit_data1: u8) -> Result<()>;
    fn return_if_carry(&mut self) -> Result<()>;
    fn store_accumulator_direct(&mut self, address1: u16) -> Result<()>;
    fn jump_if_not_zero(&mut self, address1: u16) -> Result<()>;
    fn jump_if_minus(&mut self, address1: u16) -> Result<()>;
    fn decrement_register_or_memory(&mut self, register1: Register8080) -> Result<()>;
    fn output(&mut self, data1: u8) -> Result<()>;
    fn store_accumulator(&mut self, register1: Register8080) -> Result<()>;
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080) -> Result<()>;
    fn jump_if_zero(&mut self, address1: u16) -> Result<()>;
    fn complement_accumulator(&mut self) -> Result<()>;
    fn return_if_no_carry(&mut self) -> Result<()>;
    fn return_if_zero(&mut self) -> Result<()>;
    fn return_if_parity_odd(&mut self) -> Result<()>;
    fn return_unconditionally(&mut self) -> Result<()>;
    fn store_h_and_l_direct(&mut self, address1: u16) -> Result<()>;
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080) -> Result<()>;
    fn not_implemented(&mut self) -> Result<()>;
    fn push_data_onto_stack(&mut self, register1: Register8080) -> Result<()>;
    fn jump_if_no_carry(&mut self, address1: u16) -> Result<()>;
    fn sim(&mut self) -> Result<()>;
    fn decrement_register_pair(&mut self, register1: Register8080) -> Result<()>;
    fn complement_carry(&mut self) -> Result<()>;
    fn rotate_accumulator_left(&mut self) -> Result<()>;
    fn load_accumulator_direct(&mut self, address1: u16) -> Result<()>;
    fn exchange_stack(&mut self) -> Result<()>;
    fn rotate_accumulator_right_through_carry(&mut self) -> Result<()>;
}

pub fn dispatch_opcode<I: InstructionSet8080>(
    mut stream: &[u8],
    machine: &mut I) -> Result<(u8)>
{
    let size;
    match try!(read_u8(&mut stream)) {
        0x3e => {
            try!(machine.move_immediate_data(Register8080::A, try!(read_u8(&mut stream)))); size = 2
        }
        0x3d => {
            try!(machine.decrement_register_or_memory(Register8080::A)); size = 1
        }
        0xe4 => {
            try!(machine.call_if_parity_odd(try!(read_u16(&mut stream)))); size = 3
        }
        0x3f => {
            try!(machine.complement_carry()); size = 1
        }
        0x3a => {
            try!(machine.load_accumulator_direct(try!(read_u16(&mut stream)))); size = 3
        }
        0x3c => {
            try!(machine.increment_register_or_memory(Register8080::A)); size = 1
        }
        0x3b => {
            try!(machine.decrement_register_pair(Register8080::SP)); size = 1
        }
        0xff => {
            try!(machine.restart(7 as u8)); size = 1
        }
        0xfa => {
            try!(machine.jump_if_minus(try!(read_u16(&mut stream)))); size = 3
        }
        0xda => {
            try!(machine.jump_if_carry(try!(read_u16(&mut stream)))); size = 3
        }
        0xec => {
            try!(machine.call_if_parity_even(try!(read_u16(&mut stream)))); size = 3
        }
        0x28 => {
            try!(machine.not_implemented()); size = 1
        }
        0x29 => {
            try!(machine.double_add(Register8080::H)); size = 1
        }
        0xcf => {
            try!(machine.restart(1 as u8)); size = 1
        }
        0xf8 => {
            try!(machine.return_if_minus()); size = 1
        }
        0xeb => {
            try!(machine.exchange_registers()); size = 1
        }
        0x22 => {
            try!(machine.store_h_and_l_direct(try!(read_u16(&mut stream)))); size = 3
        }
        0x23 => {
            try!(machine.increment_register_pair(Register8080::H)); size = 1
        }
        0x20 => {
            try!(machine.rim()); size = 1
        }
        0x21 => {
            try!(machine.load_register_pair_immediate(Register8080::H, try!(read_u16(&mut stream)))); size = 3
        }
        0x26 => {
            try!(machine.move_immediate_data(Register8080::H, try!(read_u8(&mut stream)))); size = 2
        }
        0x27 => {
            try!(machine.decimal_adjust_accumulator()); size = 1
        }
        0x24 => {
            try!(machine.increment_register_or_memory(Register8080::H)); size = 1
        }
        0x25 => {
            try!(machine.decrement_register_or_memory(Register8080::H)); size = 1
        }
        0xdb => {
            try!(machine.input(try!(read_u8(&mut stream)))); size = 2
        }
        0xef => {
            try!(machine.restart(5 as u8)); size = 1
        }
        0xe2 => {
            try!(machine.jump_if_parity_odd(try!(read_u16(&mut stream)))); size = 3
        }
        0xee => {
            try!(machine.exclusive_or_immediate_with_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0xed => {
            try!(machine.not_implemented()); size = 1
        }
        0xdc => {
            try!(machine.call_if_carry(try!(read_u16(&mut stream)))); size = 3
        }
        0x35 => {
            try!(machine.decrement_register_or_memory(Register8080::M)); size = 1
        }
        0x34 => {
            try!(machine.increment_register_or_memory(Register8080::M)); size = 1
        }
        0x37 => {
            try!(machine.set_carry()); size = 1
        }
        0x36 => {
            try!(machine.move_immediate_data(Register8080::M, try!(read_u8(&mut stream)))); size = 2
        }
        0x31 => {
            try!(machine.load_register_pair_immediate(Register8080::SP, try!(read_u16(&mut stream)))); size = 3
        }
        0x30 => {
            try!(machine.sim()); size = 1
        }
        0x33 => {
            try!(machine.increment_register_pair(Register8080::SP)); size = 1
        }
        0x32 => {
            try!(machine.store_accumulator_direct(try!(read_u16(&mut stream)))); size = 3
        }
        0xd4 => {
            try!(machine.call_if_no_carry(try!(read_u16(&mut stream)))); size = 3
        }
        0xe8 => {
            try!(machine.return_if_parity_even()); size = 1
        }
        0x39 => {
            try!(machine.double_add(Register8080::SP)); size = 1
        }
        0x38 => {
            try!(machine.not_implemented()); size = 1
        }
        0xc0 => {
            try!(machine.return_if_not_zero()); size = 1
        }
        0xe1 => {
            try!(machine.pop_data_off_stack(Register8080::H)); size = 1
        }
        0xfe => {
            try!(machine.compare_immediate_with_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0x88 => {
            try!(machine.add_to_accumulator_with_carry(Register8080::B)); size = 1
        }
        0xdd => {
            try!(machine.not_implemented()); size = 1
        }
        0x89 => {
            try!(machine.add_to_accumulator_with_carry(Register8080::C)); size = 1
        }
        0x2b => {
            try!(machine.decrement_register_pair(Register8080::H)); size = 1
        }
        0x2c => {
            try!(machine.increment_register_or_memory(Register8080::L)); size = 1
        }
        0xfd => {
            try!(machine.not_implemented()); size = 1
        }
        0x2a => {
            try!(machine.load_h_and_l_direct(try!(read_u16(&mut stream)))); size = 3
        }
        0x2f => {
            try!(machine.complement_accumulator()); size = 1
        }
        0xfc => {
            try!(machine.call_if_minus(try!(read_u16(&mut stream)))); size = 3
        }
        0x2d => {
            try!(machine.decrement_register_or_memory(Register8080::L)); size = 1
        }
        0x2e => {
            try!(machine.move_immediate_data(Register8080::L, try!(read_u8(&mut stream)))); size = 2
        }
        0x5c => {
            try!(machine.move_data(Register8080::E, Register8080::H)); size = 1
        }
        0x5b => {
            try!(machine.move_data(Register8080::E, Register8080::E)); size = 1
        }
        0x5a => {
            try!(machine.move_data(Register8080::E, Register8080::D)); size = 1
        }
        0xba => {
            try!(machine.compare_with_accumulator(Register8080::D)); size = 1
        }
        0x5f => {
            try!(machine.move_data(Register8080::E, Register8080::A)); size = 1
        }
        0x5e => {
            try!(machine.move_data(Register8080::E, Register8080::M)); size = 1
        }
        0x5d => {
            try!(machine.move_data(Register8080::E, Register8080::L)); size = 1
        }
        0xc9 => {
            try!(machine.return_unconditionally()); size = 1
        }
        0x40 => {
            try!(machine.move_data(Register8080::B, Register8080::B)); size = 1
        }
        0x41 => {
            try!(machine.move_data(Register8080::B, Register8080::C)); size = 1
        }
        0x42 => {
            try!(machine.move_data(Register8080::B, Register8080::D)); size = 1
        }
        0x43 => {
            try!(machine.move_data(Register8080::B, Register8080::E)); size = 1
        }
        0x44 => {
            try!(machine.move_data(Register8080::B, Register8080::H)); size = 1
        }
        0x45 => {
            try!(machine.move_data(Register8080::B, Register8080::L)); size = 1
        }
        0x46 => {
            try!(machine.move_data(Register8080::B, Register8080::M)); size = 1
        }
        0x47 => {
            try!(machine.move_data(Register8080::B, Register8080::A)); size = 1
        }
        0x48 => {
            try!(machine.move_data(Register8080::C, Register8080::B)); size = 1
        }
        0x49 => {
            try!(machine.move_data(Register8080::C, Register8080::C)); size = 1
        }
        0xaf => {
            try!(machine.logical_exclusive_or(Register8080::A)); size = 1
        }
        0xae => {
            try!(machine.logical_exclusive_or(Register8080::M)); size = 1
        }
        0xad => {
            try!(machine.logical_exclusive_or(Register8080::L)); size = 1
        }
        0xac => {
            try!(machine.logical_exclusive_or(Register8080::H)); size = 1
        }
        0xab => {
            try!(machine.logical_exclusive_or(Register8080::E)); size = 1
        }
        0xaa => {
            try!(machine.logical_exclusive_or(Register8080::D)); size = 1
        }
        0xe6 => {
            try!(machine.add_immediate_with_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0xea => {
            try!(machine.jump_if_parity_even(try!(read_u16(&mut stream)))); size = 3
        }
        0x4a => {
            try!(machine.move_data(Register8080::C, Register8080::D)); size = 1
        }
        0x4b => {
            try!(machine.move_data(Register8080::C, Register8080::E)); size = 1
        }
        0x4c => {
            try!(machine.move_data(Register8080::C, Register8080::H)); size = 1
        }
        0x4d => {
            try!(machine.move_data(Register8080::C, Register8080::L)); size = 1
        }
        0x4e => {
            try!(machine.move_data(Register8080::C, Register8080::M)); size = 1
        }
        0x4f => {
            try!(machine.move_data(Register8080::C, Register8080::A)); size = 1
        }
        0x53 => {
            try!(machine.move_data(Register8080::D, Register8080::E)); size = 1
        }
        0x52 => {
            try!(machine.move_data(Register8080::D, Register8080::D)); size = 1
        }
        0x51 => {
            try!(machine.move_data(Register8080::D, Register8080::C)); size = 1
        }
        0x50 => {
            try!(machine.move_data(Register8080::D, Register8080::B)); size = 1
        }
        0x57 => {
            try!(machine.move_data(Register8080::D, Register8080::A)); size = 1
        }
        0x56 => {
            try!(machine.move_data(Register8080::D, Register8080::M)); size = 1
        }
        0x55 => {
            try!(machine.move_data(Register8080::D, Register8080::L)); size = 1
        }
        0x54 => {
            try!(machine.move_data(Register8080::D, Register8080::H)); size = 1
        }
        0xe5 => {
            try!(machine.push_data_onto_stack(Register8080::H)); size = 1
        }
        0x59 => {
            try!(machine.move_data(Register8080::E, Register8080::C)); size = 1
        }
        0x58 => {
            try!(machine.move_data(Register8080::E, Register8080::B)); size = 1
        }
        0xf4 => {
            try!(machine.call_if_plus(try!(read_u16(&mut stream)))); size = 3
        }
        0xfb => {
            try!(machine.enable_interrupts()); size = 1
        }
        0xf9 => {
            try!(machine.load_sp_from_h_and_l()); size = 1
        }
        0xf6 => {
            try!(machine.or_immediate_with_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0xa9 => {
            try!(machine.logical_exclusive_or(Register8080::C)); size = 1
        }
        0xa8 => {
            try!(machine.logical_exclusive_or(Register8080::B)); size = 1
        }
        0xa7 => {
            try!(machine.logical_and_with_accumulator(Register8080::A)); size = 1
        }
        0xa6 => {
            try!(machine.logical_and_with_accumulator(Register8080::M)); size = 1
        }
        0xa5 => {
            try!(machine.logical_and_with_accumulator(Register8080::L)); size = 1
        }
        0xa4 => {
            try!(machine.logical_and_with_accumulator(Register8080::H)); size = 1
        }
        0xa3 => {
            try!(machine.logical_and_with_accumulator(Register8080::E)); size = 1
        }
        0xa2 => {
            try!(machine.logical_and_with_accumulator(Register8080::D)); size = 1
        }
        0xa1 => {
            try!(machine.logical_and_with_accumulator(Register8080::C)); size = 1
        }
        0xa0 => {
            try!(machine.logical_and_with_accumulator(Register8080::B)); size = 1
        }
        0xf5 => {
            try!(machine.push_data_onto_stack(Register8080::PSW)); size = 1
        }
        0x7a => {
            try!(machine.move_data(Register8080::A, Register8080::D)); size = 1
        }
        0xf2 => {
            try!(machine.jump_if_positive(try!(read_u16(&mut stream)))); size = 3
        }
        0x7c => {
            try!(machine.move_data(Register8080::A, Register8080::H)); size = 1
        }
        0x7b => {
            try!(machine.move_data(Register8080::A, Register8080::E)); size = 1
        }
        0x7e => {
            try!(machine.move_data(Register8080::A, Register8080::M)); size = 1
        }
        0x7d => {
            try!(machine.move_data(Register8080::A, Register8080::L)); size = 1
        }
        0x7f => {
            try!(machine.move_data(Register8080::A, Register8080::A)); size = 1
        }
        0xf0 => {
            try!(machine.return_if_plus()); size = 1
        }
        0x68 => {
            try!(machine.move_data(Register8080::L, Register8080::B)); size = 1
        }
        0x69 => {
            try!(machine.move_data(Register8080::L, Register8080::C)); size = 1
        }
        0x66 => {
            try!(machine.move_data(Register8080::H, Register8080::M)); size = 1
        }
        0x67 => {
            try!(machine.move_data(Register8080::H, Register8080::A)); size = 1
        }
        0x64 => {
            try!(machine.move_data(Register8080::H, Register8080::H)); size = 1
        }
        0x65 => {
            try!(machine.move_data(Register8080::H, Register8080::L)); size = 1
        }
        0x62 => {
            try!(machine.move_data(Register8080::H, Register8080::D)); size = 1
        }
        0x63 => {
            try!(machine.move_data(Register8080::H, Register8080::E)); size = 1
        }
        0x60 => {
            try!(machine.move_data(Register8080::H, Register8080::B)); size = 1
        }
        0x61 => {
            try!(machine.move_data(Register8080::H, Register8080::C)); size = 1
        }
        0x99 => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::C)); size = 1
        }
        0xd5 => {
            try!(machine.push_data_onto_stack(Register8080::D)); size = 1
        }
        0xce => {
            try!(machine.add_immediate_to_accumulator_with_carry(try!(read_u8(&mut stream)))); size = 2
        }
        0xcd => {
            try!(machine.call(try!(read_u16(&mut stream)))); size = 3
        }
        0xb8 => {
            try!(machine.compare_with_accumulator(Register8080::B)); size = 1
        }
        0xb9 => {
            try!(machine.compare_with_accumulator(Register8080::C)); size = 1
        }
        0xca => {
            try!(machine.jump_if_zero(try!(read_u16(&mut stream)))); size = 3
        }
        0xcc => {
            try!(machine.call_if_zero(try!(read_u16(&mut stream)))); size = 3
        }
        0xcb => {
            try!(machine.not_implemented()); size = 1
        }
        0xb2 => {
            try!(machine.logical_or(Register8080::D)); size = 1
        }
        0xb3 => {
            try!(machine.logical_or(Register8080::E)); size = 1
        }
        0xb0 => {
            try!(machine.logical_or(Register8080::B)); size = 1
        }
        0xb1 => {
            try!(machine.logical_or(Register8080::C)); size = 1
        }
        0xb6 => {
            try!(machine.logical_or(Register8080::M)); size = 1
        }
        0xb7 => {
            try!(machine.logical_or(Register8080::A)); size = 1
        }
        0xb4 => {
            try!(machine.logical_or(Register8080::H)); size = 1
        }
        0xb5 => {
            try!(machine.logical_or(Register8080::L)); size = 1
        }
        0xe3 => {
            try!(machine.exchange_stack()); size = 1
        }
        0xd6 => {
            try!(machine.subtract_immediate_from_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0x6f => {
            try!(machine.move_data(Register8080::L, Register8080::A)); size = 1
        }
        0x6d => {
            try!(machine.move_data(Register8080::L, Register8080::L)); size = 1
        }
        0x6e => {
            try!(machine.move_data(Register8080::L, Register8080::M)); size = 1
        }
        0x6b => {
            try!(machine.move_data(Register8080::L, Register8080::E)); size = 1
        }
        0x6c => {
            try!(machine.move_data(Register8080::L, Register8080::H)); size = 1
        }
        0x6a => {
            try!(machine.move_data(Register8080::L, Register8080::D)); size = 1
        }
        0x79 => {
            try!(machine.move_data(Register8080::A, Register8080::C)); size = 1
        }
        0x78 => {
            try!(machine.move_data(Register8080::A, Register8080::B)); size = 1
        }
        0x71 => {
            try!(machine.move_data(Register8080::M, Register8080::C)); size = 1
        }
        0x70 => {
            try!(machine.move_data(Register8080::M, Register8080::B)); size = 1
        }
        0x73 => {
            try!(machine.move_data(Register8080::M, Register8080::E)); size = 1
        }
        0x72 => {
            try!(machine.move_data(Register8080::M, Register8080::D)); size = 1
        }
        0x75 => {
            try!(machine.move_data(Register8080::M, Register8080::L)); size = 1
        }
        0x74 => {
            try!(machine.move_data(Register8080::M, Register8080::H)); size = 1
        }
        0x77 => {
            try!(machine.move_data(Register8080::M, Register8080::A)); size = 1
        }
        0x76 => {
            try!(machine.halt()); size = 1
        }
        0xc5 => {
            try!(machine.push_data_onto_stack(Register8080::B)); size = 1
        }
        0xc4 => {
            try!(machine.call_if_not_zero(try!(read_u16(&mut stream)))); size = 3
        }
        0xc7 => {
            try!(machine.restart(0 as u8)); size = 1
        }
        0xc6 => {
            try!(machine.add_immediate_to_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0xc1 => {
            try!(machine.pop_data_off_stack(Register8080::B)); size = 1
        }
        0x8b => {
            try!(machine.add_to_accumulator_with_carry(Register8080::E)); size = 1
        }
        0xc3 => {
            try!(machine.jump(try!(read_u16(&mut stream)))); size = 3
        }
        0xc2 => {
            try!(machine.jump_if_not_zero(try!(read_u16(&mut stream)))); size = 3
        }
        0xbb => {
            try!(machine.compare_with_accumulator(Register8080::E)); size = 1
        }
        0xbc => {
            try!(machine.compare_with_accumulator(Register8080::H)); size = 1
        }
        0x8c => {
            try!(machine.add_to_accumulator_with_carry(Register8080::H)); size = 1
        }
        0xbf => {
            try!(machine.compare_with_accumulator(Register8080::A)); size = 1
        }
        0xc8 => {
            try!(machine.return_if_zero()); size = 1
        }
        0xbd => {
            try!(machine.compare_with_accumulator(Register8080::L)); size = 1
        }
        0xbe => {
            try!(machine.compare_with_accumulator(Register8080::M)); size = 1
        }
        0xf1 => {
            try!(machine.pop_data_off_stack(Register8080::PSW)); size = 1
        }
        0xe9 => {
            try!(machine.load_program_counter()); size = 1
        }
        0xd8 => {
            try!(machine.return_if_carry()); size = 1
        }
        0xd9 => {
            try!(machine.not_implemented()); size = 1
        }
        0xf7 => {
            try!(machine.restart(6 as u8)); size = 1
        }
        0xf3 => {
            try!(machine.disable_interrupts()); size = 1
        }
        0xd0 => {
            try!(machine.return_if_no_carry()); size = 1
        }
        0x9f => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::A)); size = 1
        }
        0x9e => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::M)); size = 1
        }
        0x9d => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::L)); size = 1
        }
        0x08 => {
            try!(machine.not_implemented()); size = 1
        }
        0x09 => {
            try!(machine.double_add(Register8080::B)); size = 1
        }
        0x9a => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::D)); size = 1
        }
        0xd7 => {
            try!(machine.restart(2 as u8)); size = 1
        }
        0x04 => {
            try!(machine.increment_register_or_memory(Register8080::B)); size = 1
        }
        0x05 => {
            try!(machine.decrement_register_or_memory(Register8080::B)); size = 1
        }
        0x06 => {
            try!(machine.move_immediate_data(Register8080::B, try!(read_u8(&mut stream)))); size = 2
        }
        0x07 => {
            try!(machine.rotate_accumulator_left()); size = 1
        }
        0x00 => {
            try!(machine.no_instruction()); size = 1
        }
        0x01 => {
            try!(machine.load_register_pair_immediate(Register8080::B, try!(read_u16(&mut stream)))); size = 3
        }
        0x02 => {
            try!(machine.store_accumulator(Register8080::B)); size = 1
        }
        0x03 => {
            try!(machine.increment_register_pair(Register8080::B)); size = 1
        }
        0x84 => {
            try!(machine.add_to_accumulator(Register8080::H)); size = 1
        }
        0x85 => {
            try!(machine.add_to_accumulator(Register8080::L)); size = 1
        }
        0x86 => {
            try!(machine.add_to_accumulator(Register8080::M)); size = 1
        }
        0x87 => {
            try!(machine.add_to_accumulator(Register8080::A)); size = 1
        }
        0x80 => {
            try!(machine.add_to_accumulator(Register8080::B)); size = 1
        }
        0x81 => {
            try!(machine.add_to_accumulator(Register8080::C)); size = 1
        }
        0x82 => {
            try!(machine.add_to_accumulator(Register8080::D)); size = 1
        }
        0x83 => {
            try!(machine.add_to_accumulator(Register8080::E)); size = 1
        }
        0x1f => {
            try!(machine.rotate_accumulator_right_through_carry()); size = 1
        }
        0x1e => {
            try!(machine.move_immediate_data(Register8080::E, try!(read_u8(&mut stream)))); size = 2
        }
        0x1d => {
            try!(machine.decrement_register_or_memory(Register8080::E)); size = 1
        }
        0x1c => {
            try!(machine.increment_register_or_memory(Register8080::E)); size = 1
        }
        0x1b => {
            try!(machine.decrement_register_pair(Register8080::D)); size = 1
        }
        0x1a => {
            try!(machine.load_accumulator(Register8080::D)); size = 1
        }
        0xde => {
            try!(machine.subtract_immediate_from_accumulator(try!(read_u8(&mut stream)))); size = 2
        }
        0xdf => {
            try!(machine.restart(3 as u8)); size = 1
        }
        0xd1 => {
            try!(machine.pop_data_off_stack(Register8080::D)); size = 1
        }
        0xd2 => {
            try!(machine.jump_if_no_carry(try!(read_u16(&mut stream)))); size = 3
        }
        0xd3 => {
            try!(machine.output(try!(read_u8(&mut stream)))); size = 2
        }
        0x9c => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::H)); size = 1
        }
        0x9b => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::E)); size = 1
        }
        0x8d => {
            try!(machine.add_to_accumulator_with_carry(Register8080::L)); size = 1
        }
        0x8e => {
            try!(machine.add_to_accumulator_with_carry(Register8080::M)); size = 1
        }
        0x8f => {
            try!(machine.add_to_accumulator_with_carry(Register8080::A)); size = 1
        }
        0xe0 => {
            try!(machine.return_if_parity_odd()); size = 1
        }
        0xe7 => {
            try!(machine.restart(4 as u8)); size = 1
        }
        0x8a => {
            try!(machine.add_to_accumulator_with_carry(Register8080::D)); size = 1
        }
        0x19 => {
            try!(machine.double_add(Register8080::D)); size = 1
        }
        0x18 => {
            try!(machine.not_implemented()); size = 1
        }
        0x17 => {
            try!(machine.rotate_accumulator_left_through_carry()); size = 1
        }
        0x16 => {
            try!(machine.move_immediate_data(Register8080::D, try!(read_u8(&mut stream)))); size = 2
        }
        0x15 => {
            try!(machine.decrement_register_or_memory(Register8080::D)); size = 1
        }
        0x14 => {
            try!(machine.increment_register_or_memory(Register8080::D)); size = 1
        }
        0x13 => {
            try!(machine.increment_register_pair(Register8080::D)); size = 1
        }
        0x12 => {
            try!(machine.store_accumulator(Register8080::D)); size = 1
        }
        0x11 => {
            try!(machine.load_register_pair_immediate(Register8080::D, try!(read_u16(&mut stream)))); size = 3
        }
        0x10 => {
            try!(machine.not_implemented()); size = 1
        }
        0x97 => {
            try!(machine.subtract_from_accumulator(Register8080::A)); size = 1
        }
        0x96 => {
            try!(machine.subtract_from_accumulator(Register8080::M)); size = 1
        }
        0x95 => {
            try!(machine.subtract_from_accumulator(Register8080::L)); size = 1
        }
        0x94 => {
            try!(machine.subtract_from_accumulator(Register8080::H)); size = 1
        }
        0x93 => {
            try!(machine.subtract_from_accumulator(Register8080::E)); size = 1
        }
        0x92 => {
            try!(machine.subtract_from_accumulator(Register8080::D)); size = 1
        }
        0x91 => {
            try!(machine.subtract_from_accumulator(Register8080::C)); size = 1
        }
        0x90 => {
            try!(machine.subtract_from_accumulator(Register8080::B)); size = 1
        }
        0x0d => {
            try!(machine.decrement_register_or_memory(Register8080::C)); size = 1
        }
        0x0e => {
            try!(machine.move_immediate_data(Register8080::C, try!(read_u8(&mut stream)))); size = 2
        }
        0x0f => {
            try!(machine.rotate_accumulator_right()); size = 1
        }
        0x98 => {
            try!(machine.subtract_from_accumulator_with_borrow(Register8080::B)); size = 1
        }
        0x0a => {
            try!(machine.load_accumulator(Register8080::B)); size = 1
        }
        0x0b => {
            try!(machine.decrement_register_pair(Register8080::B)); size = 1
        }
        0x0c => {
            try!(machine.increment_register_or_memory(Register8080::C)); size = 1
        }

        _ => panic!("Unknown opcode")
   };
   Ok(size)
}

pub trait OpcodePrinter<'a> {
    fn print_opcode(
        &mut self,
        stream: &[u8]) -> Result<u8>;
}
pub trait OpcodePrinterFactory<'a> {
    type Output: OpcodePrinter<'a>;
    fn new(&self, &'a mut io::Write) -> Self::Output;
}
pub struct OpcodePrinter8080<'a> {
    stream_out: &'a mut io::Write
}
pub struct OpcodePrinterFactory8080;
impl<'a> OpcodePrinterFactory<'a> for OpcodePrinterFactory8080 {
    type Output = OpcodePrinter8080<'a>;
    fn new(&self,
        stream_out: &'a mut io::Write) -> OpcodePrinter8080<'a>
    {
        return OpcodePrinter8080 {
            stream_out: stream_out
        };
    }
}
impl<'a> OpcodePrinter<'a> for OpcodePrinter8080<'a> {
    fn print_opcode(
        &mut self,
        stream: &[u8]) -> Result<u8>
    {
        Ok(try!(dispatch_opcode(stream, self)))
    }
}
impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
    fn subtract_from_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SUB"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn return_if_not_zero(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RNZ"));
        Ok(())
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn pop_data_off_stack(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "POP"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn add_to_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADD"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn jump_if_parity_even(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JPE"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn call_if_zero(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn double_add(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DAD"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ORI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn call_if_carry(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn jump(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JMP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn logical_or(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ORA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn rim(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RIM"));
        Ok(())
    }
    fn call_if_parity_even(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPE"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn jump_if_positive(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn move_data(&mut self, register1: Register8080, register2: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "MOV"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " {:?}", register2));
        Ok(())
    }
    fn no_instruction(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "NOP"));
        Ok(())
    }
    fn disable_interrupts(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DI"));
        Ok(())
    }
    fn set_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STC"));
        Ok(())
    }
    fn compare_with_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMP"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn call_if_not_zero(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CNZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn call_if_parity_odd(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPO"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SUI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn rotate_accumulator_left_through_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RAL"));
        Ok(())
    }
    fn load_sp_from_h_and_l(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SPHL"));
        Ok(())
    }
    fn logical_and_with_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ANA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn load_h_and_l_direct(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LHLD"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn add_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ANI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XRI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn call(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CALL"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn enable_interrupts(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "EI"));
        Ok(())
    }
    fn load_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LDAX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn input(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "IN"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn jump_if_parity_odd(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JPO"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn increment_register_pair(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "INX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn logical_exclusive_or(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XRA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn exchange_registers(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XCHG"));
        Ok(())
    }
    fn rotate_accumulator_right(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RRC"));
        Ok(())
    }
    fn call_if_no_carry(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CNC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn return_if_parity_even(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RPE"));
        Ok(())
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ACI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn halt(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "HLT"));
        Ok(())
    }
    fn call_if_plus(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn increment_register_or_memory(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "INR"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn load_program_counter(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "PCHL"));
        Ok(())
    }
    fn return_if_minus(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RM"));
        Ok(())
    }
    fn jump_if_carry(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn call_if_minus(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CM"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn decimal_adjust_accumulator(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DAA"));
        Ok(())
    }
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LXI"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " #${:02x}", data2));
        Ok(())
    }
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "MVI"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " #${:02x}", data2));
        Ok(())
    }
    fn return_if_plus(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RP"));
        Ok(())
    }
    fn restart(&mut self, implicit_data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RST"));
        try!(write!(self.stream_out, " {}", implicit_data1));
        Ok(())
    }
    fn return_if_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RC"));
        Ok(())
    }
    fn store_accumulator_direct(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STA"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn jump_if_not_zero(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JNZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn jump_if_minus(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JM"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn decrement_register_or_memory(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DCR"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn output(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "OUT"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn store_accumulator(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STAX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADC"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn jump_if_zero(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn complement_accumulator(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMA"));
        Ok(())
    }
    fn return_if_no_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RNC"));
        Ok(())
    }
    fn return_if_zero(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RZ"));
        Ok(())
    }
    fn return_if_parity_odd(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RPO"));
        Ok(())
    }
    fn return_unconditionally(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RET"));
        Ok(())
    }
    fn store_h_and_l_direct(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SHLD"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SBB"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn not_implemented(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "-"));
        Ok(())
    }
    fn push_data_onto_stack(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "PUSH"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn jump_if_no_carry(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JNC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn sim(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SIM"));
        Ok(())
    }
    fn decrement_register_pair(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DCX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn complement_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMC"));
        Ok(())
    }
    fn rotate_accumulator_left(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RLC"));
        Ok(())
    }
    fn load_accumulator_direct(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LDA"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn exchange_stack(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XTHL"));
        Ok(())
    }
    fn rotate_accumulator_right_through_carry(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RAR"));
        Ok(())
    }
}