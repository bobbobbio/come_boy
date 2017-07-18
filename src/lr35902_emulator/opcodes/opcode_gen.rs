use emulator_common::Intel8080Register;
use emulator_common::InstructionOption;
use emulator_common::InstructionOption::*;
use lr35902_emulator::opcodes::LR35902InstructionPrinter;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait LR35902InstructionSet {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Intel8080Register);
    fn load_sp_from_h_and_l(&mut self);
    fn shift_register_right(&mut self, register1: Intel8080Register);
    fn double_add(&mut self, register1: Intel8080Register);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn no_operation(&mut self);
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn enable_interrupts(&mut self);
    fn return_if_zero(&mut self);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn rotate_accumulator_right(&mut self);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register);
    fn halt(&mut self);
    fn return_and_enable_interrupts(&mut self);
    fn set_bit(&mut self, implicit_data1: u8, register2: Intel8080Register);
    fn rotate_register_right(&mut self, register1: Intel8080Register);
    fn shift_register_right_signed(&mut self, register1: Intel8080Register);
    fn compare_with_accumulator(&mut self, register1: Intel8080Register);
    fn restart(&mut self, implicit_data1: u8);
    fn jump_relative_if_not_zero(&mut self, data1: u8);
    fn rotate_register_left(&mut self, register1: Intel8080Register);
    fn decrement_register_pair(&mut self, register1: Intel8080Register);
    fn complement_carry(&mut self);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn return_if_not_zero(&mut self);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn shift_register_left(&mut self, register1: Intel8080Register);
    fn jump(&mut self, address1: u16);
    fn call_if_not_zero(&mut self, address1: u16);
    fn store_sp_direct(&mut self, address1: u16);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn move_and_decrement_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn jump_relative_if_no_carry(&mut self, data1: u8);
    fn return_unconditionally(&mut self);
    fn load_accumulator_one_byte(&mut self);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_relative_if_carry(&mut self, data1: u8);
    fn call_if_carry(&mut self, address1: u16);
    fn test_bit(&mut self, implicit_data1: u8, register2: Intel8080Register);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn store_accumulator_direct_one_byte(&mut self, data1: u8);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn halt_until_button_press(&mut self);
    fn jump_relative(&mut self, data1: u8);
    fn store_accumulator_one_byte(&mut self);
    fn set_carry(&mut self);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn call(&mut self, address1: u16);
    fn return_if_no_carry(&mut self);
    fn call_if_zero(&mut self, address1: u16);
    fn load_accumulator_direct_one_byte(&mut self, data1: u8);
    fn jump_if_carry(&mut self, address1: u16);
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn swap_register(&mut self, register1: Intel8080Register);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn load_program_counter(&mut self);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn store_sp_plus_immediate(&mut self, data1: u8);
    fn complement_accumulator(&mut self);
    fn move_and_increment_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn add_immediate_to_sp(&mut self, data1: u8);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn disable_interrupts(&mut self);
    fn compare_immediate_with_accumulator(&mut self, data1: u8);
    fn decimal_adjust_accumulator(&mut self);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn call_if_no_carry(&mut self, address1: u16);
    fn jump_relative_if_zero(&mut self, data1: u8);
    fn return_if_carry(&mut self);
    fn jump_if_zero(&mut self, address1: u16);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
    fn rotate_accumulator_left(&mut self);
}

