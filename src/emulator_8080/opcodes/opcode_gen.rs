use emulator_common::Register8080;
use emulator_8080::opcodes::OpcodePrinter8080;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait InstructionSet8080 {
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
    fn subtract_from_accumulator(&mut self, register1: Register8080);
    fn rim(&mut self);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
    fn call_if_parity_even(&mut self, address1: u16);
    fn jump_if_positive(&mut self, address1: u16);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Register8080);
    fn move_data(&mut self, register1: Register8080, register2: Register8080);
    fn no_instruction(&mut self);
    fn halt(&mut self);
    fn set_carry(&mut self);
    fn compare_with_accumulator(&mut self, register1: Register8080);
    fn call_if_not_zero(&mut self, address1: u16);
    fn call_if_parity_odd(&mut self, address1: u16);
    fn return_if_zero(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn disable_interrupts(&mut self);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Register8080);
    fn load_h_and_l_direct(&mut self, address1: u16);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn enable_interrupts(&mut self);
    fn load_accumulator(&mut self, register1: Register8080);
    fn input(&mut self, data1: u8);
    fn jump_if_parity_odd(&mut self, address1: u16);
    fn increment_register_pair(&mut self, register1: Register8080);
    fn return_if_no_carry(&mut self);
    fn logical_or_with_accumulator(&mut self, register1: Register8080);
    fn exchange_registers(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn call_if_no_carry(&mut self, address1: u16);
    fn return_if_parity_even(&mut self);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
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
    fn store_accumulator_direct(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_minus(&mut self, address1: u16);
    fn decrement_register_or_memory(&mut self, register1: Register8080);
    fn output(&mut self, data1: u8);
    fn store_accumulator(&mut self, register1: Register8080);
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080);
    fn jump_if_zero(&mut self, address1: u16);
    fn complement_accumulator(&mut self);
    fn return_if_carry(&mut self);
    fn return_if_parity_odd(&mut self);
    fn return_unconditionally(&mut self);
    fn store_h_and_l_direct(&mut self, address1: u16);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
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

pub fn dispatch_8080_instruction<I: InstructionSet8080>(
    mut stream: &[u8],
    machine: &mut I)
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x00 => machine.no_instruction(),
        0x01 => machine.load_register_pair_immediate(Register8080::B, read_u16(&mut stream).unwrap()),
        0x02 => machine.store_accumulator(Register8080::B),
        0x03 => machine.increment_register_pair(Register8080::B),
        0x04 => machine.increment_register_or_memory(Register8080::B),
        0x05 => machine.decrement_register_or_memory(Register8080::B),
        0x06 => machine.move_immediate_data(Register8080::B, read_u8(&mut stream).unwrap()),
        0x07 => machine.rotate_accumulator_left(),
        0x09 => machine.double_add(Register8080::B),
        0x0A => machine.load_accumulator(Register8080::B),
        0x0B => machine.decrement_register_pair(Register8080::B),
        0x0C => machine.increment_register_or_memory(Register8080::C),
        0x0D => machine.decrement_register_or_memory(Register8080::C),
        0x0E => machine.move_immediate_data(Register8080::C, read_u8(&mut stream).unwrap()),
        0x0F => machine.rotate_accumulator_right(),
        0x11 => machine.load_register_pair_immediate(Register8080::D, read_u16(&mut stream).unwrap()),
        0x12 => machine.store_accumulator(Register8080::D),
        0x13 => machine.increment_register_pair(Register8080::D),
        0x14 => machine.increment_register_or_memory(Register8080::D),
        0x15 => machine.decrement_register_or_memory(Register8080::D),
        0x16 => machine.move_immediate_data(Register8080::D, read_u8(&mut stream).unwrap()),
        0x17 => machine.rotate_accumulator_left_through_carry(),
        0x19 => machine.double_add(Register8080::D),
        0x1A => machine.load_accumulator(Register8080::D),
        0x1B => machine.decrement_register_pair(Register8080::D),
        0x1C => machine.increment_register_or_memory(Register8080::E),
        0x1D => machine.decrement_register_or_memory(Register8080::E),
        0x1E => machine.move_immediate_data(Register8080::E, read_u8(&mut stream).unwrap()),
        0x1F => machine.rotate_accumulator_right_through_carry(),
        0x20 => machine.rim(),
        0x21 => machine.load_register_pair_immediate(Register8080::H, read_u16(&mut stream).unwrap()),
        0x22 => machine.store_h_and_l_direct(read_u16(&mut stream).unwrap()),
        0x23 => machine.increment_register_pair(Register8080::H),
        0x24 => machine.increment_register_or_memory(Register8080::H),
        0x25 => machine.decrement_register_or_memory(Register8080::H),
        0x26 => machine.move_immediate_data(Register8080::H, read_u8(&mut stream).unwrap()),
        0x27 => machine.decimal_adjust_accumulator(),
        0x29 => machine.double_add(Register8080::H),
        0x2A => machine.load_h_and_l_direct(read_u16(&mut stream).unwrap()),
        0x2B => machine.decrement_register_pair(Register8080::H),
        0x2C => machine.increment_register_or_memory(Register8080::L),
        0x2D => machine.decrement_register_or_memory(Register8080::L),
        0x2E => machine.move_immediate_data(Register8080::L, read_u8(&mut stream).unwrap()),
        0x2F => machine.complement_accumulator(),
        0x30 => machine.sim(),
        0x31 => machine.load_register_pair_immediate(Register8080::SP, read_u16(&mut stream).unwrap()),
        0x32 => machine.store_accumulator_direct(read_u16(&mut stream).unwrap()),
        0x33 => machine.increment_register_pair(Register8080::SP),
        0x34 => machine.increment_register_or_memory(Register8080::M),
        0x35 => machine.decrement_register_or_memory(Register8080::M),
        0x36 => machine.move_immediate_data(Register8080::M, read_u8(&mut stream).unwrap()),
        0x37 => machine.set_carry(),
        0x39 => machine.double_add(Register8080::SP),
        0x3A => machine.load_accumulator_direct(read_u16(&mut stream).unwrap()),
        0x3B => machine.decrement_register_pair(Register8080::SP),
        0x3C => machine.increment_register_or_memory(Register8080::A),
        0x3D => machine.decrement_register_or_memory(Register8080::A),
        0x3E => machine.move_immediate_data(Register8080::A, read_u8(&mut stream).unwrap()),
        0x3F => machine.complement_carry(),
        0x40 => machine.move_data(Register8080::B, Register8080::B),
        0x41 => machine.move_data(Register8080::B, Register8080::C),
        0x42 => machine.move_data(Register8080::B, Register8080::D),
        0x43 => machine.move_data(Register8080::B, Register8080::E),
        0x44 => machine.move_data(Register8080::B, Register8080::H),
        0x45 => machine.move_data(Register8080::B, Register8080::L),
        0x46 => machine.move_data(Register8080::B, Register8080::M),
        0x47 => machine.move_data(Register8080::B, Register8080::A),
        0x48 => machine.move_data(Register8080::C, Register8080::B),
        0x49 => machine.move_data(Register8080::C, Register8080::C),
        0x4A => machine.move_data(Register8080::C, Register8080::D),
        0x4B => machine.move_data(Register8080::C, Register8080::E),
        0x4C => machine.move_data(Register8080::C, Register8080::H),
        0x4D => machine.move_data(Register8080::C, Register8080::L),
        0x4E => machine.move_data(Register8080::C, Register8080::M),
        0x4F => machine.move_data(Register8080::C, Register8080::A),
        0x50 => machine.move_data(Register8080::D, Register8080::B),
        0x51 => machine.move_data(Register8080::D, Register8080::C),
        0x52 => machine.move_data(Register8080::D, Register8080::D),
        0x53 => machine.move_data(Register8080::D, Register8080::E),
        0x54 => machine.move_data(Register8080::D, Register8080::H),
        0x55 => machine.move_data(Register8080::D, Register8080::L),
        0x56 => machine.move_data(Register8080::D, Register8080::M),
        0x57 => machine.move_data(Register8080::D, Register8080::A),
        0x58 => machine.move_data(Register8080::E, Register8080::B),
        0x59 => machine.move_data(Register8080::E, Register8080::C),
        0x5A => machine.move_data(Register8080::E, Register8080::D),
        0x5B => machine.move_data(Register8080::E, Register8080::E),
        0x5C => machine.move_data(Register8080::E, Register8080::H),
        0x5D => machine.move_data(Register8080::E, Register8080::L),
        0x5E => machine.move_data(Register8080::E, Register8080::M),
        0x5F => machine.move_data(Register8080::E, Register8080::A),
        0x60 => machine.move_data(Register8080::H, Register8080::B),
        0x61 => machine.move_data(Register8080::H, Register8080::C),
        0x62 => machine.move_data(Register8080::H, Register8080::D),
        0x63 => machine.move_data(Register8080::H, Register8080::E),
        0x64 => machine.move_data(Register8080::H, Register8080::H),
        0x65 => machine.move_data(Register8080::H, Register8080::L),
        0x66 => machine.move_data(Register8080::H, Register8080::M),
        0x67 => machine.move_data(Register8080::H, Register8080::A),
        0x68 => machine.move_data(Register8080::L, Register8080::B),
        0x69 => machine.move_data(Register8080::L, Register8080::C),
        0x6A => machine.move_data(Register8080::L, Register8080::D),
        0x6B => machine.move_data(Register8080::L, Register8080::E),
        0x6C => machine.move_data(Register8080::L, Register8080::H),
        0x6D => machine.move_data(Register8080::L, Register8080::L),
        0x6E => machine.move_data(Register8080::L, Register8080::M),
        0x6F => machine.move_data(Register8080::L, Register8080::A),
        0x70 => machine.move_data(Register8080::M, Register8080::B),
        0x71 => machine.move_data(Register8080::M, Register8080::C),
        0x72 => machine.move_data(Register8080::M, Register8080::D),
        0x73 => machine.move_data(Register8080::M, Register8080::E),
        0x74 => machine.move_data(Register8080::M, Register8080::H),
        0x75 => machine.move_data(Register8080::M, Register8080::L),
        0x76 => machine.halt(),
        0x77 => machine.move_data(Register8080::M, Register8080::A),
        0x78 => machine.move_data(Register8080::A, Register8080::B),
        0x79 => machine.move_data(Register8080::A, Register8080::C),
        0x7A => machine.move_data(Register8080::A, Register8080::D),
        0x7B => machine.move_data(Register8080::A, Register8080::E),
        0x7C => machine.move_data(Register8080::A, Register8080::H),
        0x7D => machine.move_data(Register8080::A, Register8080::L),
        0x7E => machine.move_data(Register8080::A, Register8080::M),
        0x7F => machine.move_data(Register8080::A, Register8080::A),
        0x80 => machine.add_to_accumulator(Register8080::B),
        0x81 => machine.add_to_accumulator(Register8080::C),
        0x82 => machine.add_to_accumulator(Register8080::D),
        0x83 => machine.add_to_accumulator(Register8080::E),
        0x84 => machine.add_to_accumulator(Register8080::H),
        0x85 => machine.add_to_accumulator(Register8080::L),
        0x86 => machine.add_to_accumulator(Register8080::M),
        0x87 => machine.add_to_accumulator(Register8080::A),
        0x88 => machine.add_to_accumulator_with_carry(Register8080::B),
        0x89 => machine.add_to_accumulator_with_carry(Register8080::C),
        0x8A => machine.add_to_accumulator_with_carry(Register8080::D),
        0x8B => machine.add_to_accumulator_with_carry(Register8080::E),
        0x8C => machine.add_to_accumulator_with_carry(Register8080::H),
        0x8D => machine.add_to_accumulator_with_carry(Register8080::L),
        0x8E => machine.add_to_accumulator_with_carry(Register8080::M),
        0x8F => machine.add_to_accumulator_with_carry(Register8080::A),
        0x90 => machine.subtract_from_accumulator(Register8080::B),
        0x91 => machine.subtract_from_accumulator(Register8080::C),
        0x92 => machine.subtract_from_accumulator(Register8080::D),
        0x93 => machine.subtract_from_accumulator(Register8080::E),
        0x94 => machine.subtract_from_accumulator(Register8080::H),
        0x95 => machine.subtract_from_accumulator(Register8080::L),
        0x96 => machine.subtract_from_accumulator(Register8080::M),
        0x97 => machine.subtract_from_accumulator(Register8080::A),
        0x98 => machine.subtract_from_accumulator_with_borrow(Register8080::B),
        0x99 => machine.subtract_from_accumulator_with_borrow(Register8080::C),
        0x9A => machine.subtract_from_accumulator_with_borrow(Register8080::D),
        0x9B => machine.subtract_from_accumulator_with_borrow(Register8080::E),
        0x9C => machine.subtract_from_accumulator_with_borrow(Register8080::H),
        0x9D => machine.subtract_from_accumulator_with_borrow(Register8080::L),
        0x9E => machine.subtract_from_accumulator_with_borrow(Register8080::M),
        0x9F => machine.subtract_from_accumulator_with_borrow(Register8080::A),
        0xA0 => machine.logical_and_with_accumulator(Register8080::B),
        0xA1 => machine.logical_and_with_accumulator(Register8080::C),
        0xA2 => machine.logical_and_with_accumulator(Register8080::D),
        0xA3 => machine.logical_and_with_accumulator(Register8080::E),
        0xA4 => machine.logical_and_with_accumulator(Register8080::H),
        0xA5 => machine.logical_and_with_accumulator(Register8080::L),
        0xA6 => machine.logical_and_with_accumulator(Register8080::M),
        0xA7 => machine.logical_and_with_accumulator(Register8080::A),
        0xA8 => machine.logical_exclusive_or_with_accumulator(Register8080::B),
        0xA9 => machine.logical_exclusive_or_with_accumulator(Register8080::C),
        0xAA => machine.logical_exclusive_or_with_accumulator(Register8080::D),
        0xAB => machine.logical_exclusive_or_with_accumulator(Register8080::E),
        0xAC => machine.logical_exclusive_or_with_accumulator(Register8080::H),
        0xAD => machine.logical_exclusive_or_with_accumulator(Register8080::L),
        0xAE => machine.logical_exclusive_or_with_accumulator(Register8080::M),
        0xAF => machine.logical_exclusive_or_with_accumulator(Register8080::A),
        0xB0 => machine.logical_or_with_accumulator(Register8080::B),
        0xB1 => machine.logical_or_with_accumulator(Register8080::C),
        0xB2 => machine.logical_or_with_accumulator(Register8080::D),
        0xB3 => machine.logical_or_with_accumulator(Register8080::E),
        0xB4 => machine.logical_or_with_accumulator(Register8080::H),
        0xB5 => machine.logical_or_with_accumulator(Register8080::L),
        0xB6 => machine.logical_or_with_accumulator(Register8080::M),
        0xB7 => machine.logical_or_with_accumulator(Register8080::A),
        0xB8 => machine.compare_with_accumulator(Register8080::B),
        0xB9 => machine.compare_with_accumulator(Register8080::C),
        0xBA => machine.compare_with_accumulator(Register8080::D),
        0xBB => machine.compare_with_accumulator(Register8080::E),
        0xBC => machine.compare_with_accumulator(Register8080::H),
        0xBD => machine.compare_with_accumulator(Register8080::L),
        0xBE => machine.compare_with_accumulator(Register8080::M),
        0xBF => machine.compare_with_accumulator(Register8080::A),
        0xC0 => machine.return_if_not_zero(),
        0xC1 => machine.pop_data_off_stack(Register8080::B),
        0xC2 => machine.jump_if_not_zero(read_u16(&mut stream).unwrap()),
        0xC3 => machine.jump(read_u16(&mut stream).unwrap()),
        0xC4 => machine.call_if_not_zero(read_u16(&mut stream).unwrap()),
        0xC5 => machine.push_data_onto_stack(Register8080::B),
        0xC6 => machine.add_immediate_to_accumulator(read_u8(&mut stream).unwrap()),
        0xC7 => machine.restart(0 as u8),
        0xC8 => machine.return_if_zero(),
        0xC9 => machine.return_unconditionally(),
        0xCA => machine.jump_if_zero(read_u16(&mut stream).unwrap()),
        0xCC => machine.call_if_zero(read_u16(&mut stream).unwrap()),
        0xCD => machine.call(read_u16(&mut stream).unwrap()),
        0xCE => machine.add_immediate_to_accumulator_with_carry(read_u8(&mut stream).unwrap()),
        0xCF => machine.restart(1 as u8),
        0xD0 => machine.return_if_no_carry(),
        0xD1 => machine.pop_data_off_stack(Register8080::D),
        0xD2 => machine.jump_if_no_carry(read_u16(&mut stream).unwrap()),
        0xD3 => machine.output(read_u8(&mut stream).unwrap()),
        0xD4 => machine.call_if_no_carry(read_u16(&mut stream).unwrap()),
        0xD5 => machine.push_data_onto_stack(Register8080::D),
        0xD6 => machine.subtract_immediate_from_accumulator(read_u8(&mut stream).unwrap()),
        0xD7 => machine.restart(2 as u8),
        0xD8 => machine.return_if_carry(),
        0xDA => machine.jump_if_carry(read_u16(&mut stream).unwrap()),
        0xDB => machine.input(read_u8(&mut stream).unwrap()),
        0xDC => machine.call_if_carry(read_u16(&mut stream).unwrap()),
        0xDE => machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).unwrap()),
        0xDF => machine.restart(3 as u8),
        0xE0 => machine.return_if_parity_odd(),
        0xE1 => machine.pop_data_off_stack(Register8080::H),
        0xE2 => machine.jump_if_parity_odd(read_u16(&mut stream).unwrap()),
        0xE3 => machine.exchange_stack(),
        0xE4 => machine.call_if_parity_odd(read_u16(&mut stream).unwrap()),
        0xE5 => machine.push_data_onto_stack(Register8080::H),
        0xE6 => machine.and_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xE7 => machine.restart(4 as u8),
        0xE8 => machine.return_if_parity_even(),
        0xE9 => machine.load_program_counter(),
        0xEA => machine.jump_if_parity_even(read_u16(&mut stream).unwrap()),
        0xEB => machine.exchange_registers(),
        0xEC => machine.call_if_parity_even(read_u16(&mut stream).unwrap()),
        0xEE => machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xEF => machine.restart(5 as u8),
        0xF0 => machine.return_if_plus(),
        0xF1 => machine.pop_data_off_stack(Register8080::PSW),
        0xF2 => machine.jump_if_positive(read_u16(&mut stream).unwrap()),
        0xF3 => machine.disable_interrupts(),
        0xF4 => machine.call_if_plus(read_u16(&mut stream).unwrap()),
        0xF5 => machine.push_data_onto_stack(Register8080::PSW),
        0xF6 => machine.or_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xF7 => machine.restart(6 as u8),
        0xF8 => machine.return_if_minus(),
        0xF9 => machine.load_sp_from_h_and_l(),
        0xFA => machine.jump_if_minus(read_u16(&mut stream).unwrap()),
        0xFB => machine.enable_interrupts(),
        0xFC => machine.call_if_minus(read_u16(&mut stream).unwrap()),
        0xFE => machine.compare_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xFF => machine.restart(7 as u8),
        v => panic!("Unknown opcode {}", v)
    };
}

