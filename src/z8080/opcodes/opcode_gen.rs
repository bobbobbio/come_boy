
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
    fn subtract_from_accumulator(&mut self, register1: Register8080);
    fn return_if_not_zero(&mut self);
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Register8080);
    fn add_to_accumulator(&mut self, register1: Register8080);
    fn jump_if_parity_even(&mut self, address1: u16);
    fn call_if_zero(&mut self, address1: u16);
    fn double_add(&mut self, register1: Register8080);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn call_if_carry(&mut self, address1: u16);
    fn jump(&mut self, address1: u16);
    fn logical_or(&mut self, register1: Register8080);
    fn rim(&mut self);
    fn call_if_parity_even(&mut self, address1: u16);
    fn jump_if_positive(&mut self, address1: u16);
    fn move_data(&mut self, register1: Register8080, register2: Register8080);
    fn no_instruction(&mut self);
    fn disable_interrupts(&mut self);
    fn set_carry(&mut self);
    fn compare_with_accumulator(&mut self, register1: Register8080);
    fn call_if_not_zero(&mut self, address1: u16);
    fn call_if_parity_odd(&mut self, address1: u16);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Register8080);
    fn load_h_and_l_direct(&mut self, address1: u16);
    fn add_immediate_with_accumulator(&mut self, data1: u8);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn enable_interrupts(&mut self);
    fn load_accumulator(&mut self, register1: Register8080);
    fn input(&mut self, data1: u8);
    fn jump_if_parity_odd(&mut self, address1: u16);
    fn increment_register_pair(&mut self, register1: Register8080);
    fn logical_exclusive_or(&mut self, register1: Register8080);
    fn exchange_registers(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn call_if_no_carry(&mut self, address1: u16);
    fn return_if_parity_even(&mut self);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn halt(&mut self);
    fn call_if_plus(&mut self, address1: u16);
    fn increment_register_or_memory(&mut self, register1: Register8080);
    fn compare_immediate_with_accumulator(&mut self, data1: u8);
    fn load_program_counter(&mut self);
    fn return_if_minus(&mut self);
    fn jump_if_carry(&mut self, address1: u16);
    fn call_if_minus(&mut self, address1: u16);
    fn decimal_adjust_accumulator(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16);
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8);
    fn return_if_plus(&mut self);
    fn restart(&mut self, implicit_data1: u8);
    fn return_if_carry(&mut self);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_minus(&mut self, address1: u16);
    fn decrement_register_or_memory(&mut self, register1: Register8080);
    fn output(&mut self, data1: u8);
    fn store_accumulator(&mut self, register1: Register8080);
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080);
    fn jump_if_zero(&mut self, address1: u16);
    fn complement_accumulator(&mut self);
    fn return_if_no_carry(&mut self);
    fn return_if_zero(&mut self);
    fn return_if_parity_odd(&mut self);
    fn return_unconditionally(&mut self);
    fn store_h_and_l_direct(&mut self, address1: u16);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080);
    fn not_implemented(&mut self);
    fn push_data_onto_stack(&mut self, register1: Register8080);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn sim(&mut self);
    fn decrement_register_pair(&mut self, register1: Register8080);
    fn complement_carry(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn exchange_stack(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
}

pub fn dispatch_opcode<I: InstructionSet8080>(
    mut stream: &[u8],
    machine: &mut I) -> u8
{
    let size;
    match read_u8(&mut stream).ok().expect("") {
        0x3e => {
            machine.move_immediate_data(Register8080::A, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x3d => {
            machine.decrement_register_or_memory(Register8080::A); size = 1
        }
        0xe4 => {
            machine.call_if_parity_odd(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x3f => {
            machine.complement_carry(); size = 1
        }
        0x3a => {
            machine.load_accumulator_direct(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x3c => {
            machine.increment_register_or_memory(Register8080::A); size = 1
        }
        0x3b => {
            machine.decrement_register_pair(Register8080::SP); size = 1
        }
        0xff => {
            machine.restart(7 as u8); size = 1
        }
        0xfa => {
            machine.jump_if_minus(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xda => {
            machine.jump_if_carry(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xec => {
            machine.call_if_parity_even(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x28 => {
            machine.not_implemented(); size = 1
        }
        0x29 => {
            machine.double_add(Register8080::H); size = 1
        }
        0xcf => {
            machine.restart(1 as u8); size = 1
        }
        0xf8 => {
            machine.return_if_minus(); size = 1
        }
        0xeb => {
            machine.exchange_registers(); size = 1
        }
        0x22 => {
            machine.store_h_and_l_direct(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x23 => {
            machine.increment_register_pair(Register8080::H); size = 1
        }
        0x20 => {
            machine.rim(); size = 1
        }
        0x21 => {
            machine.load_register_pair_immediate(Register8080::H, read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x26 => {
            machine.move_immediate_data(Register8080::H, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x27 => {
            machine.decimal_adjust_accumulator(); size = 1
        }
        0x24 => {
            machine.increment_register_or_memory(Register8080::H); size = 1
        }
        0x25 => {
            machine.decrement_register_or_memory(Register8080::H); size = 1
        }
        0xdb => {
            machine.input(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xef => {
            machine.restart(5 as u8); size = 1
        }
        0xe2 => {
            machine.jump_if_parity_odd(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xee => {
            machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xed => {
            machine.not_implemented(); size = 1
        }
        0xdc => {
            machine.call_if_carry(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x35 => {
            machine.decrement_register_or_memory(Register8080::M); size = 1
        }
        0x34 => {
            machine.increment_register_or_memory(Register8080::M); size = 1
        }
        0x37 => {
            machine.set_carry(); size = 1
        }
        0x36 => {
            machine.move_immediate_data(Register8080::M, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x31 => {
            machine.load_register_pair_immediate(Register8080::SP, read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x30 => {
            machine.sim(); size = 1
        }
        0x33 => {
            machine.increment_register_pair(Register8080::SP); size = 1
        }
        0x32 => {
            machine.store_accumulator_direct(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xd4 => {
            machine.call_if_no_carry(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xe8 => {
            machine.return_if_parity_even(); size = 1
        }
        0x39 => {
            machine.double_add(Register8080::SP); size = 1
        }
        0x38 => {
            machine.not_implemented(); size = 1
        }
        0xc0 => {
            machine.return_if_not_zero(); size = 1
        }
        0xe1 => {
            machine.pop_data_off_stack(Register8080::H); size = 1
        }
        0xfe => {
            machine.compare_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x88 => {
            machine.add_to_accumulator_with_carry(Register8080::B); size = 1
        }
        0xdd => {
            machine.not_implemented(); size = 1
        }
        0x89 => {
            machine.add_to_accumulator_with_carry(Register8080::C); size = 1
        }
        0x2b => {
            machine.decrement_register_pair(Register8080::H); size = 1
        }
        0x2c => {
            machine.increment_register_or_memory(Register8080::L); size = 1
        }
        0xfd => {
            machine.not_implemented(); size = 1
        }
        0x2a => {
            machine.load_h_and_l_direct(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x2f => {
            machine.complement_accumulator(); size = 1
        }
        0xfc => {
            machine.call_if_minus(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x2d => {
            machine.decrement_register_or_memory(Register8080::L); size = 1
        }
        0x2e => {
            machine.move_immediate_data(Register8080::L, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x5c => {
            machine.move_data(Register8080::E, Register8080::H); size = 1
        }
        0x5b => {
            machine.move_data(Register8080::E, Register8080::E); size = 1
        }
        0x5a => {
            machine.move_data(Register8080::E, Register8080::D); size = 1
        }
        0xba => {
            machine.compare_with_accumulator(Register8080::D); size = 1
        }
        0x5f => {
            machine.move_data(Register8080::E, Register8080::A); size = 1
        }
        0x5e => {
            machine.move_data(Register8080::E, Register8080::M); size = 1
        }
        0x5d => {
            machine.move_data(Register8080::E, Register8080::L); size = 1
        }
        0xc9 => {
            machine.return_unconditionally(); size = 1
        }
        0x40 => {
            machine.move_data(Register8080::B, Register8080::B); size = 1
        }
        0x41 => {
            machine.move_data(Register8080::B, Register8080::C); size = 1
        }
        0x42 => {
            machine.move_data(Register8080::B, Register8080::D); size = 1
        }
        0x43 => {
            machine.move_data(Register8080::B, Register8080::E); size = 1
        }
        0x44 => {
            machine.move_data(Register8080::B, Register8080::H); size = 1
        }
        0x45 => {
            machine.move_data(Register8080::B, Register8080::L); size = 1
        }
        0x46 => {
            machine.move_data(Register8080::B, Register8080::M); size = 1
        }
        0x47 => {
            machine.move_data(Register8080::B, Register8080::A); size = 1
        }
        0x48 => {
            machine.move_data(Register8080::C, Register8080::B); size = 1
        }
        0x49 => {
            machine.move_data(Register8080::C, Register8080::C); size = 1
        }
        0xaf => {
            machine.logical_exclusive_or(Register8080::A); size = 1
        }
        0xae => {
            machine.logical_exclusive_or(Register8080::M); size = 1
        }
        0xad => {
            machine.logical_exclusive_or(Register8080::L); size = 1
        }
        0xac => {
            machine.logical_exclusive_or(Register8080::H); size = 1
        }
        0xab => {
            machine.logical_exclusive_or(Register8080::E); size = 1
        }
        0xaa => {
            machine.logical_exclusive_or(Register8080::D); size = 1
        }
        0xe6 => {
            machine.add_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xea => {
            machine.jump_if_parity_even(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x4a => {
            machine.move_data(Register8080::C, Register8080::D); size = 1
        }
        0x4b => {
            machine.move_data(Register8080::C, Register8080::E); size = 1
        }
        0x4c => {
            machine.move_data(Register8080::C, Register8080::H); size = 1
        }
        0x4d => {
            machine.move_data(Register8080::C, Register8080::L); size = 1
        }
        0x4e => {
            machine.move_data(Register8080::C, Register8080::M); size = 1
        }
        0x4f => {
            machine.move_data(Register8080::C, Register8080::A); size = 1
        }
        0x53 => {
            machine.move_data(Register8080::D, Register8080::E); size = 1
        }
        0x52 => {
            machine.move_data(Register8080::D, Register8080::D); size = 1
        }
        0x51 => {
            machine.move_data(Register8080::D, Register8080::C); size = 1
        }
        0x50 => {
            machine.move_data(Register8080::D, Register8080::B); size = 1
        }
        0x57 => {
            machine.move_data(Register8080::D, Register8080::A); size = 1
        }
        0x56 => {
            machine.move_data(Register8080::D, Register8080::M); size = 1
        }
        0x55 => {
            machine.move_data(Register8080::D, Register8080::L); size = 1
        }
        0x54 => {
            machine.move_data(Register8080::D, Register8080::H); size = 1
        }
        0xe5 => {
            machine.push_data_onto_stack(Register8080::H); size = 1
        }
        0x59 => {
            machine.move_data(Register8080::E, Register8080::C); size = 1
        }
        0x58 => {
            machine.move_data(Register8080::E, Register8080::B); size = 1
        }
        0xf4 => {
            machine.call_if_plus(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xfb => {
            machine.enable_interrupts(); size = 1
        }
        0xf9 => {
            machine.load_sp_from_h_and_l(); size = 1
        }
        0xf6 => {
            machine.or_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xa9 => {
            machine.logical_exclusive_or(Register8080::C); size = 1
        }
        0xa8 => {
            machine.logical_exclusive_or(Register8080::B); size = 1
        }
        0xa7 => {
            machine.logical_and_with_accumulator(Register8080::A); size = 1
        }
        0xa6 => {
            machine.logical_and_with_accumulator(Register8080::M); size = 1
        }
        0xa5 => {
            machine.logical_and_with_accumulator(Register8080::L); size = 1
        }
        0xa4 => {
            machine.logical_and_with_accumulator(Register8080::H); size = 1
        }
        0xa3 => {
            machine.logical_and_with_accumulator(Register8080::E); size = 1
        }
        0xa2 => {
            machine.logical_and_with_accumulator(Register8080::D); size = 1
        }
        0xa1 => {
            machine.logical_and_with_accumulator(Register8080::C); size = 1
        }
        0xa0 => {
            machine.logical_and_with_accumulator(Register8080::B); size = 1
        }
        0xf5 => {
            machine.push_data_onto_stack(Register8080::PSW); size = 1
        }
        0x7a => {
            machine.move_data(Register8080::A, Register8080::D); size = 1
        }
        0xf2 => {
            machine.jump_if_positive(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x7c => {
            machine.move_data(Register8080::A, Register8080::H); size = 1
        }
        0x7b => {
            machine.move_data(Register8080::A, Register8080::E); size = 1
        }
        0x7e => {
            machine.move_data(Register8080::A, Register8080::M); size = 1
        }
        0x7d => {
            machine.move_data(Register8080::A, Register8080::L); size = 1
        }
        0x7f => {
            machine.move_data(Register8080::A, Register8080::A); size = 1
        }
        0xf0 => {
            machine.return_if_plus(); size = 1
        }
        0x68 => {
            machine.move_data(Register8080::L, Register8080::B); size = 1
        }
        0x69 => {
            machine.move_data(Register8080::L, Register8080::C); size = 1
        }
        0x66 => {
            machine.move_data(Register8080::H, Register8080::M); size = 1
        }
        0x67 => {
            machine.move_data(Register8080::H, Register8080::A); size = 1
        }
        0x64 => {
            machine.move_data(Register8080::H, Register8080::H); size = 1
        }
        0x65 => {
            machine.move_data(Register8080::H, Register8080::L); size = 1
        }
        0x62 => {
            machine.move_data(Register8080::H, Register8080::D); size = 1
        }
        0x63 => {
            machine.move_data(Register8080::H, Register8080::E); size = 1
        }
        0x60 => {
            machine.move_data(Register8080::H, Register8080::B); size = 1
        }
        0x61 => {
            machine.move_data(Register8080::H, Register8080::C); size = 1
        }
        0x99 => {
            machine.subtract_from_accumulator_with_borrow(Register8080::C); size = 1
        }
        0xd5 => {
            machine.push_data_onto_stack(Register8080::D); size = 1
        }
        0xce => {
            machine.add_immediate_to_accumulator_with_carry(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xcd => {
            machine.call(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xb8 => {
            machine.compare_with_accumulator(Register8080::B); size = 1
        }
        0xb9 => {
            machine.compare_with_accumulator(Register8080::C); size = 1
        }
        0xca => {
            machine.jump_if_zero(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xcc => {
            machine.call_if_zero(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xcb => {
            machine.not_implemented(); size = 1
        }
        0xb2 => {
            machine.logical_or(Register8080::D); size = 1
        }
        0xb3 => {
            machine.logical_or(Register8080::E); size = 1
        }
        0xb0 => {
            machine.logical_or(Register8080::B); size = 1
        }
        0xb1 => {
            machine.logical_or(Register8080::C); size = 1
        }
        0xb6 => {
            machine.logical_or(Register8080::M); size = 1
        }
        0xb7 => {
            machine.logical_or(Register8080::A); size = 1
        }
        0xb4 => {
            machine.logical_or(Register8080::H); size = 1
        }
        0xb5 => {
            machine.logical_or(Register8080::L); size = 1
        }
        0xe3 => {
            machine.exchange_stack(); size = 1
        }
        0xd6 => {
            machine.subtract_immediate_from_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x6f => {
            machine.move_data(Register8080::L, Register8080::A); size = 1
        }
        0x6d => {
            machine.move_data(Register8080::L, Register8080::L); size = 1
        }
        0x6e => {
            machine.move_data(Register8080::L, Register8080::M); size = 1
        }
        0x6b => {
            machine.move_data(Register8080::L, Register8080::E); size = 1
        }
        0x6c => {
            machine.move_data(Register8080::L, Register8080::H); size = 1
        }
        0x6a => {
            machine.move_data(Register8080::L, Register8080::D); size = 1
        }
        0x79 => {
            machine.move_data(Register8080::A, Register8080::C); size = 1
        }
        0x78 => {
            machine.move_data(Register8080::A, Register8080::B); size = 1
        }
        0x71 => {
            machine.move_data(Register8080::M, Register8080::C); size = 1
        }
        0x70 => {
            machine.move_data(Register8080::M, Register8080::B); size = 1
        }
        0x73 => {
            machine.move_data(Register8080::M, Register8080::E); size = 1
        }
        0x72 => {
            machine.move_data(Register8080::M, Register8080::D); size = 1
        }
        0x75 => {
            machine.move_data(Register8080::M, Register8080::L); size = 1
        }
        0x74 => {
            machine.move_data(Register8080::M, Register8080::H); size = 1
        }
        0x77 => {
            machine.move_data(Register8080::M, Register8080::A); size = 1
        }
        0x76 => {
            machine.halt(); size = 1
        }
        0xc5 => {
            machine.push_data_onto_stack(Register8080::B); size = 1
        }
        0xc4 => {
            machine.call_if_not_zero(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xc7 => {
            machine.restart(0 as u8); size = 1
        }
        0xc6 => {
            machine.add_immediate_to_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xc1 => {
            machine.pop_data_off_stack(Register8080::B); size = 1
        }
        0x8b => {
            machine.add_to_accumulator_with_carry(Register8080::E); size = 1
        }
        0xc3 => {
            machine.jump(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xc2 => {
            machine.jump_if_not_zero(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xbb => {
            machine.compare_with_accumulator(Register8080::E); size = 1
        }
        0xbc => {
            machine.compare_with_accumulator(Register8080::H); size = 1
        }
        0x8c => {
            machine.add_to_accumulator_with_carry(Register8080::H); size = 1
        }
        0xbf => {
            machine.compare_with_accumulator(Register8080::A); size = 1
        }
        0xc8 => {
            machine.return_if_zero(); size = 1
        }
        0xbd => {
            machine.compare_with_accumulator(Register8080::L); size = 1
        }
        0xbe => {
            machine.compare_with_accumulator(Register8080::M); size = 1
        }
        0xf1 => {
            machine.pop_data_off_stack(Register8080::PSW); size = 1
        }
        0xe9 => {
            machine.load_program_counter(); size = 1
        }
        0xd8 => {
            machine.return_if_carry(); size = 1
        }
        0xd9 => {
            machine.not_implemented(); size = 1
        }
        0xf7 => {
            machine.restart(6 as u8); size = 1
        }
        0xf3 => {
            machine.disable_interrupts(); size = 1
        }
        0xd0 => {
            machine.return_if_no_carry(); size = 1
        }
        0x9f => {
            machine.subtract_from_accumulator_with_borrow(Register8080::A); size = 1
        }
        0x9e => {
            machine.subtract_from_accumulator_with_borrow(Register8080::M); size = 1
        }
        0x9d => {
            machine.subtract_from_accumulator_with_borrow(Register8080::L); size = 1
        }
        0x08 => {
            machine.not_implemented(); size = 1
        }
        0x09 => {
            machine.double_add(Register8080::B); size = 1
        }
        0x9a => {
            machine.subtract_from_accumulator_with_borrow(Register8080::D); size = 1
        }
        0xd7 => {
            machine.restart(2 as u8); size = 1
        }
        0x04 => {
            machine.increment_register_or_memory(Register8080::B); size = 1
        }
        0x05 => {
            machine.decrement_register_or_memory(Register8080::B); size = 1
        }
        0x06 => {
            machine.move_immediate_data(Register8080::B, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x07 => {
            machine.rotate_accumulator_left(); size = 1
        }
        0x00 => {
            machine.no_instruction(); size = 1
        }
        0x01 => {
            machine.load_register_pair_immediate(Register8080::B, read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x02 => {
            machine.store_accumulator(Register8080::B); size = 1
        }
        0x03 => {
            machine.increment_register_pair(Register8080::B); size = 1
        }
        0x84 => {
            machine.add_to_accumulator(Register8080::H); size = 1
        }
        0x85 => {
            machine.add_to_accumulator(Register8080::L); size = 1
        }
        0x86 => {
            machine.add_to_accumulator(Register8080::M); size = 1
        }
        0x87 => {
            machine.add_to_accumulator(Register8080::A); size = 1
        }
        0x80 => {
            machine.add_to_accumulator(Register8080::B); size = 1
        }
        0x81 => {
            machine.add_to_accumulator(Register8080::C); size = 1
        }
        0x82 => {
            machine.add_to_accumulator(Register8080::D); size = 1
        }
        0x83 => {
            machine.add_to_accumulator(Register8080::E); size = 1
        }
        0x1f => {
            machine.rotate_accumulator_right_through_carry(); size = 1
        }
        0x1e => {
            machine.move_immediate_data(Register8080::E, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x1d => {
            machine.decrement_register_or_memory(Register8080::E); size = 1
        }
        0x1c => {
            machine.increment_register_or_memory(Register8080::E); size = 1
        }
        0x1b => {
            machine.decrement_register_pair(Register8080::D); size = 1
        }
        0x1a => {
            machine.load_accumulator(Register8080::D); size = 1
        }
        0xde => {
            machine.subtract_immediate_from_accumulator(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0xdf => {
            machine.restart(3 as u8); size = 1
        }
        0xd1 => {
            machine.pop_data_off_stack(Register8080::D); size = 1
        }
        0xd2 => {
            machine.jump_if_no_carry(read_u16(&mut stream).ok().expect("")); size = 3
        }
        0xd3 => {
            machine.output(read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x9c => {
            machine.subtract_from_accumulator_with_borrow(Register8080::H); size = 1
        }
        0x9b => {
            machine.subtract_from_accumulator_with_borrow(Register8080::E); size = 1
        }
        0x8d => {
            machine.add_to_accumulator_with_carry(Register8080::L); size = 1
        }
        0x8e => {
            machine.add_to_accumulator_with_carry(Register8080::M); size = 1
        }
        0x8f => {
            machine.add_to_accumulator_with_carry(Register8080::A); size = 1
        }
        0xe0 => {
            machine.return_if_parity_odd(); size = 1
        }
        0xe7 => {
            machine.restart(4 as u8); size = 1
        }
        0x8a => {
            machine.add_to_accumulator_with_carry(Register8080::D); size = 1
        }
        0x19 => {
            machine.double_add(Register8080::D); size = 1
        }
        0x18 => {
            machine.not_implemented(); size = 1
        }
        0x17 => {
            machine.rotate_accumulator_left_through_carry(); size = 1
        }
        0x16 => {
            machine.move_immediate_data(Register8080::D, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x15 => {
            machine.decrement_register_or_memory(Register8080::D); size = 1
        }
        0x14 => {
            machine.increment_register_or_memory(Register8080::D); size = 1
        }
        0x13 => {
            machine.increment_register_pair(Register8080::D); size = 1
        }
        0x12 => {
            machine.store_accumulator(Register8080::D); size = 1
        }
        0x11 => {
            machine.load_register_pair_immediate(Register8080::D, read_u16(&mut stream).ok().expect("")); size = 3
        }
        0x10 => {
            machine.not_implemented(); size = 1
        }
        0x97 => {
            machine.subtract_from_accumulator(Register8080::A); size = 1
        }
        0x96 => {
            machine.subtract_from_accumulator(Register8080::M); size = 1
        }
        0x95 => {
            machine.subtract_from_accumulator(Register8080::L); size = 1
        }
        0x94 => {
            machine.subtract_from_accumulator(Register8080::H); size = 1
        }
        0x93 => {
            machine.subtract_from_accumulator(Register8080::E); size = 1
        }
        0x92 => {
            machine.subtract_from_accumulator(Register8080::D); size = 1
        }
        0x91 => {
            machine.subtract_from_accumulator(Register8080::C); size = 1
        }
        0x90 => {
            machine.subtract_from_accumulator(Register8080::B); size = 1
        }
        0x0d => {
            machine.decrement_register_or_memory(Register8080::C); size = 1
        }
        0x0e => {
            machine.move_immediate_data(Register8080::C, read_u8(&mut stream).ok().expect("")); size = 2
        }
        0x0f => {
            machine.rotate_accumulator_right(); size = 1
        }
        0x98 => {
            machine.subtract_from_accumulator_with_borrow(Register8080::B); size = 1
        }
        0x0a => {
            machine.load_accumulator(Register8080::B); size = 1
        }
        0x0b => {
            machine.decrement_register_pair(Register8080::B); size = 1
        }
        0x0c => {
            machine.increment_register_or_memory(Register8080::C); size = 1
        }

        _ => panic!("Unknown opcode")
   };
   size
}

pub trait OpcodePrinter<'a> {
    fn print_opcode(
        &mut self,
        stream: &[u8]) -> u8;
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
        stream: &[u8]) -> u8
    {
        dispatch_opcode(stream, self)
    }
}
impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
    fn subtract_from_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SUB").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn return_if_not_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNZ").ok().expect("Failed to Write to Stream");
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ADI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn pop_data_off_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "POP").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn add_to_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADD").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPE").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CZ").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn double_add(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DAD").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ORI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CC").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn jump(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JMP").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn logical_or(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ORA").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn rim(&mut self)
    {
        write!(self.stream_out, "{:04}", "RIM").ok().expect("Failed to Write to Stream");
    }
    fn call_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPE").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_positive(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JP").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn move_data(&mut self, register1: Register8080, register2: Register8080)
    {
        write!(self.stream_out, "{:04}", "MOV").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register2).ok().expect("Failed to Write to Stream");
    }
    fn no_instruction(&mut self)
    {
        write!(self.stream_out, "{:04}", "NOP").ok().expect("Failed to Write to Stream");
    }
    fn disable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "DI").ok().expect("Failed to Write to Stream");
    }
    fn set_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "STC").ok().expect("Failed to Write to Stream");
    }
    fn compare_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "CMP").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNZ").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn call_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPO").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "SUI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAL").ok().expect("Failed to Write to Stream");
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        write!(self.stream_out, "{:04}", "SPHL").ok().expect("Failed to Write to Stream");
    }
    fn logical_and_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ANA").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn load_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LHLD").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn add_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ANI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "XRI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn call(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CALL").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn enable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "EI").ok().expect("Failed to Write to Stream");
    }
    fn load_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "LDAX").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn input(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "IN").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPO").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn increment_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INX").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn logical_exclusive_or(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "XRA").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn exchange_registers(&mut self)
    {
        write!(self.stream_out, "{:04}", "XCHG").ok().expect("Failed to Write to Stream");
    }
    fn rotate_accumulator_right(&mut self)
    {
        write!(self.stream_out, "{:04}", "RRC").ok().expect("Failed to Write to Stream");
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNC").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn return_if_parity_even(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPE").ok().expect("Failed to Write to Stream");
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ACI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn halt(&mut self)
    {
        write!(self.stream_out, "{:04}", "HLT").ok().expect("Failed to Write to Stream");
    }
    fn call_if_plus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CP").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn increment_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INR").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "CPI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn load_program_counter(&mut self)
    {
        write!(self.stream_out, "{:04}", "PCHL").ok().expect("Failed to Write to Stream");
    }
    fn return_if_minus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RM").ok().expect("Failed to Write to Stream");
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JC").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn call_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CM").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "DAA").ok().expect("Failed to Write to Stream");
    }
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16)
    {
        write!(self.stream_out, "{:04}", "LXI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data2).ok().expect("Failed to Write to Stream");
    }
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8)
    {
        write!(self.stream_out, "{:04}", "MVI").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data2).ok().expect("Failed to Write to Stream");
    }
    fn return_if_plus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RP").ok().expect("Failed to Write to Stream");
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        write!(self.stream_out, "{:04}", "RST").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {}", implicit_data1).ok().expect("Failed to Write to Stream");
    }
    fn return_if_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RC").ok().expect("Failed to Write to Stream");
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "STA").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNZ").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JM").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn decrement_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCR").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn output(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "OUT").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("Failed to Write to Stream");
    }
    fn store_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "STAX").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADC").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JZ").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn complement_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMA").ok().expect("Failed to Write to Stream");
    }
    fn return_if_no_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNC").ok().expect("Failed to Write to Stream");
    }
    fn return_if_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RZ").ok().expect("Failed to Write to Stream");
    }
    fn return_if_parity_odd(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPO").ok().expect("Failed to Write to Stream");
    }
    fn return_unconditionally(&mut self)
    {
        write!(self.stream_out, "{:04}", "RET").ok().expect("Failed to Write to Stream");
    }
    fn store_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "SHLD").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SBB").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn not_implemented(&mut self)
    {
        write!(self.stream_out, "{:04}", "-").ok().expect("Failed to Write to Stream");
    }
    fn push_data_onto_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "PUSH").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNC").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn sim(&mut self)
    {
        write!(self.stream_out, "{:04}", "SIM").ok().expect("Failed to Write to Stream");
    }
    fn decrement_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCX").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " {:?}", register1).ok().expect("Failed to Write to Stream");
    }
    fn complement_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMC").ok().expect("Failed to Write to Stream");
    }
    fn rotate_accumulator_left(&mut self)
    {
        write!(self.stream_out, "{:04}", "RLC").ok().expect("Failed to Write to Stream");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LDA").ok().expect("Failed to Write to Stream");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("Failed to Write to Stream");
    }
    fn exchange_stack(&mut self)
    {
        write!(self.stream_out, "{:04}", "XTHL").ok().expect("Failed to Write to Stream");
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAR").ok().expect("Failed to Write to Stream");
    }
}