pub fn dispatch_lr35902_instruction<I: LR35902InstructionSet>(
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
        0x08 => machine.store_sp_direct(read_u16(&mut stream).unwrap()),
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
        0x18 => machine.jump_relative(read_u8(&mut stream).unwrap()),
        0x19 => machine.double_add(Intel8080Register::D),
        0x1A => machine.load_accumulator(Intel8080Register::D),
        0x1B => machine.decrement_register_pair(Intel8080Register::D),
        0x1C => machine.increment_register_or_memory(Intel8080Register::E),
        0x1D => machine.decrement_register_or_memory(Intel8080Register::E),
        0x1E => machine.move_immediate_data(Intel8080Register::E, read_u8(&mut stream).unwrap()),
        0x1F => machine.rotate_accumulator_right_through_carry(),
        0x20 => machine.jump_relative_if_not_zero(read_u8(&mut stream).unwrap()),
        0x21 => machine.load_register_pair_immediate(Intel8080Register::H, read_u16(&mut stream).unwrap()),
        0x22 => machine.move_and_increment_hl(Intel8080Register::M, Intel8080Register::A),
        0x23 => machine.increment_register_pair(Intel8080Register::H),
        0x24 => machine.increment_register_or_memory(Intel8080Register::H),
        0x25 => machine.decrement_register_or_memory(Intel8080Register::H),
        0x26 => machine.move_immediate_data(Intel8080Register::H, read_u8(&mut stream).unwrap()),
        0x27 => machine.decimal_adjust_accumulator(),
        0x28 => machine.jump_relative_if_zero(read_u8(&mut stream).unwrap()),
        0x29 => machine.double_add(Intel8080Register::H),
        0x2A => machine.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M),
        0x2B => machine.decrement_register_pair(Intel8080Register::H),
        0x2C => machine.increment_register_or_memory(Intel8080Register::L),
        0x2D => machine.decrement_register_or_memory(Intel8080Register::L),
        0x2E => machine.move_immediate_data(Intel8080Register::L, read_u8(&mut stream).unwrap()),
        0x2F => machine.complement_accumulator(),
        0x30 => machine.jump_relative_if_no_carry(read_u8(&mut stream).unwrap()),
        0x31 => machine.load_register_pair_immediate(Intel8080Register::SP, read_u16(&mut stream).unwrap()),
        0x32 => machine.move_and_decrement_hl(Intel8080Register::M, Intel8080Register::A),
        0x33 => machine.increment_register_pair(Intel8080Register::SP),
        0x34 => machine.increment_register_or_memory(Intel8080Register::M),
        0x35 => machine.decrement_register_or_memory(Intel8080Register::M),
        0x36 => machine.move_immediate_data(Intel8080Register::M, read_u8(&mut stream).unwrap()),
        0x37 => machine.set_carry(),
        0x38 => machine.jump_relative_if_carry(read_u8(&mut stream).unwrap()),
        0x39 => machine.double_add(Intel8080Register::SP),
        0x3A => machine.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M),
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
        0xD4 => machine.call_if_no_carry(read_u16(&mut stream).unwrap()),
        0xD5 => machine.push_data_onto_stack(Intel8080Register::D),
        0xD6 => machine.subtract_immediate_from_accumulator(read_u8(&mut stream).unwrap()),
        0xD7 => machine.restart(2 as u8),
        0xD8 => machine.return_if_carry(),
        0xD9 => machine.return_and_enable_interrupts(),
        0xDA => machine.jump_if_carry(read_u16(&mut stream).unwrap()),
        0xDC => machine.call_if_carry(read_u16(&mut stream).unwrap()),
        0xDE => machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).unwrap()),
        0xDF => machine.restart(3 as u8),
        0xE0 => machine.store_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()),
        0xE1 => machine.pop_data_off_stack(Intel8080Register::H),
        0xE2 => machine.store_accumulator_one_byte(),
        0xE5 => machine.push_data_onto_stack(Intel8080Register::H),
        0xE6 => machine.and_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xE7 => machine.restart(4 as u8),
        0xE8 => machine.add_immediate_to_sp(read_u8(&mut stream).unwrap()),
        0xE9 => machine.load_program_counter(),
        0xEA => machine.store_accumulator_direct(read_u16(&mut stream).unwrap()),
        0xEE => machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xEF => machine.restart(5 as u8),
        0xF0 => machine.load_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()),
        0xF1 => machine.pop_data_off_stack(Intel8080Register::PSW),
        0xF2 => machine.load_accumulator_one_byte(),
        0xF3 => machine.disable_interrupts(),
        0xF5 => machine.push_data_onto_stack(Intel8080Register::PSW),
        0xF6 => machine.or_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xF7 => machine.restart(6 as u8),
        0xF8 => machine.store_sp_plus_immediate(read_u8(&mut stream).unwrap()),
        0xF9 => machine.load_sp_from_h_and_l(),
        0xFA => machine.load_accumulator_direct(read_u16(&mut stream).unwrap()),
        0xFB => machine.enable_interrupts(),
        0xFE => machine.compare_immediate_with_accumulator(read_u8(&mut stream).unwrap()),
        0xFF => machine.restart(7 as u8),
        0x10 => match (0x10 as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0x1000 => machine.halt_until_button_press(),
            v => panic!("Unknown opcode {}", v)
        },
        0xCB => match (0xCB as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0xCB00 => machine.rotate_register_left(Intel8080Register::B),
            0xCB01 => machine.rotate_register_left(Intel8080Register::C),
            0xCB02 => machine.rotate_register_left(Intel8080Register::D),
            0xCB03 => machine.rotate_register_left(Intel8080Register::E),
            0xCB04 => machine.rotate_register_left(Intel8080Register::H),
            0xCB05 => machine.rotate_register_left(Intel8080Register::L),
            0xCB06 => machine.rotate_register_left(Intel8080Register::M),
            0xCB07 => machine.rotate_register_left(Intel8080Register::A),
            0xCB08 => machine.rotate_register_right(Intel8080Register::B),
            0xCB09 => machine.rotate_register_right(Intel8080Register::C),
            0xCB0A => machine.rotate_register_right(Intel8080Register::D),
            0xCB0B => machine.rotate_register_right(Intel8080Register::E),
            0xCB0C => machine.rotate_register_right(Intel8080Register::H),
            0xCB0D => machine.rotate_register_right(Intel8080Register::L),
            0xCB0E => machine.rotate_register_right(Intel8080Register::M),
            0xCB0F => machine.rotate_register_right(Intel8080Register::A),
            0xCB10 => machine.rotate_register_left_through_carry(Intel8080Register::B),
            0xCB11 => machine.rotate_register_left_through_carry(Intel8080Register::C),
            0xCB12 => machine.rotate_register_left_through_carry(Intel8080Register::D),
            0xCB13 => machine.rotate_register_left_through_carry(Intel8080Register::E),
            0xCB14 => machine.rotate_register_left_through_carry(Intel8080Register::H),
            0xCB15 => machine.rotate_register_left_through_carry(Intel8080Register::L),
            0xCB16 => machine.rotate_register_left_through_carry(Intel8080Register::M),
            0xCB17 => machine.rotate_register_left_through_carry(Intel8080Register::A),
            0xCB18 => machine.rotate_register_right_through_carry(Intel8080Register::B),
            0xCB19 => machine.rotate_register_right_through_carry(Intel8080Register::C),
            0xCB1A => machine.rotate_register_right_through_carry(Intel8080Register::D),
            0xCB1B => machine.rotate_register_right_through_carry(Intel8080Register::E),
            0xCB1C => machine.rotate_register_right_through_carry(Intel8080Register::H),
            0xCB1D => machine.rotate_register_right_through_carry(Intel8080Register::L),
            0xCB1E => machine.rotate_register_right_through_carry(Intel8080Register::M),
            0xCB1F => machine.rotate_register_right_through_carry(Intel8080Register::A),
            0xCB20 => machine.shift_register_left(Intel8080Register::B),
            0xCB21 => machine.shift_register_left(Intel8080Register::C),
            0xCB22 => machine.shift_register_left(Intel8080Register::D),
            0xCB23 => machine.shift_register_left(Intel8080Register::E),
            0xCB24 => machine.shift_register_left(Intel8080Register::H),
            0xCB25 => machine.shift_register_left(Intel8080Register::L),
            0xCB26 => machine.shift_register_left(Intel8080Register::M),
            0xCB27 => machine.shift_register_left(Intel8080Register::A),
            0xCB28 => machine.shift_register_right_signed(Intel8080Register::B),
            0xCB29 => machine.shift_register_right_signed(Intel8080Register::C),
            0xCB2A => machine.shift_register_right_signed(Intel8080Register::D),
            0xCB2B => machine.shift_register_right_signed(Intel8080Register::E),
            0xCB2C => machine.shift_register_right_signed(Intel8080Register::H),
            0xCB2D => machine.shift_register_right_signed(Intel8080Register::L),
            0xCB2E => machine.shift_register_right_signed(Intel8080Register::M),
            0xCB2F => machine.shift_register_right_signed(Intel8080Register::A),
            0xCB30 => machine.swap_register(Intel8080Register::B),
            0xCB31 => machine.swap_register(Intel8080Register::C),
            0xCB32 => machine.swap_register(Intel8080Register::D),
            0xCB33 => machine.swap_register(Intel8080Register::E),
            0xCB34 => machine.swap_register(Intel8080Register::H),
            0xCB35 => machine.swap_register(Intel8080Register::L),
            0xCB36 => machine.swap_register(Intel8080Register::M),
            0xCB37 => machine.swap_register(Intel8080Register::A),
            0xCB38 => machine.shift_register_right(Intel8080Register::B),
            0xCB39 => machine.shift_register_right(Intel8080Register::C),
            0xCB3A => machine.shift_register_right(Intel8080Register::D),
            0xCB3B => machine.shift_register_right(Intel8080Register::E),
            0xCB3C => machine.shift_register_right(Intel8080Register::H),
            0xCB3D => machine.shift_register_right(Intel8080Register::L),
            0xCB3E => machine.shift_register_right(Intel8080Register::M),
            0xCB3F => machine.shift_register_right(Intel8080Register::A),
            0xCB40 => machine.test_bit(0 as u8, Intel8080Register::B),
            0xCB41 => machine.test_bit(0 as u8, Intel8080Register::C),
            0xCB42 => machine.test_bit(0 as u8, Intel8080Register::D),
            0xCB43 => machine.test_bit(0 as u8, Intel8080Register::E),
            0xCB44 => machine.test_bit(0 as u8, Intel8080Register::H),
            0xCB45 => machine.test_bit(0 as u8, Intel8080Register::L),
            0xCB46 => machine.test_bit(0 as u8, Intel8080Register::M),
            0xCB47 => machine.test_bit(0 as u8, Intel8080Register::A),
            0xCB48 => machine.test_bit(1 as u8, Intel8080Register::B),
            0xCB49 => machine.test_bit(1 as u8, Intel8080Register::C),
            0xCB4A => machine.test_bit(1 as u8, Intel8080Register::D),
            0xCB4B => machine.test_bit(1 as u8, Intel8080Register::E),
            0xCB4C => machine.test_bit(1 as u8, Intel8080Register::H),
            0xCB4D => machine.test_bit(1 as u8, Intel8080Register::L),
            0xCB4E => machine.test_bit(1 as u8, Intel8080Register::M),
            0xCB4F => machine.test_bit(1 as u8, Intel8080Register::A),
            0xCB50 => machine.test_bit(2 as u8, Intel8080Register::B),
            0xCB51 => machine.test_bit(2 as u8, Intel8080Register::C),
            0xCB52 => machine.test_bit(2 as u8, Intel8080Register::D),
            0xCB53 => machine.test_bit(2 as u8, Intel8080Register::E),
            0xCB54 => machine.test_bit(2 as u8, Intel8080Register::H),
            0xCB55 => machine.test_bit(2 as u8, Intel8080Register::L),
            0xCB56 => machine.test_bit(2 as u8, Intel8080Register::M),
            0xCB57 => machine.test_bit(2 as u8, Intel8080Register::A),
            0xCB58 => machine.test_bit(3 as u8, Intel8080Register::B),
            0xCB59 => machine.test_bit(3 as u8, Intel8080Register::C),
            0xCB5A => machine.test_bit(3 as u8, Intel8080Register::D),
            0xCB5B => machine.test_bit(3 as u8, Intel8080Register::E),
            0xCB5C => machine.test_bit(3 as u8, Intel8080Register::H),
            0xCB5D => machine.test_bit(3 as u8, Intel8080Register::L),
            0xCB5E => machine.test_bit(3 as u8, Intel8080Register::M),
            0xCB5F => machine.test_bit(3 as u8, Intel8080Register::A),
            0xCB60 => machine.test_bit(4 as u8, Intel8080Register::B),
            0xCB61 => machine.test_bit(4 as u8, Intel8080Register::C),
            0xCB62 => machine.test_bit(4 as u8, Intel8080Register::D),
            0xCB63 => machine.test_bit(4 as u8, Intel8080Register::E),
            0xCB64 => machine.test_bit(4 as u8, Intel8080Register::H),
            0xCB65 => machine.test_bit(4 as u8, Intel8080Register::L),
            0xCB66 => machine.test_bit(4 as u8, Intel8080Register::M),
            0xCB67 => machine.test_bit(4 as u8, Intel8080Register::A),
            0xCB68 => machine.test_bit(5 as u8, Intel8080Register::B),
            0xCB69 => machine.test_bit(5 as u8, Intel8080Register::C),
            0xCB6A => machine.test_bit(5 as u8, Intel8080Register::D),
            0xCB6B => machine.test_bit(5 as u8, Intel8080Register::E),
            0xCB6C => machine.test_bit(5 as u8, Intel8080Register::H),
            0xCB6D => machine.test_bit(5 as u8, Intel8080Register::L),
            0xCB6E => machine.test_bit(5 as u8, Intel8080Register::M),
            0xCB6F => machine.test_bit(5 as u8, Intel8080Register::A),
            0xCB70 => machine.test_bit(6 as u8, Intel8080Register::B),
            0xCB71 => machine.test_bit(6 as u8, Intel8080Register::C),
            0xCB72 => machine.test_bit(6 as u8, Intel8080Register::D),
            0xCB73 => machine.test_bit(6 as u8, Intel8080Register::E),
            0xCB74 => machine.test_bit(6 as u8, Intel8080Register::H),
            0xCB75 => machine.test_bit(6 as u8, Intel8080Register::L),
            0xCB76 => machine.test_bit(6 as u8, Intel8080Register::M),
            0xCB77 => machine.test_bit(6 as u8, Intel8080Register::A),
            0xCB78 => machine.test_bit(7 as u8, Intel8080Register::B),
            0xCB79 => machine.test_bit(7 as u8, Intel8080Register::C),
            0xCB7A => machine.test_bit(7 as u8, Intel8080Register::D),
            0xCB7B => machine.test_bit(7 as u8, Intel8080Register::E),
            0xCB7C => machine.test_bit(7 as u8, Intel8080Register::H),
            0xCB7D => machine.test_bit(7 as u8, Intel8080Register::L),
            0xCB7E => machine.test_bit(7 as u8, Intel8080Register::M),
            0xCB7F => machine.test_bit(7 as u8, Intel8080Register::A),
            0xCB80 => machine.reset_bit(0 as u8, Intel8080Register::B),
            0xCB81 => machine.reset_bit(0 as u8, Intel8080Register::C),
            0xCB82 => machine.reset_bit(0 as u8, Intel8080Register::D),
            0xCB83 => machine.reset_bit(0 as u8, Intel8080Register::E),
            0xCB84 => machine.reset_bit(0 as u8, Intel8080Register::H),
            0xCB85 => machine.reset_bit(0 as u8, Intel8080Register::L),
            0xCB86 => machine.reset_bit(0 as u8, Intel8080Register::M),
            0xCB87 => machine.reset_bit(0 as u8, Intel8080Register::A),
            0xCB88 => machine.reset_bit(1 as u8, Intel8080Register::B),
            0xCB89 => machine.reset_bit(1 as u8, Intel8080Register::C),
            0xCB8A => machine.reset_bit(1 as u8, Intel8080Register::D),
            0xCB8B => machine.reset_bit(1 as u8, Intel8080Register::E),
            0xCB8C => machine.reset_bit(1 as u8, Intel8080Register::H),
            0xCB8D => machine.reset_bit(1 as u8, Intel8080Register::L),
            0xCB8E => machine.reset_bit(1 as u8, Intel8080Register::M),
            0xCB8F => machine.reset_bit(1 as u8, Intel8080Register::A),
            0xCB90 => machine.reset_bit(2 as u8, Intel8080Register::B),
            0xCB91 => machine.reset_bit(2 as u8, Intel8080Register::C),
            0xCB92 => machine.reset_bit(2 as u8, Intel8080Register::D),
            0xCB93 => machine.reset_bit(2 as u8, Intel8080Register::E),
            0xCB94 => machine.reset_bit(2 as u8, Intel8080Register::H),
            0xCB95 => machine.reset_bit(2 as u8, Intel8080Register::L),
            0xCB96 => machine.reset_bit(2 as u8, Intel8080Register::M),
            0xCB97 => machine.reset_bit(2 as u8, Intel8080Register::A),
            0xCB98 => machine.reset_bit(3 as u8, Intel8080Register::B),
            0xCB99 => machine.reset_bit(3 as u8, Intel8080Register::C),
            0xCB9A => machine.reset_bit(3 as u8, Intel8080Register::D),
            0xCB9B => machine.reset_bit(3 as u8, Intel8080Register::E),
            0xCB9C => machine.reset_bit(3 as u8, Intel8080Register::H),
            0xCB9D => machine.reset_bit(3 as u8, Intel8080Register::L),
            0xCB9E => machine.reset_bit(3 as u8, Intel8080Register::M),
            0xCB9F => machine.reset_bit(3 as u8, Intel8080Register::A),
            0xCBA0 => machine.reset_bit(4 as u8, Intel8080Register::B),
            0xCBA1 => machine.reset_bit(4 as u8, Intel8080Register::C),
            0xCBA2 => machine.reset_bit(4 as u8, Intel8080Register::D),
            0xCBA3 => machine.reset_bit(4 as u8, Intel8080Register::E),
            0xCBA4 => machine.reset_bit(4 as u8, Intel8080Register::H),
            0xCBA5 => machine.reset_bit(4 as u8, Intel8080Register::L),
            0xCBA6 => machine.reset_bit(4 as u8, Intel8080Register::M),
            0xCBA7 => machine.reset_bit(4 as u8, Intel8080Register::A),
            0xCBA8 => machine.reset_bit(5 as u8, Intel8080Register::B),
            0xCBA9 => machine.reset_bit(5 as u8, Intel8080Register::C),
            0xCBAA => machine.reset_bit(5 as u8, Intel8080Register::D),
            0xCBAB => machine.reset_bit(5 as u8, Intel8080Register::E),
            0xCBAC => machine.reset_bit(5 as u8, Intel8080Register::H),
            0xCBAD => machine.reset_bit(5 as u8, Intel8080Register::L),
            0xCBAE => machine.reset_bit(5 as u8, Intel8080Register::M),
            0xCBAF => machine.reset_bit(5 as u8, Intel8080Register::A),
            0xCBB0 => machine.reset_bit(6 as u8, Intel8080Register::B),
            0xCBB1 => machine.reset_bit(6 as u8, Intel8080Register::C),
            0xCBB2 => machine.reset_bit(6 as u8, Intel8080Register::D),
            0xCBB3 => machine.reset_bit(6 as u8, Intel8080Register::E),
            0xCBB4 => machine.reset_bit(6 as u8, Intel8080Register::H),
            0xCBB5 => machine.reset_bit(6 as u8, Intel8080Register::L),
            0xCBB6 => machine.reset_bit(6 as u8, Intel8080Register::M),
            0xCBB7 => machine.reset_bit(6 as u8, Intel8080Register::A),
            0xCBB8 => machine.reset_bit(7 as u8, Intel8080Register::B),
            0xCBB9 => machine.reset_bit(7 as u8, Intel8080Register::C),
            0xCBBA => machine.reset_bit(7 as u8, Intel8080Register::D),
            0xCBBB => machine.reset_bit(7 as u8, Intel8080Register::E),
            0xCBBC => machine.reset_bit(7 as u8, Intel8080Register::H),
            0xCBBD => machine.reset_bit(7 as u8, Intel8080Register::L),
            0xCBBE => machine.reset_bit(7 as u8, Intel8080Register::M),
            0xCBBF => machine.reset_bit(7 as u8, Intel8080Register::A),
            0xCBC0 => machine.set_bit(0 as u8, Intel8080Register::B),
            0xCBC1 => machine.set_bit(0 as u8, Intel8080Register::C),
            0xCBC2 => machine.set_bit(0 as u8, Intel8080Register::D),
            0xCBC3 => machine.set_bit(0 as u8, Intel8080Register::E),
            0xCBC4 => machine.set_bit(0 as u8, Intel8080Register::H),
            0xCBC5 => machine.set_bit(0 as u8, Intel8080Register::L),
            0xCBC6 => machine.set_bit(0 as u8, Intel8080Register::M),
            0xCBC7 => machine.set_bit(0 as u8, Intel8080Register::A),
            0xCBC8 => machine.set_bit(1 as u8, Intel8080Register::B),
            0xCBC9 => machine.set_bit(1 as u8, Intel8080Register::C),
            0xCBCA => machine.set_bit(1 as u8, Intel8080Register::D),
            0xCBCB => machine.set_bit(1 as u8, Intel8080Register::E),
            0xCBCC => machine.set_bit(1 as u8, Intel8080Register::H),
            0xCBCD => machine.set_bit(1 as u8, Intel8080Register::L),
            0xCBCE => machine.set_bit(1 as u8, Intel8080Register::M),
            0xCBCF => machine.set_bit(1 as u8, Intel8080Register::A),
            0xCBD0 => machine.set_bit(2 as u8, Intel8080Register::B),
            0xCBD1 => machine.set_bit(2 as u8, Intel8080Register::C),
            0xCBD2 => machine.set_bit(2 as u8, Intel8080Register::D),
            0xCBD3 => machine.set_bit(2 as u8, Intel8080Register::E),
            0xCBD4 => machine.set_bit(2 as u8, Intel8080Register::H),
            0xCBD5 => machine.set_bit(2 as u8, Intel8080Register::L),
            0xCBD6 => machine.set_bit(2 as u8, Intel8080Register::M),
            0xCBD7 => machine.set_bit(2 as u8, Intel8080Register::A),
            0xCBD8 => machine.set_bit(3 as u8, Intel8080Register::B),
            0xCBD9 => machine.set_bit(3 as u8, Intel8080Register::C),
            0xCBDA => machine.set_bit(3 as u8, Intel8080Register::D),
            0xCBDB => machine.set_bit(3 as u8, Intel8080Register::E),
            0xCBDC => machine.set_bit(3 as u8, Intel8080Register::H),
            0xCBDD => machine.set_bit(3 as u8, Intel8080Register::L),
            0xCBDE => machine.set_bit(3 as u8, Intel8080Register::M),
            0xCBDF => machine.set_bit(3 as u8, Intel8080Register::A),
            0xCBE0 => machine.set_bit(4 as u8, Intel8080Register::B),
            0xCBE1 => machine.set_bit(4 as u8, Intel8080Register::C),
            0xCBE2 => machine.set_bit(4 as u8, Intel8080Register::D),
            0xCBE3 => machine.set_bit(4 as u8, Intel8080Register::E),
            0xCBE4 => machine.set_bit(4 as u8, Intel8080Register::H),
            0xCBE5 => machine.set_bit(4 as u8, Intel8080Register::L),
            0xCBE6 => machine.set_bit(4 as u8, Intel8080Register::M),
            0xCBE7 => machine.set_bit(4 as u8, Intel8080Register::A),
            0xCBE8 => machine.set_bit(5 as u8, Intel8080Register::B),
            0xCBE9 => machine.set_bit(5 as u8, Intel8080Register::C),
            0xCBEA => machine.set_bit(5 as u8, Intel8080Register::D),
            0xCBEB => machine.set_bit(5 as u8, Intel8080Register::E),
            0xCBEC => machine.set_bit(5 as u8, Intel8080Register::H),
            0xCBED => machine.set_bit(5 as u8, Intel8080Register::L),
            0xCBEE => machine.set_bit(5 as u8, Intel8080Register::M),
            0xCBEF => machine.set_bit(5 as u8, Intel8080Register::A),
            0xCBF0 => machine.set_bit(6 as u8, Intel8080Register::B),
            0xCBF1 => machine.set_bit(6 as u8, Intel8080Register::C),
            0xCBF2 => machine.set_bit(6 as u8, Intel8080Register::D),
            0xCBF3 => machine.set_bit(6 as u8, Intel8080Register::E),
            0xCBF4 => machine.set_bit(6 as u8, Intel8080Register::H),
            0xCBF5 => machine.set_bit(6 as u8, Intel8080Register::L),
            0xCBF6 => machine.set_bit(6 as u8, Intel8080Register::M),
            0xCBF7 => machine.set_bit(6 as u8, Intel8080Register::A),
            0xCBF8 => machine.set_bit(7 as u8, Intel8080Register::B),
            0xCBF9 => machine.set_bit(7 as u8, Intel8080Register::C),
            0xCBFA => machine.set_bit(7 as u8, Intel8080Register::D),
            0xCBFB => machine.set_bit(7 as u8, Intel8080Register::E),
            0xCBFC => machine.set_bit(7 as u8, Intel8080Register::H),
            0xCBFD => machine.set_bit(7 as u8, Intel8080Register::L),
            0xCBFE => machine.set_bit(7 as u8, Intel8080Register::M),
            0xCBFF => machine.set_bit(7 as u8, Intel8080Register::A),
            v => panic!("Unknown opcode {}", v)
        },
        v => panic!("Unknown opcode {}", v)
    };
}