pub fn get_8080_instruction(original_stream: &[u8]) -> Option<Vec<u8>>
{
    let mut stream = original_stream;
    let size = match read_u8(&mut stream).unwrap() {
        0x00 => 1,
        0x01 => 3,
        0x02 => 1,
        0x03 => 1,
        0x04 => 1,
        0x05 => 1,
        0x06 => 2,
        0x07 => 1,
        0x09 => 1,
        0x0A => 1,
        0x0B => 1,
        0x0C => 1,
        0x0D => 1,
        0x0E => 2,
        0x0F => 1,
        0x11 => 3,
        0x12 => 1,
        0x13 => 1,
        0x14 => 1,
        0x15 => 1,
        0x16 => 2,
        0x17 => 1,
        0x19 => 1,
        0x1A => 1,
        0x1B => 1,
        0x1C => 1,
        0x1D => 1,
        0x1E => 2,
        0x1F => 1,
        0x20 => 1,
        0x21 => 3,
        0x22 => 3,
        0x23 => 1,
        0x24 => 1,
        0x25 => 1,
        0x26 => 2,
        0x27 => 1,
        0x29 => 1,
        0x2A => 3,
        0x2B => 1,
        0x2C => 1,
        0x2D => 1,
        0x2E => 2,
        0x2F => 1,
        0x30 => 1,
        0x31 => 3,
        0x32 => 3,
        0x33 => 1,
        0x34 => 1,
        0x35 => 1,
        0x36 => 2,
        0x37 => 1,
        0x39 => 1,
        0x3A => 3,
        0x3B => 1,
        0x3C => 1,
        0x3D => 1,
        0x3E => 2,
        0x3F => 1,
        0x40 => 1,
        0x41 => 1,
        0x42 => 1,
        0x43 => 1,
        0x44 => 1,
        0x45 => 1,
        0x46 => 1,
        0x47 => 1,
        0x48 => 1,
        0x49 => 1,
        0x4A => 1,
        0x4B => 1,
        0x4C => 1,
        0x4D => 1,
        0x4E => 1,
        0x4F => 1,
        0x50 => 1,
        0x51 => 1,
        0x52 => 1,
        0x53 => 1,
        0x54 => 1,
        0x55 => 1,
        0x56 => 1,
        0x57 => 1,
        0x58 => 1,
        0x59 => 1,
        0x5A => 1,
        0x5B => 1,
        0x5C => 1,
        0x5D => 1,
        0x5E => 1,
        0x5F => 1,
        0x60 => 1,
        0x61 => 1,
        0x62 => 1,
        0x63 => 1,
        0x64 => 1,
        0x65 => 1,
        0x66 => 1,
        0x67 => 1,
        0x68 => 1,
        0x69 => 1,
        0x6A => 1,
        0x6B => 1,
        0x6C => 1,
        0x6D => 1,
        0x6E => 1,
        0x6F => 1,
        0x70 => 1,
        0x71 => 1,
        0x72 => 1,
        0x73 => 1,
        0x74 => 1,
        0x75 => 1,
        0x76 => 1,
        0x77 => 1,
        0x78 => 1,
        0x79 => 1,
        0x7A => 1,
        0x7B => 1,
        0x7C => 1,
        0x7D => 1,
        0x7E => 1,
        0x7F => 1,
        0x80 => 1,
        0x81 => 1,
        0x82 => 1,
        0x83 => 1,
        0x84 => 1,
        0x85 => 1,
        0x86 => 1,
        0x87 => 1,
        0x88 => 1,
        0x89 => 1,
        0x8A => 1,
        0x8B => 1,
        0x8C => 1,
        0x8D => 1,
        0x8E => 1,
        0x8F => 1,
        0x90 => 1,
        0x91 => 1,
        0x92 => 1,
        0x93 => 1,
        0x94 => 1,
        0x95 => 1,
        0x96 => 1,
        0x97 => 1,
        0x98 => 1,
        0x99 => 1,
        0x9A => 1,
        0x9B => 1,
        0x9C => 1,
        0x9D => 1,
        0x9E => 1,
        0x9F => 1,
        0xA0 => 1,
        0xA1 => 1,
        0xA2 => 1,
        0xA3 => 1,
        0xA4 => 1,
        0xA5 => 1,
        0xA6 => 1,
        0xA7 => 1,
        0xA8 => 1,
        0xA9 => 1,
        0xAA => 1,
        0xAB => 1,
        0xAC => 1,
        0xAD => 1,
        0xAE => 1,
        0xAF => 1,
        0xB0 => 1,
        0xB1 => 1,
        0xB2 => 1,
        0xB3 => 1,
        0xB4 => 1,
        0xB5 => 1,
        0xB6 => 1,
        0xB7 => 1,
        0xB8 => 1,
        0xB9 => 1,
        0xBA => 1,
        0xBB => 1,
        0xBC => 1,
        0xBD => 1,
        0xBE => 1,
        0xBF => 1,
        0xC0 => 1,
        0xC1 => 1,
        0xC2 => 3,
        0xC3 => 3,
        0xC4 => 3,
        0xC5 => 1,
        0xC6 => 2,
        0xC7 => 1,
        0xC8 => 1,
        0xC9 => 1,
        0xCA => 3,
        0xCC => 3,
        0xCD => 3,
        0xCE => 2,
        0xCF => 1,
        0xD0 => 1,
        0xD1 => 1,
        0xD2 => 3,
        0xD3 => 2,
        0xD4 => 3,
        0xD5 => 1,
        0xD6 => 2,
        0xD7 => 1,
        0xD8 => 1,
        0xDA => 3,
        0xDB => 2,
        0xDC => 3,
        0xDE => 2,
        0xDF => 1,
        0xE0 => 1,
        0xE1 => 1,
        0xE2 => 3,
        0xE3 => 1,
        0xE4 => 3,
        0xE5 => 1,
        0xE6 => 2,
        0xE7 => 1,
        0xE8 => 1,
        0xE9 => 1,
        0xEA => 3,
        0xEB => 1,
        0xEC => 3,
        0xEE => 2,
        0xEF => 1,
        0xF0 => 1,
        0xF1 => 1,
        0xF2 => 3,
        0xF3 => 1,
        0xF4 => 3,
        0xF5 => 1,
        0xF6 => 2,
        0xF7 => 1,
        0xF8 => 1,
        0xF9 => 1,
        0xFA => 3,
        0xFB => 1,
        0xFC => 3,
        0xFE => 2,
        0xFF => 1,
        _ => return None
    };

    let mut instruction = vec![];
    instruction.resize(size, 0);
    instruction.clone_from_slice(&original_stream[0..size]);
    return Some(instruction);
}

impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
    fn return_if_not_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNZ").unwrap();
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ADI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn pop_data_off_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "POP").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn add_to_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADD").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn jump_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPE").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CZ").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn double_add(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DAD").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ORI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CC").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn jump(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JMP").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn subtract_from_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SUB").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn rim(&mut self)
    {
        write!(self.stream_out, "{:04}", "RIM").unwrap();
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "SBI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn call_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPE").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn jump_if_positive(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JP").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "XRA").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn move_data(&mut self, register1: Register8080, register2: Register8080)
    {
        write!(self.stream_out, "{:04}", "MOV").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
        write!(self.stream_out, " {:?}", register2).unwrap();
    }
    fn no_instruction(&mut self)
    {
        write!(self.stream_out, "{:04}", "NOP").unwrap();
    }
    fn halt(&mut self)
    {
        write!(self.stream_out, "{:04}", "HLT").unwrap();
    }
    fn set_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "STC").unwrap();
    }
    fn compare_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "CMP").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNZ").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn call_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPO").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn return_if_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RZ").unwrap();
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAL").unwrap();
    }
    fn disable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "DI").unwrap();
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        write!(self.stream_out, "{:04}", "SPHL").unwrap();
    }
    fn logical_and_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ANA").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn load_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LHLD").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "XRI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn call(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CALL").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn enable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "EI").unwrap();
    }
    fn load_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "LDAX").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn input(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "IN").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn jump_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPO").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn increment_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INX").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn return_if_no_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNC").unwrap();
    }
    fn logical_or_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ORA").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn exchange_registers(&mut self)
    {
        write!(self.stream_out, "{:04}", "XCHG").unwrap();
    }
    fn rotate_accumulator_right(&mut self)
    {
        write!(self.stream_out, "{:04}", "RRC").unwrap();
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNC").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn return_if_parity_even(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPE").unwrap();
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ACI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ANI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn call_if_plus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CP").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn increment_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INR").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "CPI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn load_program_counter(&mut self)
    {
        write!(self.stream_out, "{:04}", "PCHL").unwrap();
    }
    fn return_if_minus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RM").unwrap();
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JC").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn call_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CM").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "DAA").unwrap();
    }
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16)
    {
        write!(self.stream_out, "{:04}", "LXI").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
        write!(self.stream_out, " #${:02x}", data2).unwrap();
    }
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8)
    {
        write!(self.stream_out, "{:04}", "MVI").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
        write!(self.stream_out, " #${:02x}", data2).unwrap();
    }
    fn return_if_plus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RP").unwrap();
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        write!(self.stream_out, "{:04}", "RST").unwrap();
        write!(self.stream_out, " {}", implicit_data1).unwrap();
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "STA").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNZ").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn jump_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JM").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn decrement_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCR").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn output(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "OUT").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn store_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "STAX").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADC").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JZ").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn complement_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMA").unwrap();
    }
    fn return_if_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RC").unwrap();
    }
    fn return_if_parity_odd(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPO").unwrap();
    }
    fn return_unconditionally(&mut self)
    {
        write!(self.stream_out, "{:04}", "RET").unwrap();
    }
    fn store_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "SHLD").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SBB").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "SUI").unwrap();
        write!(self.stream_out, " #${:02x}", data1).unwrap();
    }
    fn push_data_onto_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "PUSH").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNC").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn sim(&mut self)
    {
        write!(self.stream_out, "{:04}", "SIM").unwrap();
    }
    fn decrement_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCX").unwrap();
        write!(self.stream_out, " {:?}", register1).unwrap();
    }
    fn complement_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMC").unwrap();
    }
    fn rotate_accumulator_left(&mut self)
    {
        write!(self.stream_out, "{:04}", "RLC").unwrap();
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LDA").unwrap();
        write!(self.stream_out, " ${:02x}", address1).unwrap();
    }
    fn exchange_stack(&mut self)
    {
        write!(self.stream_out, "{:04}", "XTHL").unwrap();
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAR").unwrap();
    }
}
