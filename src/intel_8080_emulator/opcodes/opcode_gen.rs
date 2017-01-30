use emulator_common::Intel8080Register;
use emulator_common::InstructionOption;
use emulator_common::InstructionOption::*;
use intel_8080_emulator::opcodes::Intel8080InstructionPrinter;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait Intel8080InstructionSet {
    fn return_if_not_zero(&mut self);
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn jump_if_parity_even(&mut self, address1: u16);
    fn call_if_zero(&mut self, address1: u16);
    fn double_add(&mut self, register1: Intel8080Register);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn call_if_carry(&mut self, address1: u16);
    fn jump(&mut self, address1: u16);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn rim(&mut self);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
    fn call_if_parity_even(&mut self, address1: u16);
    fn jump_if_positive(&mut self, address1: u16);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn halt(&mut self);
    fn set_carry(&mut self);
    fn compare_with_accumulator(&mut self, register1: Intel8080Register);
    fn call_if_not_zero(&mut self, address1: u16);
    fn call_if_parity_odd(&mut self, address1: u16);
    fn return_if_zero(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn disable_interrupts(&mut self);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn load_h_and_l_direct(&mut self, address1: u16);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn enable_interrupts(&mut self);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn input(&mut self, data1: u8);
    fn jump_if_parity_odd(&mut self, address1: u16);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn return_if_no_carry(&mut self);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn exchange_registers(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn call_if_no_carry(&mut self, address1: u16);
    fn return_if_parity_even(&mut self);
    fn no_operation(&mut self);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn call_if_plus(&mut self, address1: u16);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn compare_immediate_with_accumulator(&mut self, data1: u8);
    fn load_program_counter(&mut self);
    fn return_if_minus(&mut self);
    fn jump_if_carry(&mut self, address1: u16);
    fn call_if_minus(&mut self, address1: u16);
    fn decimal_adjust_accumulator(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn return_if_plus(&mut self);
    fn restart(&mut self, implicit_data1: u8);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_minus(&mut self, address1: u16);
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register);
    fn output(&mut self, data1: u8);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn jump_if_zero(&mut self, address1: u16);
    fn complement_accumulator(&mut self);
    fn return_if_carry(&mut self);
    fn return_if_parity_odd(&mut self);
    fn return_unconditionally(&mut self);
    fn store_h_and_l_direct(&mut self, address1: u16);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn sim(&mut self);
    fn decrement_register_pair(&mut self, register1: Intel8080Register);
    fn complement_carry(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn exchange_stack(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
}

pub fn dispatch_intel8080_instruction<I: Intel8080InstructionSet>(
    mut stream: &[u8],
    machine: &mut I)
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x00 => machine.no_operation(),
        0x01 => machine.load_register_pair_immediate(Intel8080Register::B, read_u16(&mut stream).unwrap()),
        0x02 => machine.store_accumulator(Intel8080Register::B),
        0x03 => machine.increment_register_pair(Intel8080Register::B),
        0x04 => machine.increment_register_or_memory(Intel8080Register::B),
        0x05 => machine.decrement_register_or_memory(Intel8080Register::B),
        0x06 => machine.move_immediate_data(Intel8080Register::B, read_u8(&mut stream).unwrap()),
        0x07 => machine.rotate_accumulator_left(),
        0x09 => machine.double_add(Intel8080Register::B),
        0x0A => machine.load_accumulator(Intel8080Register::B),
        0x0B => machine.decrement_register_pair(Intel8080Register::B),
        0x0C => machine.increment_register_or_memory(Intel8080Register::C),
        0x0D => machine.decrement_register_or_memory(Intel8080Register::C),
        0x0E => machine.move_immediate_data(Intel8080Register::C, read_u8(&mut stream).unwrap()),
        0x0F => machine.rotate_accumulator_right(),
        0x11 => machine.load_register_pair_immediate(Intel8080Register::D, read_u16(&mut stream).unwrap()),
        0x12 => machine.store_accumulator(Intel8080Register::D),
        0x13 => machine.increment_register_pair(Intel8080Register::D),
        0x14 => machine.increment_register_or_memory(Intel8080Register::D),
        0x15 => machine.decrement_register_or_memory(Intel8080Register::D),
        0x16 => machine.move_immediate_data(Intel8080Register::D, read_u8(&mut stream).unwrap()),
        0x17 => machine.rotate_accumulator_left_through_carry(),
        0x19 => machine.double_add(Intel8080Register::D),
        0x1A => machine.load_accumulator(Intel8080Register::D),
        0x1B => machine.decrement_register_pair(Intel8080Register::D),
        0x1C => machine.increment_register_or_memory(Intel8080Register::E),
        0x1D => machine.decrement_register_or_memory(Intel8080Register::E),
        0x1E => machine.move_immediate_data(Intel8080Register::E, read_u8(&mut stream).unwrap()),
        0x1F => machine.rotate_accumulator_right_through_carry(),
        0x20 => machine.rim(),
        0x21 => machine.load_register_pair_immediate(Intel8080Register::H, read_u16(&mut stream).unwrap()),
        0x22 => machine.store_h_and_l_direct(read_u16(&mut stream).unwrap()),
        0x23 => machine.increment_register_pair(Intel8080Register::H),
        0x24 => machine.increment_register_or_memory(Intel8080Register::H),
        0x25 => machine.decrement_register_or_memory(Intel8080Register::H),
        0x26 => machine.move_immediate_data(Intel8080Register::H, read_u8(&mut stream).unwrap()),
        0x27 => machine.decimal_adjust_accumulator(),
        0x29 => machine.double_add(Intel8080Register::H),
        0x2A => machine.load_h_and_l_direct(read_u16(&mut stream).unwrap()),
        0x2B => machine.decrement_register_pair(Intel8080Register::H),
        0x2C => machine.increment_register_or_memory(Intel8080Register::L),
        0x2D => machine.decrement_register_or_memory(Intel8080Register::L),
        0x2E => machine.move_immediate_data(Intel8080Register::L, read_u8(&mut stream).unwrap()),
        0x2F => machine.complement_accumulator(),
        0x30 => machine.sim(),
        0x31 => machine.load_register_pair_immediate(Intel8080Register::SP, read_u16(&mut stream).unwrap()),
        0x32 => machine.store_accumulator_direct(read_u16(&mut stream).unwrap()),
        0x33 => machine.increment_register_pair(Intel8080Register::SP),
        0x34 => machine.increment_register_or_memory(Intel8080Register::M),
        0x35 => machine.decrement_register_or_memory(Intel8080Register::M),
        0x36 => machine.move_immediate_data(Intel8080Register::M, read_u8(&mut stream).unwrap()),
        0x37 => machine.set_carry(),
        0x39 => machine.double_add(Intel8080Register::SP),
        0x3A => machine.load_accumulator_direct(read_u16(&mut stream).unwrap()),
        0x3B => machine.decrement_register_pair(Intel8080Register::SP),
        0x3C => machine.increment_register_or_memory(Intel8080Register::A),
        0x3D => machine.decrement_register_or_memory(Intel8080Register::A),
        0x3E => machine.move_immediate_data(Intel8080Register::A, read_u8(&mut stream).unwrap()),
        0x3F => machine.complement_carry(),
        0x40 => machine.move_data(Intel8080Register::B, Intel8080Register::B),
        0x41 => machine.move_data(Intel8080Register::B, Intel8080Register::C),
        0x42 => machine.move_data(Intel8080Register::B, Intel8080Register::D),
        0x43 => machine.move_data(Intel8080Register::B, Intel8080Register::E),
        0x44 => machine.move_data(Intel8080Register::B, Intel8080Register::H),
        0x45 => machine.move_data(Intel8080Register::B, Intel8080Register::L),
        0x46 => machine.move_data(Intel8080Register::B, Intel8080Register::M),
        0x47 => machine.move_data(Intel8080Register::B, Intel8080Register::A),
        0x48 => machine.move_data(Intel8080Register::C, Intel8080Register::B),
        0x49 => machine.move_data(Intel8080Register::C, Intel8080Register::C),
        0x4A => machine.move_data(Intel8080Register::C, Intel8080Register::D),
        0x4B => machine.move_data(Intel8080Register::C, Intel8080Register::E),
        0x4C => machine.move_data(Intel8080Register::C, Intel8080Register::H),
        0x4D => machine.move_data(Intel8080Register::C, Intel8080Register::L),
        0x4E => machine.move_data(Intel8080Register::C, Intel8080Register::M),
        0x4F => machine.move_data(Intel8080Register::C, Intel8080Register::A),
        0x50 => machine.move_data(Intel8080Register::D, Intel8080Register::B),
        0x51 => machine.move_data(Intel8080Register::D, Intel8080Register::C),
        0x52 => machine.move_data(Intel8080Register::D, Intel8080Register::D),
        0x53 => machine.move_data(Intel8080Register::D, Intel8080Register::E),
        0x54 => machine.move_data(Intel8080Register::D, Intel8080Register::H),
        0x55 => machine.move_data(Intel8080Register::D, Intel8080Register::L),
        0x56 => machine.move_data(Intel8080Register::D, Intel8080Register::M),
        0x57 => machine.move_data(Intel8080Register::D, Intel8080Register::A),
        0x58 => machine.move_data(Intel8080Register::E, Intel8080Register::B),
        0x59 => machine.move_data(Intel8080Register::E, Intel8080Register::C),
        0x5A => machine.move_data(Intel8080Register::E, Intel8080Register::D),
        0x5B => machine.move_data(Intel8080Register::E, Intel8080Register::E),
        0x5C => machine.move_data(Intel8080Register::E, Intel8080Register::H),
        0x5D => machine.move_data(Intel8080Register::E, Intel8080Register::L),
        0x5E => machine.move_data(Intel8080Register::E, Intel8080Register::M),
        0x5F => machine.move_data(Intel8080Register::E, Intel8080Register::A),
        0x60 => machine.move_data(Intel8080Register::H, Intel8080Register::B),
        0x61 => machine.move_data(Intel8080Register::H, Intel8080Register::C),
        0x62 => machine.move_data(Intel8080Register::H, Intel8080Register::D),
        0x63 => machine.move_data(Intel8080Register::H, Intel8080Register::E),
        0x64 => machine.move_data(Intel8080Register::H, Intel8080Register::H),
        0x65 => machine.move_data(Intel8080Register::H, Intel8080Register::L),
        0x66 => machine.move_data(Intel8080Register::H, Intel8080Register::M),
        0x67 => machine.move_data(Intel8080Register::H, Intel8080Register::A),
        0x68 => machine.move_data(Intel8080Register::L, Intel8080Register::B),
        0x69 => machine.move_data(Intel8080Register::L, Intel8080Register::C),
        0x6A => machine.move_data(Intel8080Register::L, Intel8080Register::D),
        0x6B => machine.move_data(Intel8080Register::L, Intel8080Register::E),
        0x6C => machine.move_data(Intel8080Register::L, Intel8080Register::H),
        0x6D => machine.move_data(Intel8080Register::L, Intel8080Register::L),
        0x6E => machine.move_data(Intel8080Register::L, Intel8080Register::M),
        0x6F => machine.move_data(Intel8080Register::L, Intel8080Register::A),
        0x70 => machine.move_data(Intel8080Register::M, Intel8080Register::B),
        0x71 => machine.move_data(Intel8080Register::M, Intel8080Register::C),
        0x72 => machine.move_data(Intel8080Register::M, Intel8080Register::D),
        0x73 => machine.move_data(Intel8080Register::M, Intel8080Register::E),
        0x74 => machine.move_data(Intel8080Register::M, Intel8080Register::H),
        0x75 => machine.move_data(Intel8080Register::M, Intel8080Register::L),
        0x76 => machine.halt(),
        0x77 => machine.move_data(Intel8080Register::M, Intel8080Register::A),
        0x78 => machine.move_data(Intel8080Register::A, Intel8080Register::B),
        0x79 => machine.move_data(Intel8080Register::A, Intel8080Register::C),
        0x7A => machine.move_data(Intel8080Register::A, Intel8080Register::D),
        0x7B => machine.move_data(Intel8080Register::A, Intel8080Register::E),
        0x7C => machine.move_data(Intel8080Register::A, Intel8080Register::H),
        0x7D => machine.move_data(Intel8080Register::A, Intel8080Register::L),
        0x7E => machine.move_data(Intel8080Register::A, Intel8080Register::M),
        0x7F => machine.move_data(Intel8080Register::A, Intel8080Register::A),
        0x80 => machine.add_to_accumulator(Intel8080Register::B),
        0x81 => machine.add_to_accumulator(Intel8080Register::C),
        0x82 => machine.add_to_accumulator(Intel8080Register::D),
        0x83 => machine.add_to_accumulator(Intel8080Register::E),
        0x84 => machine.add_to_accumulator(Intel8080Register::H),
        0x85 => machine.add_to_accumulator(Intel8080Register::L),
        0x86 => machine.add_to_accumulator(Intel8080Register::M),
        0x87 => machine.add_to_accumulator(Intel8080Register::A),
        0x88 => machine.add_to_accumulator_with_carry(Intel8080Register::B),
        0x89 => machine.add_to_accumulator_with_carry(Intel8080Register::C),
        0x8A => machine.add_to_accumulator_with_carry(Intel8080Register::D),
        0x8B => machine.add_to_accumulator_with_carry(Intel8080Register::E),
        0x8C => machine.add_to_accumulator_with_carry(Intel8080Register::H),
        0x8D => machine.add_to_accumulator_with_carry(Intel8080Register::L),
        0x8E => machine.add_to_accumulator_with_carry(Intel8080Register::M),
        0x8F => machine.add_to_accumulator_with_carry(Intel8080Register::A),
        0x90 => machine.subtract_from_accumulator(Intel8080Register::B),
        0x91 => machine.subtract_from_accumulator(Intel8080Register::C),
        0x92 => machine.subtract_from_accumulator(Intel8080Register::D),
        0x93 => machine.subtract_from_accumulator(Intel8080Register::E),
        0x94 => machine.subtract_from_accumulator(Intel8080Register::H),
        0x95 => machine.subtract_from_accumulator(Intel8080Register::L),
        0x96 => machine.subtract_from_accumulator(Intel8080Register::M),
        0x97 => machine.subtract_from_accumulator(Intel8080Register::A),
        0x98 => machine.subtract_from_accumulator_with_borrow(Intel8080Register::B),
        0x99 => machine.subtract_from_accumulator_with_borrow(Intel8080Register::C),
        0x9A => machine.subtract_from_accumulator_with_borrow(Intel8080Register::D),
        0x9B => machine.subtract_from_accumulator_with_borrow(Intel8080Register::E),
        0x9C => machine.subtract_from_accumulator_with_borrow(Intel8080Register::H),
        0x9D => machine.subtract_from_accumulator_with_borrow(Intel8080Register::L),
        0x9E => machine.subtract_from_accumulator_with_borrow(Intel8080Register::M),
        0x9F => machine.subtract_from_accumulator_with_borrow(Intel8080Register::A),
        0xA0 => machine.logical_and_with_accumulator(Intel8080Register::B),
        0xA1 => machine.logical_and_with_accumulator(Intel8080Register::C),
        0xA2 => machine.logical_and_with_accumulator(Intel8080Register::D),
        0xA3 => machine.logical_and_with_accumulator(Intel8080Register::E),
        0xA4 => machine.logical_and_with_accumulator(Intel8080Register::H),
        0xA5 => machine.logical_and_with_accumulator(Intel8080Register::L),
        0xA6 => machine.logical_and_with_accumulator(Intel8080Register::M),
        0xA7 => machine.logical_and_with_accumulator(Intel8080Register::A),
        0xA8 => machine.logical_exclusive_or_with_accumulator(Intel8080Register::B),
        0xA9 => machine.logical_exclusive_or_with_accumulator(Intel8080Register::C),
        0xAA => machine.logical_exclusive_or_with_accumulator(Intel8080Register::D),
        0xAB => machine.logical_exclusive_or_with_accumulator(Intel8080Register::E),
        0xAC => machine.logical_exclusive_or_with_accumulator(Intel8080Register::H),
        0xAD => machine.logical_exclusive_or_with_accumulator(Intel8080Register::L),
        0xAE => machine.logical_exclusive_or_with_accumulator(Intel8080Register::M),
        0xAF => machine.logical_exclusive_or_with_accumulator(Intel8080Register::A),
        0xB0 => machine.logical_or_with_accumulator(Intel8080Register::B),
        0xB1 => machine.logical_or_with_accumulator(Intel8080Register::C),
        0xB2 => machine.logical_or_with_accumulator(Intel8080Register::D),
        0xB3 => machine.logical_or_with_accumulator(Intel8080Register::E),
        0xB4 => machine.logical_or_with_accumulator(Intel8080Register::H),
        0xB5 => machine.logical_or_with_accumulator(Intel8080Register::L),
        0xB6 => machine.logical_or_with_accumulator(Intel8080Register::M),
        0xB7 => machine.logical_or_with_accumulator(Intel8080Register::A),
        0xB8 => machine.compare_with_accumulator(Intel8080Register::B),
        0xB9 => machine.compare_with_accumulator(Intel8080Register::C),
        0xBA => machine.compare_with_accumulator(Intel8080Register::D),
        0xBB => machine.compare_with_accumulator(Intel8080Register::E),
        0xBC => machine.compare_with_accumulator(Intel8080Register::H),
        0xBD => machine.compare_with_accumulator(Intel8080Register::L),
        0xBE => machine.compare_with_accumulator(Intel8080Register::M),
        0xBF => machine.compare_with_accumulator(Intel8080Register::A),
        0xC0 => machine.return_if_not_zero(),
        0xC1 => machine.pop_data_off_stack(Intel8080Register::B),
        0xC2 => machine.jump_if_not_zero(read_u16(&mut stream).unwrap()),
        0xC3 => machine.jump(read_u16(&mut stream).unwrap()),
        0xC4 => machine.call_if_not_zero(read_u16(&mut stream).unwrap()),
        0xC5 => machine.push_data_onto_stack(Intel8080Register::B),
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
        0xD1 => machine.pop_data_off_stack(Intel8080Register::D),
        0xD2 => machine.jump_if_no_carry(read_u16(&mut stream).unwrap()),
        0xD3 => machine.output(read_u8(&mut stream).unwrap()),
        0xD4 => machine.call_if_no_carry(read_u16(&mut stream).unwrap()),
        0xD5 => machine.push_data_onto_stack(Intel8080Register::D),
        0xD6 => machine.subtract_immediate_from_accumulator(read_u8(&mut stream).unwrap()),
        0xD7 => machine.restart(2 as u8),
        0xD8 => machine.return_if_carry(),
        0xDA => machine.jump_if_carry(read_u16(&mut stream).unwrap()),
        0xDB => machine.input(read_u8(&mut stream).unwrap()),
        0xDC => machine.call_if_carry(read_u16(&mut stream).unwrap()),
        0xDE => machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).unwrap()),
        0xDF => machine.restart(3 as u8),
        0xE0 => machine.return_if_parity_odd(),
        0xE1 => machine.pop_data_off_stack(Intel8080Register::H),
        0xE2 => machine.jump_if_parity_odd(read_u16(&mut stream).unwrap()),
        0xE3 => machine.exchange_stack(),
        0xE4 => machine.call_if_parity_odd(read_u16(&mut stream).unwrap()),
        0xE5 => machine.push_data_onto_stack(Intel8080Register::H),
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
        0xF1 => machine.pop_data_off_stack(Intel8080Register::PSW),
        0xF2 => machine.jump_if_positive(read_u16(&mut stream).unwrap()),
        0xF3 => machine.disable_interrupts(),
        0xF4 => machine.call_if_plus(read_u16(&mut stream).unwrap()),
        0xF5 => machine.push_data_onto_stack(Intel8080Register::PSW),
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

pub fn get_intel8080_instruction(
    original_stream: &[u8]) -> InstructionOption<Vec<u8>>
{
    let mut stream = original_stream;
    let size = match read_u8(&mut stream).unwrap() {
        0x00 =>         1,
        0x01 =>         3,
        0x02 =>         1,
        0x03 =>         1,
        0x04 =>         1,
        0x05 =>         1,
        0x06 =>         2,
        0x07 =>         1,
        0x09 =>         1,
        0x0A =>         1,
        0x0B =>         1,
        0x0C =>         1,
        0x0D =>         1,
        0x0E =>         2,
        0x0F =>         1,
        0x11 =>         3,
        0x12 =>         1,
        0x13 =>         1,
        0x14 =>         1,
        0x15 =>         1,
        0x16 =>         2,
        0x17 =>         1,
        0x19 =>         1,
        0x1A =>         1,
        0x1B =>         1,
        0x1C =>         1,
        0x1D =>         1,
        0x1E =>         2,
        0x1F =>         1,
        0x20 =>         1,
        0x21 =>         3,
        0x22 =>         3,
        0x23 =>         1,
        0x24 =>         1,
        0x25 =>         1,
        0x26 =>         2,
        0x27 =>         1,
        0x29 =>         1,
        0x2A =>         3,
        0x2B =>         1,
        0x2C =>         1,
        0x2D =>         1,
        0x2E =>         2,
        0x2F =>         1,
        0x30 =>         1,
        0x31 =>         3,
        0x32 =>         3,
        0x33 =>         1,
        0x34 =>         1,
        0x35 =>         1,
        0x36 =>         2,
        0x37 =>         1,
        0x39 =>         1,
        0x3A =>         3,
        0x3B =>         1,
        0x3C =>         1,
        0x3D =>         1,
        0x3E =>         2,
        0x3F =>         1,
        0x40 =>         1,
        0x41 =>         1,
        0x42 =>         1,
        0x43 =>         1,
        0x44 =>         1,
        0x45 =>         1,
        0x46 =>         1,
        0x47 =>         1,
        0x48 =>         1,
        0x49 =>         1,
        0x4A =>         1,
        0x4B =>         1,
        0x4C =>         1,
        0x4D =>         1,
        0x4E =>         1,
        0x4F =>         1,
        0x50 =>         1,
        0x51 =>         1,
        0x52 =>         1,
        0x53 =>         1,
        0x54 =>         1,
        0x55 =>         1,
        0x56 =>         1,
        0x57 =>         1,
        0x58 =>         1,
        0x59 =>         1,
        0x5A =>         1,
        0x5B =>         1,
        0x5C =>         1,
        0x5D =>         1,
        0x5E =>         1,
        0x5F =>         1,
        0x60 =>         1,
        0x61 =>         1,
        0x62 =>         1,
        0x63 =>         1,
        0x64 =>         1,
        0x65 =>         1,
        0x66 =>         1,
        0x67 =>         1,
        0x68 =>         1,
        0x69 =>         1,
        0x6A =>         1,
        0x6B =>         1,
        0x6C =>         1,
        0x6D =>         1,
        0x6E =>         1,
        0x6F =>         1,
        0x70 =>         1,
        0x71 =>         1,
        0x72 =>         1,
        0x73 =>         1,
        0x74 =>         1,
        0x75 =>         1,
        0x76 =>         1,
        0x77 =>         1,
        0x78 =>         1,
        0x79 =>         1,
        0x7A =>         1,
        0x7B =>         1,
        0x7C =>         1,
        0x7D =>         1,
        0x7E =>         1,
        0x7F =>         1,
        0x80 =>         1,
        0x81 =>         1,
        0x82 =>         1,
        0x83 =>         1,
        0x84 =>         1,
        0x85 =>         1,
        0x86 =>         1,
        0x87 =>         1,
        0x88 =>         1,
        0x89 =>         1,
        0x8A =>         1,
        0x8B =>         1,
        0x8C =>         1,
        0x8D =>         1,
        0x8E =>         1,
        0x8F =>         1,
        0x90 =>         1,
        0x91 =>         1,
        0x92 =>         1,
        0x93 =>         1,
        0x94 =>         1,
        0x95 =>         1,
        0x96 =>         1,
        0x97 =>         1,
        0x98 =>         1,
        0x99 =>         1,
        0x9A =>         1,
        0x9B =>         1,
        0x9C =>         1,
        0x9D =>         1,
        0x9E =>         1,
        0x9F =>         1,
        0xA0 =>         1,
        0xA1 =>         1,
        0xA2 =>         1,
        0xA3 =>         1,
        0xA4 =>         1,
        0xA5 =>         1,
        0xA6 =>         1,
        0xA7 =>         1,
        0xA8 =>         1,
        0xA9 =>         1,
        0xAA =>         1,
        0xAB =>         1,
        0xAC =>         1,
        0xAD =>         1,
        0xAE =>         1,
        0xAF =>         1,
        0xB0 =>         1,
        0xB1 =>         1,
        0xB2 =>         1,
        0xB3 =>         1,
        0xB4 =>         1,
        0xB5 =>         1,
        0xB6 =>         1,
        0xB7 =>         1,
        0xB8 =>         1,
        0xB9 =>         1,
        0xBA =>         1,
        0xBB =>         1,
        0xBC =>         1,
        0xBD =>         1,
        0xBE =>         1,
        0xBF =>         1,
        0xC0 =>         1,
        0xC1 =>         1,
        0xC2 =>         3,
        0xC3 =>         3,
        0xC4 =>         3,
        0xC5 =>         1,
        0xC6 =>         2,
        0xC7 =>         1,
        0xC8 =>         1,
        0xC9 =>         1,
        0xCA =>         3,
        0xCC =>         3,
        0xCD =>         3,
        0xCE =>         2,
        0xCF =>         1,
        0xD0 =>         1,
        0xD1 =>         1,
        0xD2 =>         3,
        0xD3 =>         2,
        0xD4 =>         3,
        0xD5 =>         1,
        0xD6 =>         2,
        0xD7 =>         1,
        0xD8 =>         1,
        0xDA =>         3,
        0xDB =>         2,
        0xDC =>         3,
        0xDE =>         2,
        0xDF =>         1,
        0xE0 =>         1,
        0xE1 =>         1,
        0xE2 =>         3,
        0xE3 =>         1,
        0xE4 =>         3,
        0xE5 =>         1,
        0xE6 =>         2,
        0xE7 =>         1,
        0xE8 =>         1,
        0xE9 =>         1,
        0xEA =>         3,
        0xEB =>         1,
        0xEC =>         3,
        0xEE =>         2,
        0xEF =>         1,
        0xF0 =>         1,
        0xF1 =>         1,
        0xF2 =>         3,
        0xF3 =>         1,
        0xF4 =>         3,
        0xF5 =>         1,
        0xF6 =>         2,
        0xF7 =>         1,
        0xF8 =>         1,
        0xF9 =>         1,
        0xFA =>         3,
        0xFB =>         1,
        0xFC =>         3,
        0xFE =>         2,
        0xFF =>         1,
        _ => return NoInstruction
    };

    let mut instruction = vec![];
    instruction.resize(size, 0);
    instruction.clone_from_slice(&original_stream[0..size]);
    return SomeInstruction(instruction);
}

impl<'a> Intel8080InstructionSet for Intel8080InstructionPrinter<'a> {
    fn return_if_not_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADI", data1);
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "POP", register1);
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADD", register1);
    }
    fn jump_if_parity_even(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPE", address1);
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CZ", address1);
    }
    fn double_add(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DAD", register1);
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ORI", data1);
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CC", address1);
    }
    fn jump(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JMP", address1);
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SUB", register1);
    }
    fn rim(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RIM");
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SBI", data1);
    }
    fn call_if_parity_even(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPE", address1);
    }
    fn jump_if_positive(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JP", address1);
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "XRA", register1);
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MOV", register1, register2);
    }
    fn halt(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn set_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "STC");
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "CMP", register1);
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNZ", address1);
    }
    fn call_if_parity_odd(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPO", address1);
    }
    fn return_if_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RAL");
    }
    fn disable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "DI");
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "SPHL");
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ANA", register1);
    }
    fn load_h_and_l_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LHLD", address1);
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "XRI", data1);
    }
    fn call(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CALL", address1);
    }
    fn enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "EI");
    }
    fn load_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "LDAX", register1);
    }
    fn input(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "IN", data1);
    }
    fn jump_if_parity_odd(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPO", address1);
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "INX", register1);
    }
    fn return_if_no_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ORA", register1);
    }
    fn exchange_registers(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "XCHG");
    }
    fn rotate_accumulator_right(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RRC");
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNC", address1);
    }
    fn return_if_parity_even(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RPE");
    }
    fn no_operation(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "NOP");
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ACI", data1);
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ANI", data1);
    }
    fn call_if_plus(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CP", address1);
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "INR", register1);
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "CPI", data1);
    }
    fn load_program_counter(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "PCHL");
    }
    fn return_if_minus(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RM");
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JC", address1);
    }
    fn call_if_minus(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CM", address1);
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16)
    {
        self.error = write!(self.stream_out, "{:04} {:?} #${:02x}", "LXI", register1, data2);
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8)
    {
        self.error = write!(self.stream_out, "{:04} {:?} #${:02x}", "MVI", register1, data2);
    }
    fn return_if_plus(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RP");
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} {}", "RST", implicit_data1);
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNZ", address1);
    }
    fn jump_if_minus(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JM", address1);
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCR", register1);
    }
    fn output(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "OUT", data1);
    }
    fn store_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "STAX", register1);
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADC", register1);
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JZ", address1);
    }
    fn complement_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CMA");
    }
    fn return_if_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn return_if_parity_odd(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RPO");
    }
    fn return_unconditionally(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RET");
    }
    fn store_h_and_l_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SHLD", address1);
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SBB", register1);
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SUI", data1);
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "PUSH", register1);
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNC", address1);
    }
    fn sim(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "SIM");
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCX", register1);
    }
    fn complement_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CMC");
    }
    fn rotate_accumulator_left(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RLC");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDA", address1);
    }
    fn exchange_stack(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "XTHL");
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RAR");
    }
}