pub fn get_lr35902_instruction(
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
        0x08 =>         3,
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
        0x18 =>         2,
        0x19 =>         1,
        0x1A =>         1,
        0x1B =>         1,
        0x1C =>         1,
        0x1D =>         1,
        0x1E =>         2,
        0x1F =>         1,
        0x20 =>         2,
        0x21 =>         3,
        0x22 =>         1,
        0x23 =>         1,
        0x24 =>         1,
        0x25 =>         1,
        0x26 =>         2,
        0x27 =>         1,
        0x28 =>         2,
        0x29 =>         1,
        0x2A =>         1,
        0x2B =>         1,
        0x2C =>         1,
        0x2D =>         1,
        0x2E =>         2,
        0x2F =>         1,
        0x30 =>         2,
        0x31 =>         3,
        0x32 =>         1,
        0x33 =>         1,
        0x34 =>         1,
        0x35 =>         1,
        0x36 =>         2,
        0x37 =>         1,
        0x38 =>         2,
        0x39 =>         1,
        0x3A =>         1,
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
        0xD4 =>         3,
        0xD5 =>         1,
        0xD6 =>         2,
        0xD7 =>         1,
        0xD8 =>         1,
        0xD9 =>         1,
        0xDA =>         3,
        0xDC =>         3,
        0xDE =>         2,
        0xDF =>         1,
        0xE0 =>         2,
        0xE1 =>         1,
        0xE2 =>         1,
        0xE5 =>         1,
        0xE6 =>         2,
        0xE7 =>         1,
        0xE8 =>         2,
        0xE9 =>         1,
        0xEA =>         3,
        0xEE =>         2,
        0xEF =>         1,
        0xF0 =>         2,
        0xF1 =>         1,
        0xF2 =>         1,
        0xF3 =>         1,
        0xF5 =>         1,
        0xF6 =>         2,
        0xF7 =>         1,
        0xF8 =>         2,
        0xF9 =>         1,
        0xFA =>         3,
        0xFB =>         1,
        0xFE =>         2,
        0xFF =>         1,
        0x10 => match (0x10 as u16) << 8 |
            match read_u8(&mut stream) { Ok(x) => x, _ => return NoInstruction } as u16{
            0x1000 =>             2,
            _ => return NoInstruction
        },
        0xCB => match (0xCB as u16) << 8 |
            match read_u8(&mut stream) { Ok(x) => x, _ => return NoInstruction } as u16{
            0xCB00 =>             2,
            0xCB01 =>             2,
            0xCB02 =>             2,
            0xCB03 =>             2,
            0xCB04 =>             2,
            0xCB05 =>             2,
            0xCB06 =>             2,
            0xCB07 =>             2,
            0xCB08 =>             2,
            0xCB09 =>             2,
            0xCB0A =>             2,
            0xCB0B =>             2,
            0xCB0C =>             2,
            0xCB0D =>             2,
            0xCB0E =>             2,
            0xCB0F =>             2,
            0xCB10 =>             2,
            0xCB11 =>             2,
            0xCB12 =>             2,
            0xCB13 =>             2,
            0xCB14 =>             2,
            0xCB15 =>             2,
            0xCB16 =>             2,
            0xCB17 =>             2,
            0xCB18 =>             2,
            0xCB19 =>             2,
            0xCB1A =>             2,
            0xCB1B =>             2,
            0xCB1C =>             2,
            0xCB1D =>             2,
            0xCB1E =>             2,
            0xCB1F =>             2,
            0xCB20 =>             2,
            0xCB21 =>             2,
            0xCB22 =>             2,
            0xCB23 =>             2,
            0xCB24 =>             2,
            0xCB25 =>             2,
            0xCB26 =>             2,
            0xCB27 =>             2,
            0xCB28 =>             2,
            0xCB29 =>             2,
            0xCB2A =>             2,
            0xCB2B =>             2,
            0xCB2C =>             2,
            0xCB2D =>             2,
            0xCB2E =>             2,
            0xCB2F =>             2,
            0xCB30 =>             2,
            0xCB31 =>             2,
            0xCB32 =>             2,
            0xCB33 =>             2,
            0xCB34 =>             2,
            0xCB35 =>             2,
            0xCB36 =>             2,
            0xCB37 =>             2,
            0xCB38 =>             2,
            0xCB39 =>             2,
            0xCB3A =>             2,
            0xCB3B =>             2,
            0xCB3C =>             2,
            0xCB3D =>             2,
            0xCB3E =>             2,
            0xCB3F =>             2,
            0xCB40 =>             2,
            0xCB41 =>             2,
            0xCB42 =>             2,
            0xCB43 =>             2,
            0xCB44 =>             2,
            0xCB45 =>             2,
            0xCB46 =>             2,
            0xCB47 =>             2,
            0xCB48 =>             2,
            0xCB49 =>             2,
            0xCB4A =>             2,
            0xCB4B =>             2,
            0xCB4C =>             2,
            0xCB4D =>             2,
            0xCB4E =>             2,
            0xCB4F =>             2,
            0xCB50 =>             2,
            0xCB51 =>             2,
            0xCB52 =>             2,
            0xCB53 =>             2,
            0xCB54 =>             2,
            0xCB55 =>             2,
            0xCB56 =>             2,
            0xCB57 =>             2,
            0xCB58 =>             2,
            0xCB59 =>             2,
            0xCB5A =>             2,
            0xCB5B =>             2,
            0xCB5C =>             2,
            0xCB5D =>             2,
            0xCB5E =>             2,
            0xCB5F =>             2,
            0xCB60 =>             2,
            0xCB61 =>             2,
            0xCB62 =>             2,
            0xCB63 =>             2,
            0xCB64 =>             2,
            0xCB65 =>             2,
            0xCB66 =>             2,
            0xCB67 =>             2,
            0xCB68 =>             2,
            0xCB69 =>             2,
            0xCB6A =>             2,
            0xCB6B =>             2,
            0xCB6C =>             2,
            0xCB6D =>             2,
            0xCB6E =>             2,
            0xCB6F =>             2,
            0xCB70 =>             2,
            0xCB71 =>             2,
            0xCB72 =>             2,
            0xCB73 =>             2,
            0xCB74 =>             2,
            0xCB75 =>             2,
            0xCB76 =>             2,
            0xCB77 =>             2,
            0xCB78 =>             2,
            0xCB79 =>             2,
            0xCB7A =>             2,
            0xCB7B =>             2,
            0xCB7C =>             2,
            0xCB7D =>             2,
            0xCB7E =>             2,
            0xCB7F =>             2,
            0xCB80 =>             2,
            0xCB81 =>             2,
            0xCB82 =>             2,
            0xCB83 =>             2,
            0xCB84 =>             2,
            0xCB85 =>             2,
            0xCB86 =>             2,
            0xCB87 =>             2,
            0xCB88 =>             2,
            0xCB89 =>             2,
            0xCB8A =>             2,
            0xCB8B =>             2,
            0xCB8C =>             2,
            0xCB8D =>             2,
            0xCB8E =>             2,
            0xCB8F =>             2,
            0xCB90 =>             2,
            0xCB91 =>             2,
            0xCB92 =>             2,
            0xCB93 =>             2,
            0xCB94 =>             2,
            0xCB95 =>             2,
            0xCB96 =>             2,
            0xCB97 =>             2,
            0xCB98 =>             2,
            0xCB99 =>             2,
            0xCB9A =>             2,
            0xCB9B =>             2,
            0xCB9C =>             2,
            0xCB9D =>             2,
            0xCB9E =>             2,
            0xCB9F =>             2,
            0xCBA0 =>             2,
            0xCBA1 =>             2,
            0xCBA2 =>             2,
            0xCBA3 =>             2,
            0xCBA4 =>             2,
            0xCBA5 =>             2,
            0xCBA6 =>             2,
            0xCBA7 =>             2,
            0xCBA8 =>             2,
            0xCBA9 =>             2,
            0xCBAA =>             2,
            0xCBAB =>             2,
            0xCBAC =>             2,
            0xCBAD =>             2,
            0xCBAE =>             2,
            0xCBAF =>             2,
            0xCBB0 =>             2,
            0xCBB1 =>             2,
            0xCBB2 =>             2,
            0xCBB3 =>             2,
            0xCBB4 =>             2,
            0xCBB5 =>             2,
            0xCBB6 =>             2,
            0xCBB7 =>             2,
            0xCBB8 =>             2,
            0xCBB9 =>             2,
            0xCBBA =>             2,
            0xCBBB =>             2,
            0xCBBC =>             2,
            0xCBBD =>             2,
            0xCBBE =>             2,
            0xCBBF =>             2,
            0xCBC0 =>             2,
            0xCBC1 =>             2,
            0xCBC2 =>             2,
            0xCBC3 =>             2,
            0xCBC4 =>             2,
            0xCBC5 =>             2,
            0xCBC6 =>             2,
            0xCBC7 =>             2,
            0xCBC8 =>             2,
            0xCBC9 =>             2,
            0xCBCA =>             2,
            0xCBCB =>             2,
            0xCBCC =>             2,
            0xCBCD =>             2,
            0xCBCE =>             2,
            0xCBCF =>             2,
            0xCBD0 =>             2,
            0xCBD1 =>             2,
            0xCBD2 =>             2,
            0xCBD3 =>             2,
            0xCBD4 =>             2,
            0xCBD5 =>             2,
            0xCBD6 =>             2,
            0xCBD7 =>             2,
            0xCBD8 =>             2,
            0xCBD9 =>             2,
            0xCBDA =>             2,
            0xCBDB =>             2,
            0xCBDC =>             2,
            0xCBDD =>             2,
            0xCBDE =>             2,
            0xCBDF =>             2,
            0xCBE0 =>             2,
            0xCBE1 =>             2,
            0xCBE2 =>             2,
            0xCBE3 =>             2,
            0xCBE4 =>             2,
            0xCBE5 =>             2,
            0xCBE6 =>             2,
            0xCBE7 =>             2,
            0xCBE8 =>             2,
            0xCBE9 =>             2,
            0xCBEA =>             2,
            0xCBEB =>             2,
            0xCBEC =>             2,
            0xCBED =>             2,
            0xCBEE =>             2,
            0xCBEF =>             2,
            0xCBF0 =>             2,
            0xCBF1 =>             2,
            0xCBF2 =>             2,
            0xCBF3 =>             2,
            0xCBF4 =>             2,
            0xCBF5 =>             2,
            0xCBF6 =>             2,
            0xCBF7 =>             2,
            0xCBF8 =>             2,
            0xCBF9 =>             2,
            0xCBFA =>             2,
            0xCBFB =>             2,
            0xCBFC =>             2,
            0xCBFD =>             2,
            0xCBFE =>             2,
            0xCBFF =>             2,
            _ => return NoInstruction
        },
        _ => return NoInstruction
    };

    let mut instruction = vec![];
    instruction.resize(size, 0);
    instruction.clone_from_slice(&original_stream[0..size]);
    return SomeInstruction(instruction);
}

impl<'a> LR35902InstructionSet for LR35902InstructionPrinter<'a> {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "RES", implicit_data1, register2);
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "SPHL");
    }
    fn shift_register_right(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRL", register1);
    }
    fn double_add(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DAD", register1);
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ORI", data1);
    }
    fn no_operation(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "NOP");
    }
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RL", register1);
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16)
    {
        self.error = write!(self.stream_out, "{:04} {:?} #${:02x}", "LXI", register1, data2);
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MOV", register1, register2);
    }
    fn enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "EI");
    }
    fn return_if_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "XRI", data1);
    }
    fn rotate_accumulator_right(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RRC");
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ANI", data1);
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCR", register1);
    }
    fn halt(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn return_and_enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RETI");
    }
    fn set_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "SET", implicit_data1, register2);
    }
    fn rotate_register_right(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RRC", register1);
    }
    fn shift_register_right_signed(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRA", register1);
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "CMP", register1);
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} {}", "RST", implicit_data1);
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRNZ", data1);
    }
    fn rotate_register_left(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RLC", register1);
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCX", register1);
    }
    fn complement_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CCF");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDAD", address1);
    }
    fn return_if_not_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ORA", register1);
    }
    fn shift_register_left(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SLA", register1);
    }
    fn jump(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JMP", address1);
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNZ", address1);
    }
    fn store_sp_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SSPD", address1);
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SUI", data1);
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RAL");
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SUB", register1);
    }
    fn load_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "LDAX", register1);
    }
    fn move_and_decrement_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MVM-", register1, register2);
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRNC", data1);
    }
    fn return_unconditionally(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RET");
    }
    fn load_accumulator_one_byte(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "LDAC");
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNZ", address1);
    }
    fn jump_relative_if_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRC", data1);
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CC", address1);
    }
    fn test_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "BIT", implicit_data1, register2);
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RAR");
    }
    fn store_accumulator_direct_one_byte(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "STAB", data1);
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ANA", register1);
    }
    fn halt_until_button_press(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "STOP");
    }
    fn jump_relative(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn store_accumulator_one_byte(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "STAC");
    }
    fn set_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "SCF");
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNC", address1);
    }
    fn call(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CALL", address1);
    }
    fn return_if_no_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CZ", address1);
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "LDAB", data1);
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JC", address1);
    }
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RR", register1);
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ACI", data1);
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn swap_register(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SWAP", register1);
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "INX", register1);
    }
    fn store_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "STAX", register1);
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADC", register1);
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SBB", register1);
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "PUSH", register1);
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "INR", register1);
    }
    fn load_program_counter(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "PCHL");
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "POP", register1);
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADI", data1);
    }
    fn store_sp_plus_immediate(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "STSP", data1);
    }
    fn complement_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CPL");
    }
    fn move_and_increment_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MVM+", register1, register2);
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "XRA", register1);
    }
    fn add_immediate_to_sp(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADDS", data1);
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADD", register1);
    }
    fn disable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "DI");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "CPI", data1);
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8)
    {
        self.error = write!(self.stream_out, "{:04} {:?} #${:02x}", "MVI", register1, data2);
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNC", address1);
    }
    fn jump_relative_if_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRZ", data1);
    }
    fn return_if_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JZ", address1);
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SBI", data1);
    }
    fn rotate_accumulator_left(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RLC");
    }
}
