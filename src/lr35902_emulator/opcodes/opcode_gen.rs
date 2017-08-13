use emulator_common::Intel8080Register;
use lr35902_emulator::opcodes::LR35902InstructionPrinter;
use std::io;
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
    machine: &mut I) -> u8
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x00 => { machine.no_operation(); 4 },
        0x01 => { machine.load_register_pair_immediate(Intel8080Register::B, read_u16(&mut stream).unwrap()); 12 },
        0x02 => { machine.store_accumulator(Intel8080Register::B); 8 },
        0x03 => { machine.increment_register_pair(Intel8080Register::B); 8 },
        0x04 => { machine.increment_register_or_memory(Intel8080Register::B); 4 },
        0x05 => { machine.decrement_register_or_memory(Intel8080Register::B); 4 },
        0x06 => { machine.move_immediate_data(Intel8080Register::B, read_u8(&mut stream).unwrap()); 8 },
        0x07 => { machine.rotate_accumulator_left(); 4 },
        0x08 => { machine.store_sp_direct(read_u16(&mut stream).unwrap()); 20 },
        0x09 => { machine.double_add(Intel8080Register::B); 8 },
        0x0A => { machine.load_accumulator(Intel8080Register::B); 8 },
        0x0B => { machine.decrement_register_pair(Intel8080Register::B); 8 },
        0x0C => { machine.increment_register_or_memory(Intel8080Register::C); 4 },
        0x0D => { machine.decrement_register_or_memory(Intel8080Register::C); 4 },
        0x0E => { machine.move_immediate_data(Intel8080Register::C, read_u8(&mut stream).unwrap()); 8 },
        0x0F => { machine.rotate_accumulator_right(); 4 },
        0x11 => { machine.load_register_pair_immediate(Intel8080Register::D, read_u16(&mut stream).unwrap()); 12 },
        0x12 => { machine.store_accumulator(Intel8080Register::D); 8 },
        0x13 => { machine.increment_register_pair(Intel8080Register::D); 8 },
        0x14 => { machine.increment_register_or_memory(Intel8080Register::D); 4 },
        0x15 => { machine.decrement_register_or_memory(Intel8080Register::D); 4 },
        0x16 => { machine.move_immediate_data(Intel8080Register::D, read_u8(&mut stream).unwrap()); 8 },
        0x17 => { machine.rotate_accumulator_left_through_carry(); 4 },
        0x18 => { machine.jump_relative(read_u8(&mut stream).unwrap()); 12 },
        0x19 => { machine.double_add(Intel8080Register::D); 8 },
        0x1A => { machine.load_accumulator(Intel8080Register::D); 8 },
        0x1B => { machine.decrement_register_pair(Intel8080Register::D); 8 },
        0x1C => { machine.increment_register_or_memory(Intel8080Register::E); 4 },
        0x1D => { machine.decrement_register_or_memory(Intel8080Register::E); 4 },
        0x1E => { machine.move_immediate_data(Intel8080Register::E, read_u8(&mut stream).unwrap()); 8 },
        0x1F => { machine.rotate_accumulator_right_through_carry(); 4 },
        0x20 => { machine.jump_relative_if_not_zero(read_u8(&mut stream).unwrap()); 8 },
        0x21 => { machine.load_register_pair_immediate(Intel8080Register::H, read_u16(&mut stream).unwrap()); 12 },
        0x22 => { machine.move_and_increment_hl(Intel8080Register::M, Intel8080Register::A); 8 },
        0x23 => { machine.increment_register_pair(Intel8080Register::H); 8 },
        0x24 => { machine.increment_register_or_memory(Intel8080Register::H); 4 },
        0x25 => { machine.decrement_register_or_memory(Intel8080Register::H); 4 },
        0x26 => { machine.move_immediate_data(Intel8080Register::H, read_u8(&mut stream).unwrap()); 8 },
        0x27 => { machine.decimal_adjust_accumulator(); 4 },
        0x28 => { machine.jump_relative_if_zero(read_u8(&mut stream).unwrap()); 8 },
        0x29 => { machine.double_add(Intel8080Register::H); 8 },
        0x2A => { machine.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M); 8 },
        0x2B => { machine.decrement_register_pair(Intel8080Register::H); 8 },
        0x2C => { machine.increment_register_or_memory(Intel8080Register::L); 4 },
        0x2D => { machine.decrement_register_or_memory(Intel8080Register::L); 4 },
        0x2E => { machine.move_immediate_data(Intel8080Register::L, read_u8(&mut stream).unwrap()); 8 },
        0x2F => { machine.complement_accumulator(); 4 },
        0x30 => { machine.jump_relative_if_no_carry(read_u8(&mut stream).unwrap()); 8 },
        0x31 => { machine.load_register_pair_immediate(Intel8080Register::SP, read_u16(&mut stream).unwrap()); 12 },
        0x32 => { machine.move_and_decrement_hl(Intel8080Register::M, Intel8080Register::A); 8 },
        0x33 => { machine.increment_register_pair(Intel8080Register::SP); 8 },
        0x34 => { machine.increment_register_or_memory(Intel8080Register::M); 12 },
        0x35 => { machine.decrement_register_or_memory(Intel8080Register::M); 12 },
        0x36 => { machine.move_immediate_data(Intel8080Register::M, read_u8(&mut stream).unwrap()); 12 },
        0x37 => { machine.set_carry(); 4 },
        0x38 => { machine.jump_relative_if_carry(read_u8(&mut stream).unwrap()); 8 },
        0x39 => { machine.double_add(Intel8080Register::SP); 8 },
        0x3A => { machine.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M); 8 },
        0x3B => { machine.decrement_register_pair(Intel8080Register::SP); 8 },
        0x3C => { machine.increment_register_or_memory(Intel8080Register::A); 4 },
        0x3D => { machine.decrement_register_or_memory(Intel8080Register::A); 4 },
        0x3E => { machine.move_immediate_data(Intel8080Register::A, read_u8(&mut stream).unwrap()); 8 },
        0x3F => { machine.complement_carry(); 4 },
        0x40 => { machine.move_data(Intel8080Register::B, Intel8080Register::B); 4 },
        0x41 => { machine.move_data(Intel8080Register::B, Intel8080Register::C); 4 },
        0x42 => { machine.move_data(Intel8080Register::B, Intel8080Register::D); 4 },
        0x43 => { machine.move_data(Intel8080Register::B, Intel8080Register::E); 4 },
        0x44 => { machine.move_data(Intel8080Register::B, Intel8080Register::H); 4 },
        0x45 => { machine.move_data(Intel8080Register::B, Intel8080Register::L); 4 },
        0x46 => { machine.move_data(Intel8080Register::B, Intel8080Register::M); 8 },
        0x47 => { machine.move_data(Intel8080Register::B, Intel8080Register::A); 4 },
        0x48 => { machine.move_data(Intel8080Register::C, Intel8080Register::B); 4 },
        0x49 => { machine.move_data(Intel8080Register::C, Intel8080Register::C); 4 },
        0x4A => { machine.move_data(Intel8080Register::C, Intel8080Register::D); 4 },
        0x4B => { machine.move_data(Intel8080Register::C, Intel8080Register::E); 4 },
        0x4C => { machine.move_data(Intel8080Register::C, Intel8080Register::H); 4 },
        0x4D => { machine.move_data(Intel8080Register::C, Intel8080Register::L); 4 },
        0x4E => { machine.move_data(Intel8080Register::C, Intel8080Register::M); 8 },
        0x4F => { machine.move_data(Intel8080Register::C, Intel8080Register::A); 4 },
        0x50 => { machine.move_data(Intel8080Register::D, Intel8080Register::B); 4 },
        0x51 => { machine.move_data(Intel8080Register::D, Intel8080Register::C); 4 },
        0x52 => { machine.move_data(Intel8080Register::D, Intel8080Register::D); 4 },
        0x53 => { machine.move_data(Intel8080Register::D, Intel8080Register::E); 4 },
        0x54 => { machine.move_data(Intel8080Register::D, Intel8080Register::H); 4 },
        0x55 => { machine.move_data(Intel8080Register::D, Intel8080Register::L); 4 },
        0x56 => { machine.move_data(Intel8080Register::D, Intel8080Register::M); 8 },
        0x57 => { machine.move_data(Intel8080Register::D, Intel8080Register::A); 4 },
        0x58 => { machine.move_data(Intel8080Register::E, Intel8080Register::B); 4 },
        0x59 => { machine.move_data(Intel8080Register::E, Intel8080Register::C); 4 },
        0x5A => { machine.move_data(Intel8080Register::E, Intel8080Register::D); 4 },
        0x5B => { machine.move_data(Intel8080Register::E, Intel8080Register::E); 4 },
        0x5C => { machine.move_data(Intel8080Register::E, Intel8080Register::H); 4 },
        0x5D => { machine.move_data(Intel8080Register::E, Intel8080Register::L); 4 },
        0x5E => { machine.move_data(Intel8080Register::E, Intel8080Register::M); 8 },
        0x5F => { machine.move_data(Intel8080Register::E, Intel8080Register::A); 4 },
        0x60 => { machine.move_data(Intel8080Register::H, Intel8080Register::B); 4 },
        0x61 => { machine.move_data(Intel8080Register::H, Intel8080Register::C); 4 },
        0x62 => { machine.move_data(Intel8080Register::H, Intel8080Register::D); 4 },
        0x63 => { machine.move_data(Intel8080Register::H, Intel8080Register::E); 4 },
        0x64 => { machine.move_data(Intel8080Register::H, Intel8080Register::H); 4 },
        0x65 => { machine.move_data(Intel8080Register::H, Intel8080Register::L); 4 },
        0x66 => { machine.move_data(Intel8080Register::H, Intel8080Register::M); 8 },
        0x67 => { machine.move_data(Intel8080Register::H, Intel8080Register::A); 4 },
        0x68 => { machine.move_data(Intel8080Register::L, Intel8080Register::B); 4 },
        0x69 => { machine.move_data(Intel8080Register::L, Intel8080Register::C); 4 },
        0x6A => { machine.move_data(Intel8080Register::L, Intel8080Register::D); 4 },
        0x6B => { machine.move_data(Intel8080Register::L, Intel8080Register::E); 4 },
        0x6C => { machine.move_data(Intel8080Register::L, Intel8080Register::H); 4 },
        0x6D => { machine.move_data(Intel8080Register::L, Intel8080Register::L); 4 },
        0x6E => { machine.move_data(Intel8080Register::L, Intel8080Register::M); 8 },
        0x6F => { machine.move_data(Intel8080Register::L, Intel8080Register::A); 4 },
        0x70 => { machine.move_data(Intel8080Register::M, Intel8080Register::B); 8 },
        0x71 => { machine.move_data(Intel8080Register::M, Intel8080Register::C); 8 },
        0x72 => { machine.move_data(Intel8080Register::M, Intel8080Register::D); 8 },
        0x73 => { machine.move_data(Intel8080Register::M, Intel8080Register::E); 8 },
        0x74 => { machine.move_data(Intel8080Register::M, Intel8080Register::H); 8 },
        0x75 => { machine.move_data(Intel8080Register::M, Intel8080Register::L); 8 },
        0x76 => { machine.halt(); 4 },
        0x77 => { machine.move_data(Intel8080Register::M, Intel8080Register::A); 8 },
        0x78 => { machine.move_data(Intel8080Register::A, Intel8080Register::B); 4 },
        0x79 => { machine.move_data(Intel8080Register::A, Intel8080Register::C); 4 },
        0x7A => { machine.move_data(Intel8080Register::A, Intel8080Register::D); 4 },
        0x7B => { machine.move_data(Intel8080Register::A, Intel8080Register::E); 4 },
        0x7C => { machine.move_data(Intel8080Register::A, Intel8080Register::H); 4 },
        0x7D => { machine.move_data(Intel8080Register::A, Intel8080Register::L); 4 },
        0x7E => { machine.move_data(Intel8080Register::A, Intel8080Register::M); 8 },
        0x7F => { machine.move_data(Intel8080Register::A, Intel8080Register::A); 4 },
        0x80 => { machine.add_to_accumulator(Intel8080Register::B); 4 },
        0x81 => { machine.add_to_accumulator(Intel8080Register::C); 4 },
        0x82 => { machine.add_to_accumulator(Intel8080Register::D); 4 },
        0x83 => { machine.add_to_accumulator(Intel8080Register::E); 4 },
        0x84 => { machine.add_to_accumulator(Intel8080Register::H); 4 },
        0x85 => { machine.add_to_accumulator(Intel8080Register::L); 4 },
        0x86 => { machine.add_to_accumulator(Intel8080Register::M); 8 },
        0x87 => { machine.add_to_accumulator(Intel8080Register::A); 4 },
        0x88 => { machine.add_to_accumulator_with_carry(Intel8080Register::B); 4 },
        0x89 => { machine.add_to_accumulator_with_carry(Intel8080Register::C); 4 },
        0x8A => { machine.add_to_accumulator_with_carry(Intel8080Register::D); 4 },
        0x8B => { machine.add_to_accumulator_with_carry(Intel8080Register::E); 4 },
        0x8C => { machine.add_to_accumulator_with_carry(Intel8080Register::H); 4 },
        0x8D => { machine.add_to_accumulator_with_carry(Intel8080Register::L); 4 },
        0x8E => { machine.add_to_accumulator_with_carry(Intel8080Register::M); 8 },
        0x8F => { machine.add_to_accumulator_with_carry(Intel8080Register::A); 4 },
        0x90 => { machine.subtract_from_accumulator(Intel8080Register::B); 4 },
        0x91 => { machine.subtract_from_accumulator(Intel8080Register::C); 4 },
        0x92 => { machine.subtract_from_accumulator(Intel8080Register::D); 4 },
        0x93 => { machine.subtract_from_accumulator(Intel8080Register::E); 4 },
        0x94 => { machine.subtract_from_accumulator(Intel8080Register::H); 4 },
        0x95 => { machine.subtract_from_accumulator(Intel8080Register::L); 4 },
        0x96 => { machine.subtract_from_accumulator(Intel8080Register::M); 8 },
        0x97 => { machine.subtract_from_accumulator(Intel8080Register::A); 4 },
        0x98 => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::B); 4 },
        0x99 => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::C); 4 },
        0x9A => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::D); 4 },
        0x9B => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::E); 4 },
        0x9C => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::H); 4 },
        0x9D => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::L); 4 },
        0x9E => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::M); 8 },
        0x9F => { machine.subtract_from_accumulator_with_borrow(Intel8080Register::A); 4 },
        0xA0 => { machine.logical_and_with_accumulator(Intel8080Register::B); 4 },
        0xA1 => { machine.logical_and_with_accumulator(Intel8080Register::C); 4 },
        0xA2 => { machine.logical_and_with_accumulator(Intel8080Register::D); 4 },
        0xA3 => { machine.logical_and_with_accumulator(Intel8080Register::E); 4 },
        0xA4 => { machine.logical_and_with_accumulator(Intel8080Register::H); 4 },
        0xA5 => { machine.logical_and_with_accumulator(Intel8080Register::L); 4 },
        0xA6 => { machine.logical_and_with_accumulator(Intel8080Register::M); 8 },
        0xA7 => { machine.logical_and_with_accumulator(Intel8080Register::A); 4 },
        0xA8 => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::B); 4 },
        0xA9 => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::C); 4 },
        0xAA => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::D); 4 },
        0xAB => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::E); 4 },
        0xAC => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::H); 4 },
        0xAD => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::L); 4 },
        0xAE => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::M); 8 },
        0xAF => { machine.logical_exclusive_or_with_accumulator(Intel8080Register::A); 4 },
        0xB0 => { machine.logical_or_with_accumulator(Intel8080Register::B); 4 },
        0xB1 => { machine.logical_or_with_accumulator(Intel8080Register::C); 4 },
        0xB2 => { machine.logical_or_with_accumulator(Intel8080Register::D); 4 },
        0xB3 => { machine.logical_or_with_accumulator(Intel8080Register::E); 4 },
        0xB4 => { machine.logical_or_with_accumulator(Intel8080Register::H); 4 },
        0xB5 => { machine.logical_or_with_accumulator(Intel8080Register::L); 4 },
        0xB6 => { machine.logical_or_with_accumulator(Intel8080Register::M); 8 },
        0xB7 => { machine.logical_or_with_accumulator(Intel8080Register::A); 4 },
        0xB8 => { machine.compare_with_accumulator(Intel8080Register::B); 4 },
        0xB9 => { machine.compare_with_accumulator(Intel8080Register::C); 4 },
        0xBA => { machine.compare_with_accumulator(Intel8080Register::D); 4 },
        0xBB => { machine.compare_with_accumulator(Intel8080Register::E); 4 },
        0xBC => { machine.compare_with_accumulator(Intel8080Register::H); 4 },
        0xBD => { machine.compare_with_accumulator(Intel8080Register::L); 4 },
        0xBE => { machine.compare_with_accumulator(Intel8080Register::M); 8 },
        0xBF => { machine.compare_with_accumulator(Intel8080Register::A); 4 },
        0xC0 => { machine.return_if_not_zero(); 8 },
        0xC1 => { machine.pop_data_off_stack(Intel8080Register::B); 12 },
        0xC2 => { machine.jump_if_not_zero(read_u16(&mut stream).unwrap()); 12 },
        0xC3 => { machine.jump(read_u16(&mut stream).unwrap()); 16 },
        0xC4 => { machine.call_if_not_zero(read_u16(&mut stream).unwrap()); 12 },
        0xC5 => { machine.push_data_onto_stack(Intel8080Register::B); 16 },
        0xC6 => { machine.add_immediate_to_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xC7 => { machine.restart(0 as u8); 16 },
        0xC8 => { machine.return_if_zero(); 8 },
        0xC9 => { machine.return_unconditionally(); 16 },
        0xCA => { machine.jump_if_zero(read_u16(&mut stream).unwrap()); 12 },
        0xCC => { machine.call_if_zero(read_u16(&mut stream).unwrap()); 12 },
        0xCD => { machine.call(read_u16(&mut stream).unwrap()); 24 },
        0xCE => { machine.add_immediate_to_accumulator_with_carry(read_u8(&mut stream).unwrap()); 8 },
        0xCF => { machine.restart(1 as u8); 16 },
        0xD0 => { machine.return_if_no_carry(); 8 },
        0xD1 => { machine.pop_data_off_stack(Intel8080Register::D); 12 },
        0xD2 => { machine.jump_if_no_carry(read_u16(&mut stream).unwrap()); 12 },
        0xD4 => { machine.call_if_no_carry(read_u16(&mut stream).unwrap()); 12 },
        0xD5 => { machine.push_data_onto_stack(Intel8080Register::D); 16 },
        0xD6 => { machine.subtract_immediate_from_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xD7 => { machine.restart(2 as u8); 16 },
        0xD8 => { machine.return_if_carry(); 8 },
        0xD9 => { machine.return_and_enable_interrupts(); 16 },
        0xDA => { machine.jump_if_carry(read_u16(&mut stream).unwrap()); 12 },
        0xDC => { machine.call_if_carry(read_u16(&mut stream).unwrap()); 12 },
        0xDE => { machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).unwrap()); 8 },
        0xDF => { machine.restart(3 as u8); 16 },
        0xE0 => { machine.store_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()); 12 },
        0xE1 => { machine.pop_data_off_stack(Intel8080Register::H); 12 },
        0xE2 => { machine.store_accumulator_one_byte(); 8 },
        0xE5 => { machine.push_data_onto_stack(Intel8080Register::H); 16 },
        0xE6 => { machine.and_immediate_with_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xE7 => { machine.restart(4 as u8); 16 },
        0xE8 => { machine.add_immediate_to_sp(read_u8(&mut stream).unwrap()); 16 },
        0xE9 => { machine.load_program_counter(); 4 },
        0xEA => { machine.store_accumulator_direct(read_u16(&mut stream).unwrap()); 16 },
        0xEE => { machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xEF => { machine.restart(5 as u8); 16 },
        0xF0 => { machine.load_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()); 12 },
        0xF1 => { machine.pop_data_off_stack(Intel8080Register::PSW); 12 },
        0xF2 => { machine.load_accumulator_one_byte(); 8 },
        0xF3 => { machine.disable_interrupts(); 4 },
        0xF5 => { machine.push_data_onto_stack(Intel8080Register::PSW); 16 },
        0xF6 => { machine.or_immediate_with_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xF7 => { machine.restart(6 as u8); 16 },
        0xF8 => { machine.store_sp_plus_immediate(read_u8(&mut stream).unwrap()); 12 },
        0xF9 => { machine.load_sp_from_h_and_l(); 8 },
        0xFA => { machine.load_accumulator_direct(read_u16(&mut stream).unwrap()); 16 },
        0xFB => { machine.enable_interrupts(); 4 },
        0xFE => { machine.compare_immediate_with_accumulator(read_u8(&mut stream).unwrap()); 8 },
        0xFF => { machine.restart(7 as u8); 16 },
        0x10 => match (0x10 as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0x1000 => { machine.halt_until_button_press(); 4 },
            v => panic!("Unknown opcode {}", v)
        },
        0xCB => match (0xCB as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0xCB00 => { machine.rotate_register_left(Intel8080Register::B); 8 },
            0xCB01 => { machine.rotate_register_left(Intel8080Register::C); 8 },
            0xCB02 => { machine.rotate_register_left(Intel8080Register::D); 8 },
            0xCB03 => { machine.rotate_register_left(Intel8080Register::E); 8 },
            0xCB04 => { machine.rotate_register_left(Intel8080Register::H); 8 },
            0xCB05 => { machine.rotate_register_left(Intel8080Register::L); 8 },
            0xCB06 => { machine.rotate_register_left(Intel8080Register::M); 16 },
            0xCB07 => { machine.rotate_register_left(Intel8080Register::A); 8 },
            0xCB08 => { machine.rotate_register_right(Intel8080Register::B); 8 },
            0xCB09 => { machine.rotate_register_right(Intel8080Register::C); 8 },
            0xCB0A => { machine.rotate_register_right(Intel8080Register::D); 8 },
            0xCB0B => { machine.rotate_register_right(Intel8080Register::E); 8 },
            0xCB0C => { machine.rotate_register_right(Intel8080Register::H); 8 },
            0xCB0D => { machine.rotate_register_right(Intel8080Register::L); 8 },
            0xCB0E => { machine.rotate_register_right(Intel8080Register::M); 16 },
            0xCB0F => { machine.rotate_register_right(Intel8080Register::A); 8 },
            0xCB10 => { machine.rotate_register_left_through_carry(Intel8080Register::B); 8 },
            0xCB11 => { machine.rotate_register_left_through_carry(Intel8080Register::C); 8 },
            0xCB12 => { machine.rotate_register_left_through_carry(Intel8080Register::D); 8 },
            0xCB13 => { machine.rotate_register_left_through_carry(Intel8080Register::E); 8 },
            0xCB14 => { machine.rotate_register_left_through_carry(Intel8080Register::H); 8 },
            0xCB15 => { machine.rotate_register_left_through_carry(Intel8080Register::L); 8 },
            0xCB16 => { machine.rotate_register_left_through_carry(Intel8080Register::M); 16 },
            0xCB17 => { machine.rotate_register_left_through_carry(Intel8080Register::A); 8 },
            0xCB18 => { machine.rotate_register_right_through_carry(Intel8080Register::B); 8 },
            0xCB19 => { machine.rotate_register_right_through_carry(Intel8080Register::C); 8 },
            0xCB1A => { machine.rotate_register_right_through_carry(Intel8080Register::D); 8 },
            0xCB1B => { machine.rotate_register_right_through_carry(Intel8080Register::E); 8 },
            0xCB1C => { machine.rotate_register_right_through_carry(Intel8080Register::H); 8 },
            0xCB1D => { machine.rotate_register_right_through_carry(Intel8080Register::L); 8 },
            0xCB1E => { machine.rotate_register_right_through_carry(Intel8080Register::M); 16 },
            0xCB1F => { machine.rotate_register_right_through_carry(Intel8080Register::A); 8 },
            0xCB20 => { machine.shift_register_left(Intel8080Register::B); 8 },
            0xCB21 => { machine.shift_register_left(Intel8080Register::C); 8 },
            0xCB22 => { machine.shift_register_left(Intel8080Register::D); 8 },
            0xCB23 => { machine.shift_register_left(Intel8080Register::E); 8 },
            0xCB24 => { machine.shift_register_left(Intel8080Register::H); 8 },
            0xCB25 => { machine.shift_register_left(Intel8080Register::L); 8 },
            0xCB26 => { machine.shift_register_left(Intel8080Register::M); 16 },
            0xCB27 => { machine.shift_register_left(Intel8080Register::A); 8 },
            0xCB28 => { machine.shift_register_right_signed(Intel8080Register::B); 8 },
            0xCB29 => { machine.shift_register_right_signed(Intel8080Register::C); 8 },
            0xCB2A => { machine.shift_register_right_signed(Intel8080Register::D); 8 },
            0xCB2B => { machine.shift_register_right_signed(Intel8080Register::E); 8 },
            0xCB2C => { machine.shift_register_right_signed(Intel8080Register::H); 8 },
            0xCB2D => { machine.shift_register_right_signed(Intel8080Register::L); 8 },
            0xCB2E => { machine.shift_register_right_signed(Intel8080Register::M); 16 },
            0xCB2F => { machine.shift_register_right_signed(Intel8080Register::A); 8 },
            0xCB30 => { machine.swap_register(Intel8080Register::B); 8 },
            0xCB31 => { machine.swap_register(Intel8080Register::C); 8 },
            0xCB32 => { machine.swap_register(Intel8080Register::D); 8 },
            0xCB33 => { machine.swap_register(Intel8080Register::E); 8 },
            0xCB34 => { machine.swap_register(Intel8080Register::H); 8 },
            0xCB35 => { machine.swap_register(Intel8080Register::L); 8 },
            0xCB36 => { machine.swap_register(Intel8080Register::M); 16 },
            0xCB37 => { machine.swap_register(Intel8080Register::A); 8 },
            0xCB38 => { machine.shift_register_right(Intel8080Register::B); 8 },
            0xCB39 => { machine.shift_register_right(Intel8080Register::C); 8 },
            0xCB3A => { machine.shift_register_right(Intel8080Register::D); 8 },
            0xCB3B => { machine.shift_register_right(Intel8080Register::E); 8 },
            0xCB3C => { machine.shift_register_right(Intel8080Register::H); 8 },
            0xCB3D => { machine.shift_register_right(Intel8080Register::L); 8 },
            0xCB3E => { machine.shift_register_right(Intel8080Register::M); 16 },
            0xCB3F => { machine.shift_register_right(Intel8080Register::A); 8 },
            0xCB40 => { machine.test_bit(0 as u8, Intel8080Register::B); 8 },
            0xCB41 => { machine.test_bit(0 as u8, Intel8080Register::C); 8 },
            0xCB42 => { machine.test_bit(0 as u8, Intel8080Register::D); 8 },
            0xCB43 => { machine.test_bit(0 as u8, Intel8080Register::E); 8 },
            0xCB44 => { machine.test_bit(0 as u8, Intel8080Register::H); 8 },
            0xCB45 => { machine.test_bit(0 as u8, Intel8080Register::L); 8 },
            0xCB46 => { machine.test_bit(0 as u8, Intel8080Register::M); 16 },
            0xCB47 => { machine.test_bit(0 as u8, Intel8080Register::A); 8 },
            0xCB48 => { machine.test_bit(1 as u8, Intel8080Register::B); 8 },
            0xCB49 => { machine.test_bit(1 as u8, Intel8080Register::C); 8 },
            0xCB4A => { machine.test_bit(1 as u8, Intel8080Register::D); 8 },
            0xCB4B => { machine.test_bit(1 as u8, Intel8080Register::E); 8 },
            0xCB4C => { machine.test_bit(1 as u8, Intel8080Register::H); 8 },
            0xCB4D => { machine.test_bit(1 as u8, Intel8080Register::L); 8 },
            0xCB4E => { machine.test_bit(1 as u8, Intel8080Register::M); 16 },
            0xCB4F => { machine.test_bit(1 as u8, Intel8080Register::A); 8 },
            0xCB50 => { machine.test_bit(2 as u8, Intel8080Register::B); 8 },
            0xCB51 => { machine.test_bit(2 as u8, Intel8080Register::C); 8 },
            0xCB52 => { machine.test_bit(2 as u8, Intel8080Register::D); 8 },
            0xCB53 => { machine.test_bit(2 as u8, Intel8080Register::E); 8 },
            0xCB54 => { machine.test_bit(2 as u8, Intel8080Register::H); 8 },
            0xCB55 => { machine.test_bit(2 as u8, Intel8080Register::L); 8 },
            0xCB56 => { machine.test_bit(2 as u8, Intel8080Register::M); 16 },
            0xCB57 => { machine.test_bit(2 as u8, Intel8080Register::A); 8 },
            0xCB58 => { machine.test_bit(3 as u8, Intel8080Register::B); 8 },
            0xCB59 => { machine.test_bit(3 as u8, Intel8080Register::C); 8 },
            0xCB5A => { machine.test_bit(3 as u8, Intel8080Register::D); 8 },
            0xCB5B => { machine.test_bit(3 as u8, Intel8080Register::E); 8 },
            0xCB5C => { machine.test_bit(3 as u8, Intel8080Register::H); 8 },
            0xCB5D => { machine.test_bit(3 as u8, Intel8080Register::L); 8 },
            0xCB5E => { machine.test_bit(3 as u8, Intel8080Register::M); 16 },
            0xCB5F => { machine.test_bit(3 as u8, Intel8080Register::A); 8 },
            0xCB60 => { machine.test_bit(4 as u8, Intel8080Register::B); 8 },
            0xCB61 => { machine.test_bit(4 as u8, Intel8080Register::C); 8 },
            0xCB62 => { machine.test_bit(4 as u8, Intel8080Register::D); 8 },
            0xCB63 => { machine.test_bit(4 as u8, Intel8080Register::E); 8 },
            0xCB64 => { machine.test_bit(4 as u8, Intel8080Register::H); 8 },
            0xCB65 => { machine.test_bit(4 as u8, Intel8080Register::L); 8 },
            0xCB66 => { machine.test_bit(4 as u8, Intel8080Register::M); 16 },
            0xCB67 => { machine.test_bit(4 as u8, Intel8080Register::A); 8 },
            0xCB68 => { machine.test_bit(5 as u8, Intel8080Register::B); 8 },
            0xCB69 => { machine.test_bit(5 as u8, Intel8080Register::C); 8 },
            0xCB6A => { machine.test_bit(5 as u8, Intel8080Register::D); 8 },
            0xCB6B => { machine.test_bit(5 as u8, Intel8080Register::E); 8 },
            0xCB6C => { machine.test_bit(5 as u8, Intel8080Register::H); 8 },
            0xCB6D => { machine.test_bit(5 as u8, Intel8080Register::L); 8 },
            0xCB6E => { machine.test_bit(5 as u8, Intel8080Register::M); 16 },
            0xCB6F => { machine.test_bit(5 as u8, Intel8080Register::A); 8 },
            0xCB70 => { machine.test_bit(6 as u8, Intel8080Register::B); 8 },
            0xCB71 => { machine.test_bit(6 as u8, Intel8080Register::C); 8 },
            0xCB72 => { machine.test_bit(6 as u8, Intel8080Register::D); 8 },
            0xCB73 => { machine.test_bit(6 as u8, Intel8080Register::E); 8 },
            0xCB74 => { machine.test_bit(6 as u8, Intel8080Register::H); 8 },
            0xCB75 => { machine.test_bit(6 as u8, Intel8080Register::L); 8 },
            0xCB76 => { machine.test_bit(6 as u8, Intel8080Register::M); 16 },
            0xCB77 => { machine.test_bit(6 as u8, Intel8080Register::A); 8 },
            0xCB78 => { machine.test_bit(7 as u8, Intel8080Register::B); 8 },
            0xCB79 => { machine.test_bit(7 as u8, Intel8080Register::C); 8 },
            0xCB7A => { machine.test_bit(7 as u8, Intel8080Register::D); 8 },
            0xCB7B => { machine.test_bit(7 as u8, Intel8080Register::E); 8 },
            0xCB7C => { machine.test_bit(7 as u8, Intel8080Register::H); 8 },
            0xCB7D => { machine.test_bit(7 as u8, Intel8080Register::L); 8 },
            0xCB7E => { machine.test_bit(7 as u8, Intel8080Register::M); 16 },
            0xCB7F => { machine.test_bit(7 as u8, Intel8080Register::A); 8 },
            0xCB80 => { machine.reset_bit(0 as u8, Intel8080Register::B); 8 },
            0xCB81 => { machine.reset_bit(0 as u8, Intel8080Register::C); 8 },
            0xCB82 => { machine.reset_bit(0 as u8, Intel8080Register::D); 8 },
            0xCB83 => { machine.reset_bit(0 as u8, Intel8080Register::E); 8 },
            0xCB84 => { machine.reset_bit(0 as u8, Intel8080Register::H); 8 },
            0xCB85 => { machine.reset_bit(0 as u8, Intel8080Register::L); 8 },
            0xCB86 => { machine.reset_bit(0 as u8, Intel8080Register::M); 16 },
            0xCB87 => { machine.reset_bit(0 as u8, Intel8080Register::A); 8 },
            0xCB88 => { machine.reset_bit(1 as u8, Intel8080Register::B); 8 },
            0xCB89 => { machine.reset_bit(1 as u8, Intel8080Register::C); 8 },
            0xCB8A => { machine.reset_bit(1 as u8, Intel8080Register::D); 8 },
            0xCB8B => { machine.reset_bit(1 as u8, Intel8080Register::E); 8 },
            0xCB8C => { machine.reset_bit(1 as u8, Intel8080Register::H); 8 },
            0xCB8D => { machine.reset_bit(1 as u8, Intel8080Register::L); 8 },
            0xCB8E => { machine.reset_bit(1 as u8, Intel8080Register::M); 16 },
            0xCB8F => { machine.reset_bit(1 as u8, Intel8080Register::A); 8 },
            0xCB90 => { machine.reset_bit(2 as u8, Intel8080Register::B); 8 },
            0xCB91 => { machine.reset_bit(2 as u8, Intel8080Register::C); 8 },
            0xCB92 => { machine.reset_bit(2 as u8, Intel8080Register::D); 8 },
            0xCB93 => { machine.reset_bit(2 as u8, Intel8080Register::E); 8 },
            0xCB94 => { machine.reset_bit(2 as u8, Intel8080Register::H); 8 },
            0xCB95 => { machine.reset_bit(2 as u8, Intel8080Register::L); 8 },
            0xCB96 => { machine.reset_bit(2 as u8, Intel8080Register::M); 16 },
            0xCB97 => { machine.reset_bit(2 as u8, Intel8080Register::A); 8 },
            0xCB98 => { machine.reset_bit(3 as u8, Intel8080Register::B); 8 },
            0xCB99 => { machine.reset_bit(3 as u8, Intel8080Register::C); 8 },
            0xCB9A => { machine.reset_bit(3 as u8, Intel8080Register::D); 8 },
            0xCB9B => { machine.reset_bit(3 as u8, Intel8080Register::E); 8 },
            0xCB9C => { machine.reset_bit(3 as u8, Intel8080Register::H); 8 },
            0xCB9D => { machine.reset_bit(3 as u8, Intel8080Register::L); 8 },
            0xCB9E => { machine.reset_bit(3 as u8, Intel8080Register::M); 16 },
            0xCB9F => { machine.reset_bit(3 as u8, Intel8080Register::A); 8 },
            0xCBA0 => { machine.reset_bit(4 as u8, Intel8080Register::B); 8 },
            0xCBA1 => { machine.reset_bit(4 as u8, Intel8080Register::C); 8 },
            0xCBA2 => { machine.reset_bit(4 as u8, Intel8080Register::D); 8 },
            0xCBA3 => { machine.reset_bit(4 as u8, Intel8080Register::E); 8 },
            0xCBA4 => { machine.reset_bit(4 as u8, Intel8080Register::H); 8 },
            0xCBA5 => { machine.reset_bit(4 as u8, Intel8080Register::L); 8 },
            0xCBA6 => { machine.reset_bit(4 as u8, Intel8080Register::M); 16 },
            0xCBA7 => { machine.reset_bit(4 as u8, Intel8080Register::A); 8 },
            0xCBA8 => { machine.reset_bit(5 as u8, Intel8080Register::B); 8 },
            0xCBA9 => { machine.reset_bit(5 as u8, Intel8080Register::C); 8 },
            0xCBAA => { machine.reset_bit(5 as u8, Intel8080Register::D); 8 },
            0xCBAB => { machine.reset_bit(5 as u8, Intel8080Register::E); 8 },
            0xCBAC => { machine.reset_bit(5 as u8, Intel8080Register::H); 8 },
            0xCBAD => { machine.reset_bit(5 as u8, Intel8080Register::L); 8 },
            0xCBAE => { machine.reset_bit(5 as u8, Intel8080Register::M); 16 },
            0xCBAF => { machine.reset_bit(5 as u8, Intel8080Register::A); 8 },
            0xCBB0 => { machine.reset_bit(6 as u8, Intel8080Register::B); 8 },
            0xCBB1 => { machine.reset_bit(6 as u8, Intel8080Register::C); 8 },
            0xCBB2 => { machine.reset_bit(6 as u8, Intel8080Register::D); 8 },
            0xCBB3 => { machine.reset_bit(6 as u8, Intel8080Register::E); 8 },
            0xCBB4 => { machine.reset_bit(6 as u8, Intel8080Register::H); 8 },
            0xCBB5 => { machine.reset_bit(6 as u8, Intel8080Register::L); 8 },
            0xCBB6 => { machine.reset_bit(6 as u8, Intel8080Register::M); 16 },
            0xCBB7 => { machine.reset_bit(6 as u8, Intel8080Register::A); 8 },
            0xCBB8 => { machine.reset_bit(7 as u8, Intel8080Register::B); 8 },
            0xCBB9 => { machine.reset_bit(7 as u8, Intel8080Register::C); 8 },
            0xCBBA => { machine.reset_bit(7 as u8, Intel8080Register::D); 8 },
            0xCBBB => { machine.reset_bit(7 as u8, Intel8080Register::E); 8 },
            0xCBBC => { machine.reset_bit(7 as u8, Intel8080Register::H); 8 },
            0xCBBD => { machine.reset_bit(7 as u8, Intel8080Register::L); 8 },
            0xCBBE => { machine.reset_bit(7 as u8, Intel8080Register::M); 16 },
            0xCBBF => { machine.reset_bit(7 as u8, Intel8080Register::A); 8 },
            0xCBC0 => { machine.set_bit(0 as u8, Intel8080Register::B); 8 },
            0xCBC1 => { machine.set_bit(0 as u8, Intel8080Register::C); 8 },
            0xCBC2 => { machine.set_bit(0 as u8, Intel8080Register::D); 8 },
            0xCBC3 => { machine.set_bit(0 as u8, Intel8080Register::E); 8 },
            0xCBC4 => { machine.set_bit(0 as u8, Intel8080Register::H); 8 },
            0xCBC5 => { machine.set_bit(0 as u8, Intel8080Register::L); 8 },
            0xCBC6 => { machine.set_bit(0 as u8, Intel8080Register::M); 16 },
            0xCBC7 => { machine.set_bit(0 as u8, Intel8080Register::A); 8 },
            0xCBC8 => { machine.set_bit(1 as u8, Intel8080Register::B); 8 },
            0xCBC9 => { machine.set_bit(1 as u8, Intel8080Register::C); 8 },
            0xCBCA => { machine.set_bit(1 as u8, Intel8080Register::D); 8 },
            0xCBCB => { machine.set_bit(1 as u8, Intel8080Register::E); 8 },
            0xCBCC => { machine.set_bit(1 as u8, Intel8080Register::H); 8 },
            0xCBCD => { machine.set_bit(1 as u8, Intel8080Register::L); 8 },
            0xCBCE => { machine.set_bit(1 as u8, Intel8080Register::M); 16 },
            0xCBCF => { machine.set_bit(1 as u8, Intel8080Register::A); 8 },
            0xCBD0 => { machine.set_bit(2 as u8, Intel8080Register::B); 8 },
            0xCBD1 => { machine.set_bit(2 as u8, Intel8080Register::C); 8 },
            0xCBD2 => { machine.set_bit(2 as u8, Intel8080Register::D); 8 },
            0xCBD3 => { machine.set_bit(2 as u8, Intel8080Register::E); 8 },
            0xCBD4 => { machine.set_bit(2 as u8, Intel8080Register::H); 8 },
            0xCBD5 => { machine.set_bit(2 as u8, Intel8080Register::L); 8 },
            0xCBD6 => { machine.set_bit(2 as u8, Intel8080Register::M); 16 },
            0xCBD7 => { machine.set_bit(2 as u8, Intel8080Register::A); 8 },
            0xCBD8 => { machine.set_bit(3 as u8, Intel8080Register::B); 8 },
            0xCBD9 => { machine.set_bit(3 as u8, Intel8080Register::C); 8 },
            0xCBDA => { machine.set_bit(3 as u8, Intel8080Register::D); 8 },
            0xCBDB => { machine.set_bit(3 as u8, Intel8080Register::E); 8 },
            0xCBDC => { machine.set_bit(3 as u8, Intel8080Register::H); 8 },
            0xCBDD => { machine.set_bit(3 as u8, Intel8080Register::L); 8 },
            0xCBDE => { machine.set_bit(3 as u8, Intel8080Register::M); 16 },
            0xCBDF => { machine.set_bit(3 as u8, Intel8080Register::A); 8 },
            0xCBE0 => { machine.set_bit(4 as u8, Intel8080Register::B); 8 },
            0xCBE1 => { machine.set_bit(4 as u8, Intel8080Register::C); 8 },
            0xCBE2 => { machine.set_bit(4 as u8, Intel8080Register::D); 8 },
            0xCBE3 => { machine.set_bit(4 as u8, Intel8080Register::E); 8 },
            0xCBE4 => { machine.set_bit(4 as u8, Intel8080Register::H); 8 },
            0xCBE5 => { machine.set_bit(4 as u8, Intel8080Register::L); 8 },
            0xCBE6 => { machine.set_bit(4 as u8, Intel8080Register::M); 16 },
            0xCBE7 => { machine.set_bit(4 as u8, Intel8080Register::A); 8 },
            0xCBE8 => { machine.set_bit(5 as u8, Intel8080Register::B); 8 },
            0xCBE9 => { machine.set_bit(5 as u8, Intel8080Register::C); 8 },
            0xCBEA => { machine.set_bit(5 as u8, Intel8080Register::D); 8 },
            0xCBEB => { machine.set_bit(5 as u8, Intel8080Register::E); 8 },
            0xCBEC => { machine.set_bit(5 as u8, Intel8080Register::H); 8 },
            0xCBED => { machine.set_bit(5 as u8, Intel8080Register::L); 8 },
            0xCBEE => { machine.set_bit(5 as u8, Intel8080Register::M); 16 },
            0xCBEF => { machine.set_bit(5 as u8, Intel8080Register::A); 8 },
            0xCBF0 => { machine.set_bit(6 as u8, Intel8080Register::B); 8 },
            0xCBF1 => { machine.set_bit(6 as u8, Intel8080Register::C); 8 },
            0xCBF2 => { machine.set_bit(6 as u8, Intel8080Register::D); 8 },
            0xCBF3 => { machine.set_bit(6 as u8, Intel8080Register::E); 8 },
            0xCBF4 => { machine.set_bit(6 as u8, Intel8080Register::H); 8 },
            0xCBF5 => { machine.set_bit(6 as u8, Intel8080Register::L); 8 },
            0xCBF6 => { machine.set_bit(6 as u8, Intel8080Register::M); 16 },
            0xCBF7 => { machine.set_bit(6 as u8, Intel8080Register::A); 8 },
            0xCBF8 => { machine.set_bit(7 as u8, Intel8080Register::B); 8 },
            0xCBF9 => { machine.set_bit(7 as u8, Intel8080Register::C); 8 },
            0xCBFA => { machine.set_bit(7 as u8, Intel8080Register::D); 8 },
            0xCBFB => { machine.set_bit(7 as u8, Intel8080Register::E); 8 },
            0xCBFC => { machine.set_bit(7 as u8, Intel8080Register::H); 8 },
            0xCBFD => { machine.set_bit(7 as u8, Intel8080Register::L); 8 },
            0xCBFE => { machine.set_bit(7 as u8, Intel8080Register::M); 16 },
            0xCBFF => { machine.set_bit(7 as u8, Intel8080Register::A); 8 },
            v => panic!("Unknown opcode {}", v)
        },
        v => panic!("Unknown opcode {}", v)
    }
}

pub fn get_lr35902_instruction<R: io::Read>(
    mut stream: R) -> Option<Vec<u8>>
{
    let (mut instr, size) = match read_u8(&mut stream).unwrap() {
        0x00 =>         (vec![0x00], 1),
        0x01 =>         (vec![0x01], 3),
        0x02 =>         (vec![0x02], 1),
        0x03 =>         (vec![0x03], 1),
        0x04 =>         (vec![0x04], 1),
        0x05 =>         (vec![0x05], 1),
        0x06 =>         (vec![0x06], 2),
        0x07 =>         (vec![0x07], 1),
        0x08 =>         (vec![0x08], 3),
        0x09 =>         (vec![0x09], 1),
        0x0A =>         (vec![0x0A], 1),
        0x0B =>         (vec![0x0B], 1),
        0x0C =>         (vec![0x0C], 1),
        0x0D =>         (vec![0x0D], 1),
        0x0E =>         (vec![0x0E], 2),
        0x0F =>         (vec![0x0F], 1),
        0x11 =>         (vec![0x11], 3),
        0x12 =>         (vec![0x12], 1),
        0x13 =>         (vec![0x13], 1),
        0x14 =>         (vec![0x14], 1),
        0x15 =>         (vec![0x15], 1),
        0x16 =>         (vec![0x16], 2),
        0x17 =>         (vec![0x17], 1),
        0x18 =>         (vec![0x18], 2),
        0x19 =>         (vec![0x19], 1),
        0x1A =>         (vec![0x1A], 1),
        0x1B =>         (vec![0x1B], 1),
        0x1C =>         (vec![0x1C], 1),
        0x1D =>         (vec![0x1D], 1),
        0x1E =>         (vec![0x1E], 2),
        0x1F =>         (vec![0x1F], 1),
        0x20 =>         (vec![0x20], 2),
        0x21 =>         (vec![0x21], 3),
        0x22 =>         (vec![0x22], 1),
        0x23 =>         (vec![0x23], 1),
        0x24 =>         (vec![0x24], 1),
        0x25 =>         (vec![0x25], 1),
        0x26 =>         (vec![0x26], 2),
        0x27 =>         (vec![0x27], 1),
        0x28 =>         (vec![0x28], 2),
        0x29 =>         (vec![0x29], 1),
        0x2A =>         (vec![0x2A], 1),
        0x2B =>         (vec![0x2B], 1),
        0x2C =>         (vec![0x2C], 1),
        0x2D =>         (vec![0x2D], 1),
        0x2E =>         (vec![0x2E], 2),
        0x2F =>         (vec![0x2F], 1),
        0x30 =>         (vec![0x30], 2),
        0x31 =>         (vec![0x31], 3),
        0x32 =>         (vec![0x32], 1),
        0x33 =>         (vec![0x33], 1),
        0x34 =>         (vec![0x34], 1),
        0x35 =>         (vec![0x35], 1),
        0x36 =>         (vec![0x36], 2),
        0x37 =>         (vec![0x37], 1),
        0x38 =>         (vec![0x38], 2),
        0x39 =>         (vec![0x39], 1),
        0x3A =>         (vec![0x3A], 1),
        0x3B =>         (vec![0x3B], 1),
        0x3C =>         (vec![0x3C], 1),
        0x3D =>         (vec![0x3D], 1),
        0x3E =>         (vec![0x3E], 2),
        0x3F =>         (vec![0x3F], 1),
        0x40 =>         (vec![0x40], 1),
        0x41 =>         (vec![0x41], 1),
        0x42 =>         (vec![0x42], 1),
        0x43 =>         (vec![0x43], 1),
        0x44 =>         (vec![0x44], 1),
        0x45 =>         (vec![0x45], 1),
        0x46 =>         (vec![0x46], 1),
        0x47 =>         (vec![0x47], 1),
        0x48 =>         (vec![0x48], 1),
        0x49 =>         (vec![0x49], 1),
        0x4A =>         (vec![0x4A], 1),
        0x4B =>         (vec![0x4B], 1),
        0x4C =>         (vec![0x4C], 1),
        0x4D =>         (vec![0x4D], 1),
        0x4E =>         (vec![0x4E], 1),
        0x4F =>         (vec![0x4F], 1),
        0x50 =>         (vec![0x50], 1),
        0x51 =>         (vec![0x51], 1),
        0x52 =>         (vec![0x52], 1),
        0x53 =>         (vec![0x53], 1),
        0x54 =>         (vec![0x54], 1),
        0x55 =>         (vec![0x55], 1),
        0x56 =>         (vec![0x56], 1),
        0x57 =>         (vec![0x57], 1),
        0x58 =>         (vec![0x58], 1),
        0x59 =>         (vec![0x59], 1),
        0x5A =>         (vec![0x5A], 1),
        0x5B =>         (vec![0x5B], 1),
        0x5C =>         (vec![0x5C], 1),
        0x5D =>         (vec![0x5D], 1),
        0x5E =>         (vec![0x5E], 1),
        0x5F =>         (vec![0x5F], 1),
        0x60 =>         (vec![0x60], 1),
        0x61 =>         (vec![0x61], 1),
        0x62 =>         (vec![0x62], 1),
        0x63 =>         (vec![0x63], 1),
        0x64 =>         (vec![0x64], 1),
        0x65 =>         (vec![0x65], 1),
        0x66 =>         (vec![0x66], 1),
        0x67 =>         (vec![0x67], 1),
        0x68 =>         (vec![0x68], 1),
        0x69 =>         (vec![0x69], 1),
        0x6A =>         (vec![0x6A], 1),
        0x6B =>         (vec![0x6B], 1),
        0x6C =>         (vec![0x6C], 1),
        0x6D =>         (vec![0x6D], 1),
        0x6E =>         (vec![0x6E], 1),
        0x6F =>         (vec![0x6F], 1),
        0x70 =>         (vec![0x70], 1),
        0x71 =>         (vec![0x71], 1),
        0x72 =>         (vec![0x72], 1),
        0x73 =>         (vec![0x73], 1),
        0x74 =>         (vec![0x74], 1),
        0x75 =>         (vec![0x75], 1),
        0x76 =>         (vec![0x76], 1),
        0x77 =>         (vec![0x77], 1),
        0x78 =>         (vec![0x78], 1),
        0x79 =>         (vec![0x79], 1),
        0x7A =>         (vec![0x7A], 1),
        0x7B =>         (vec![0x7B], 1),
        0x7C =>         (vec![0x7C], 1),
        0x7D =>         (vec![0x7D], 1),
        0x7E =>         (vec![0x7E], 1),
        0x7F =>         (vec![0x7F], 1),
        0x80 =>         (vec![0x80], 1),
        0x81 =>         (vec![0x81], 1),
        0x82 =>         (vec![0x82], 1),
        0x83 =>         (vec![0x83], 1),
        0x84 =>         (vec![0x84], 1),
        0x85 =>         (vec![0x85], 1),
        0x86 =>         (vec![0x86], 1),
        0x87 =>         (vec![0x87], 1),
        0x88 =>         (vec![0x88], 1),
        0x89 =>         (vec![0x89], 1),
        0x8A =>         (vec![0x8A], 1),
        0x8B =>         (vec![0x8B], 1),
        0x8C =>         (vec![0x8C], 1),
        0x8D =>         (vec![0x8D], 1),
        0x8E =>         (vec![0x8E], 1),
        0x8F =>         (vec![0x8F], 1),
        0x90 =>         (vec![0x90], 1),
        0x91 =>         (vec![0x91], 1),
        0x92 =>         (vec![0x92], 1),
        0x93 =>         (vec![0x93], 1),
        0x94 =>         (vec![0x94], 1),
        0x95 =>         (vec![0x95], 1),
        0x96 =>         (vec![0x96], 1),
        0x97 =>         (vec![0x97], 1),
        0x98 =>         (vec![0x98], 1),
        0x99 =>         (vec![0x99], 1),
        0x9A =>         (vec![0x9A], 1),
        0x9B =>         (vec![0x9B], 1),
        0x9C =>         (vec![0x9C], 1),
        0x9D =>         (vec![0x9D], 1),
        0x9E =>         (vec![0x9E], 1),
        0x9F =>         (vec![0x9F], 1),
        0xA0 =>         (vec![0xA0], 1),
        0xA1 =>         (vec![0xA1], 1),
        0xA2 =>         (vec![0xA2], 1),
        0xA3 =>         (vec![0xA3], 1),
        0xA4 =>         (vec![0xA4], 1),
        0xA5 =>         (vec![0xA5], 1),
        0xA6 =>         (vec![0xA6], 1),
        0xA7 =>         (vec![0xA7], 1),
        0xA8 =>         (vec![0xA8], 1),
        0xA9 =>         (vec![0xA9], 1),
        0xAA =>         (vec![0xAA], 1),
        0xAB =>         (vec![0xAB], 1),
        0xAC =>         (vec![0xAC], 1),
        0xAD =>         (vec![0xAD], 1),
        0xAE =>         (vec![0xAE], 1),
        0xAF =>         (vec![0xAF], 1),
        0xB0 =>         (vec![0xB0], 1),
        0xB1 =>         (vec![0xB1], 1),
        0xB2 =>         (vec![0xB2], 1),
        0xB3 =>         (vec![0xB3], 1),
        0xB4 =>         (vec![0xB4], 1),
        0xB5 =>         (vec![0xB5], 1),
        0xB6 =>         (vec![0xB6], 1),
        0xB7 =>         (vec![0xB7], 1),
        0xB8 =>         (vec![0xB8], 1),
        0xB9 =>         (vec![0xB9], 1),
        0xBA =>         (vec![0xBA], 1),
        0xBB =>         (vec![0xBB], 1),
        0xBC =>         (vec![0xBC], 1),
        0xBD =>         (vec![0xBD], 1),
        0xBE =>         (vec![0xBE], 1),
        0xBF =>         (vec![0xBF], 1),
        0xC0 =>         (vec![0xC0], 1),
        0xC1 =>         (vec![0xC1], 1),
        0xC2 =>         (vec![0xC2], 3),
        0xC3 =>         (vec![0xC3], 3),
        0xC4 =>         (vec![0xC4], 3),
        0xC5 =>         (vec![0xC5], 1),
        0xC6 =>         (vec![0xC6], 2),
        0xC7 =>         (vec![0xC7], 1),
        0xC8 =>         (vec![0xC8], 1),
        0xC9 =>         (vec![0xC9], 1),
        0xCA =>         (vec![0xCA], 3),
        0xCC =>         (vec![0xCC], 3),
        0xCD =>         (vec![0xCD], 3),
        0xCE =>         (vec![0xCE], 2),
        0xCF =>         (vec![0xCF], 1),
        0xD0 =>         (vec![0xD0], 1),
        0xD1 =>         (vec![0xD1], 1),
        0xD2 =>         (vec![0xD2], 3),
        0xD4 =>         (vec![0xD4], 3),
        0xD5 =>         (vec![0xD5], 1),
        0xD6 =>         (vec![0xD6], 2),
        0xD7 =>         (vec![0xD7], 1),
        0xD8 =>         (vec![0xD8], 1),
        0xD9 =>         (vec![0xD9], 1),
        0xDA =>         (vec![0xDA], 3),
        0xDC =>         (vec![0xDC], 3),
        0xDE =>         (vec![0xDE], 2),
        0xDF =>         (vec![0xDF], 1),
        0xE0 =>         (vec![0xE0], 2),
        0xE1 =>         (vec![0xE1], 1),
        0xE2 =>         (vec![0xE2], 1),
        0xE5 =>         (vec![0xE5], 1),
        0xE6 =>         (vec![0xE6], 2),
        0xE7 =>         (vec![0xE7], 1),
        0xE8 =>         (vec![0xE8], 2),
        0xE9 =>         (vec![0xE9], 1),
        0xEA =>         (vec![0xEA], 3),
        0xEE =>         (vec![0xEE], 2),
        0xEF =>         (vec![0xEF], 1),
        0xF0 =>         (vec![0xF0], 2),
        0xF1 =>         (vec![0xF1], 1),
        0xF2 =>         (vec![0xF2], 1),
        0xF3 =>         (vec![0xF3], 1),
        0xF5 =>         (vec![0xF5], 1),
        0xF6 =>         (vec![0xF6], 2),
        0xF7 =>         (vec![0xF7], 1),
        0xF8 =>         (vec![0xF8], 2),
        0xF9 =>         (vec![0xF9], 1),
        0xFA =>         (vec![0xFA], 3),
        0xFB =>         (vec![0xFB], 1),
        0xFE =>         (vec![0xFE], 2),
        0xFF =>         (vec![0xFF], 1),
        0x10 => match (0x10 as u16) << 8 |
            match read_u8(&mut stream) { Ok(x) => x, _ => return None } as u16{
            0x1000 =>             (vec![0x10, 0x00], 2),
            _ => return None
        },
        0xCB => match (0xCB as u16) << 8 |
            match read_u8(&mut stream) { Ok(x) => x, _ => return None } as u16{
            0xCB00 =>             (vec![0xCB, 0x00], 2),
            0xCB01 =>             (vec![0xCB, 0x01], 2),
            0xCB02 =>             (vec![0xCB, 0x02], 2),
            0xCB03 =>             (vec![0xCB, 0x03], 2),
            0xCB04 =>             (vec![0xCB, 0x04], 2),
            0xCB05 =>             (vec![0xCB, 0x05], 2),
            0xCB06 =>             (vec![0xCB, 0x06], 2),
            0xCB07 =>             (vec![0xCB, 0x07], 2),
            0xCB08 =>             (vec![0xCB, 0x08], 2),
            0xCB09 =>             (vec![0xCB, 0x09], 2),
            0xCB0A =>             (vec![0xCB, 0x0A], 2),
            0xCB0B =>             (vec![0xCB, 0x0B], 2),
            0xCB0C =>             (vec![0xCB, 0x0C], 2),
            0xCB0D =>             (vec![0xCB, 0x0D], 2),
            0xCB0E =>             (vec![0xCB, 0x0E], 2),
            0xCB0F =>             (vec![0xCB, 0x0F], 2),
            0xCB10 =>             (vec![0xCB, 0x10], 2),
            0xCB11 =>             (vec![0xCB, 0x11], 2),
            0xCB12 =>             (vec![0xCB, 0x12], 2),
            0xCB13 =>             (vec![0xCB, 0x13], 2),
            0xCB14 =>             (vec![0xCB, 0x14], 2),
            0xCB15 =>             (vec![0xCB, 0x15], 2),
            0xCB16 =>             (vec![0xCB, 0x16], 2),
            0xCB17 =>             (vec![0xCB, 0x17], 2),
            0xCB18 =>             (vec![0xCB, 0x18], 2),
            0xCB19 =>             (vec![0xCB, 0x19], 2),
            0xCB1A =>             (vec![0xCB, 0x1A], 2),
            0xCB1B =>             (vec![0xCB, 0x1B], 2),
            0xCB1C =>             (vec![0xCB, 0x1C], 2),
            0xCB1D =>             (vec![0xCB, 0x1D], 2),
            0xCB1E =>             (vec![0xCB, 0x1E], 2),
            0xCB1F =>             (vec![0xCB, 0x1F], 2),
            0xCB20 =>             (vec![0xCB, 0x20], 2),
            0xCB21 =>             (vec![0xCB, 0x21], 2),
            0xCB22 =>             (vec![0xCB, 0x22], 2),
            0xCB23 =>             (vec![0xCB, 0x23], 2),
            0xCB24 =>             (vec![0xCB, 0x24], 2),
            0xCB25 =>             (vec![0xCB, 0x25], 2),
            0xCB26 =>             (vec![0xCB, 0x26], 2),
            0xCB27 =>             (vec![0xCB, 0x27], 2),
            0xCB28 =>             (vec![0xCB, 0x28], 2),
            0xCB29 =>             (vec![0xCB, 0x29], 2),
            0xCB2A =>             (vec![0xCB, 0x2A], 2),
            0xCB2B =>             (vec![0xCB, 0x2B], 2),
            0xCB2C =>             (vec![0xCB, 0x2C], 2),
            0xCB2D =>             (vec![0xCB, 0x2D], 2),
            0xCB2E =>             (vec![0xCB, 0x2E], 2),
            0xCB2F =>             (vec![0xCB, 0x2F], 2),
            0xCB30 =>             (vec![0xCB, 0x30], 2),
            0xCB31 =>             (vec![0xCB, 0x31], 2),
            0xCB32 =>             (vec![0xCB, 0x32], 2),
            0xCB33 =>             (vec![0xCB, 0x33], 2),
            0xCB34 =>             (vec![0xCB, 0x34], 2),
            0xCB35 =>             (vec![0xCB, 0x35], 2),
            0xCB36 =>             (vec![0xCB, 0x36], 2),
            0xCB37 =>             (vec![0xCB, 0x37], 2),
            0xCB38 =>             (vec![0xCB, 0x38], 2),
            0xCB39 =>             (vec![0xCB, 0x39], 2),
            0xCB3A =>             (vec![0xCB, 0x3A], 2),
            0xCB3B =>             (vec![0xCB, 0x3B], 2),
            0xCB3C =>             (vec![0xCB, 0x3C], 2),
            0xCB3D =>             (vec![0xCB, 0x3D], 2),
            0xCB3E =>             (vec![0xCB, 0x3E], 2),
            0xCB3F =>             (vec![0xCB, 0x3F], 2),
            0xCB40 =>             (vec![0xCB, 0x40], 2),
            0xCB41 =>             (vec![0xCB, 0x41], 2),
            0xCB42 =>             (vec![0xCB, 0x42], 2),
            0xCB43 =>             (vec![0xCB, 0x43], 2),
            0xCB44 =>             (vec![0xCB, 0x44], 2),
            0xCB45 =>             (vec![0xCB, 0x45], 2),
            0xCB46 =>             (vec![0xCB, 0x46], 2),
            0xCB47 =>             (vec![0xCB, 0x47], 2),
            0xCB48 =>             (vec![0xCB, 0x48], 2),
            0xCB49 =>             (vec![0xCB, 0x49], 2),
            0xCB4A =>             (vec![0xCB, 0x4A], 2),
            0xCB4B =>             (vec![0xCB, 0x4B], 2),
            0xCB4C =>             (vec![0xCB, 0x4C], 2),
            0xCB4D =>             (vec![0xCB, 0x4D], 2),
            0xCB4E =>             (vec![0xCB, 0x4E], 2),
            0xCB4F =>             (vec![0xCB, 0x4F], 2),
            0xCB50 =>             (vec![0xCB, 0x50], 2),
            0xCB51 =>             (vec![0xCB, 0x51], 2),
            0xCB52 =>             (vec![0xCB, 0x52], 2),
            0xCB53 =>             (vec![0xCB, 0x53], 2),
            0xCB54 =>             (vec![0xCB, 0x54], 2),
            0xCB55 =>             (vec![0xCB, 0x55], 2),
            0xCB56 =>             (vec![0xCB, 0x56], 2),
            0xCB57 =>             (vec![0xCB, 0x57], 2),
            0xCB58 =>             (vec![0xCB, 0x58], 2),
            0xCB59 =>             (vec![0xCB, 0x59], 2),
            0xCB5A =>             (vec![0xCB, 0x5A], 2),
            0xCB5B =>             (vec![0xCB, 0x5B], 2),
            0xCB5C =>             (vec![0xCB, 0x5C], 2),
            0xCB5D =>             (vec![0xCB, 0x5D], 2),
            0xCB5E =>             (vec![0xCB, 0x5E], 2),
            0xCB5F =>             (vec![0xCB, 0x5F], 2),
            0xCB60 =>             (vec![0xCB, 0x60], 2),
            0xCB61 =>             (vec![0xCB, 0x61], 2),
            0xCB62 =>             (vec![0xCB, 0x62], 2),
            0xCB63 =>             (vec![0xCB, 0x63], 2),
            0xCB64 =>             (vec![0xCB, 0x64], 2),
            0xCB65 =>             (vec![0xCB, 0x65], 2),
            0xCB66 =>             (vec![0xCB, 0x66], 2),
            0xCB67 =>             (vec![0xCB, 0x67], 2),
            0xCB68 =>             (vec![0xCB, 0x68], 2),
            0xCB69 =>             (vec![0xCB, 0x69], 2),
            0xCB6A =>             (vec![0xCB, 0x6A], 2),
            0xCB6B =>             (vec![0xCB, 0x6B], 2),
            0xCB6C =>             (vec![0xCB, 0x6C], 2),
            0xCB6D =>             (vec![0xCB, 0x6D], 2),
            0xCB6E =>             (vec![0xCB, 0x6E], 2),
            0xCB6F =>             (vec![0xCB, 0x6F], 2),
            0xCB70 =>             (vec![0xCB, 0x70], 2),
            0xCB71 =>             (vec![0xCB, 0x71], 2),
            0xCB72 =>             (vec![0xCB, 0x72], 2),
            0xCB73 =>             (vec![0xCB, 0x73], 2),
            0xCB74 =>             (vec![0xCB, 0x74], 2),
            0xCB75 =>             (vec![0xCB, 0x75], 2),
            0xCB76 =>             (vec![0xCB, 0x76], 2),
            0xCB77 =>             (vec![0xCB, 0x77], 2),
            0xCB78 =>             (vec![0xCB, 0x78], 2),
            0xCB79 =>             (vec![0xCB, 0x79], 2),
            0xCB7A =>             (vec![0xCB, 0x7A], 2),
            0xCB7B =>             (vec![0xCB, 0x7B], 2),
            0xCB7C =>             (vec![0xCB, 0x7C], 2),
            0xCB7D =>             (vec![0xCB, 0x7D], 2),
            0xCB7E =>             (vec![0xCB, 0x7E], 2),
            0xCB7F =>             (vec![0xCB, 0x7F], 2),
            0xCB80 =>             (vec![0xCB, 0x80], 2),
            0xCB81 =>             (vec![0xCB, 0x81], 2),
            0xCB82 =>             (vec![0xCB, 0x82], 2),
            0xCB83 =>             (vec![0xCB, 0x83], 2),
            0xCB84 =>             (vec![0xCB, 0x84], 2),
            0xCB85 =>             (vec![0xCB, 0x85], 2),
            0xCB86 =>             (vec![0xCB, 0x86], 2),
            0xCB87 =>             (vec![0xCB, 0x87], 2),
            0xCB88 =>             (vec![0xCB, 0x88], 2),
            0xCB89 =>             (vec![0xCB, 0x89], 2),
            0xCB8A =>             (vec![0xCB, 0x8A], 2),
            0xCB8B =>             (vec![0xCB, 0x8B], 2),
            0xCB8C =>             (vec![0xCB, 0x8C], 2),
            0xCB8D =>             (vec![0xCB, 0x8D], 2),
            0xCB8E =>             (vec![0xCB, 0x8E], 2),
            0xCB8F =>             (vec![0xCB, 0x8F], 2),
            0xCB90 =>             (vec![0xCB, 0x90], 2),
            0xCB91 =>             (vec![0xCB, 0x91], 2),
            0xCB92 =>             (vec![0xCB, 0x92], 2),
            0xCB93 =>             (vec![0xCB, 0x93], 2),
            0xCB94 =>             (vec![0xCB, 0x94], 2),
            0xCB95 =>             (vec![0xCB, 0x95], 2),
            0xCB96 =>             (vec![0xCB, 0x96], 2),
            0xCB97 =>             (vec![0xCB, 0x97], 2),
            0xCB98 =>             (vec![0xCB, 0x98], 2),
            0xCB99 =>             (vec![0xCB, 0x99], 2),
            0xCB9A =>             (vec![0xCB, 0x9A], 2),
            0xCB9B =>             (vec![0xCB, 0x9B], 2),
            0xCB9C =>             (vec![0xCB, 0x9C], 2),
            0xCB9D =>             (vec![0xCB, 0x9D], 2),
            0xCB9E =>             (vec![0xCB, 0x9E], 2),
            0xCB9F =>             (vec![0xCB, 0x9F], 2),
            0xCBA0 =>             (vec![0xCB, 0xA0], 2),
            0xCBA1 =>             (vec![0xCB, 0xA1], 2),
            0xCBA2 =>             (vec![0xCB, 0xA2], 2),
            0xCBA3 =>             (vec![0xCB, 0xA3], 2),
            0xCBA4 =>             (vec![0xCB, 0xA4], 2),
            0xCBA5 =>             (vec![0xCB, 0xA5], 2),
            0xCBA6 =>             (vec![0xCB, 0xA6], 2),
            0xCBA7 =>             (vec![0xCB, 0xA7], 2),
            0xCBA8 =>             (vec![0xCB, 0xA8], 2),
            0xCBA9 =>             (vec![0xCB, 0xA9], 2),
            0xCBAA =>             (vec![0xCB, 0xAA], 2),
            0xCBAB =>             (vec![0xCB, 0xAB], 2),
            0xCBAC =>             (vec![0xCB, 0xAC], 2),
            0xCBAD =>             (vec![0xCB, 0xAD], 2),
            0xCBAE =>             (vec![0xCB, 0xAE], 2),
            0xCBAF =>             (vec![0xCB, 0xAF], 2),
            0xCBB0 =>             (vec![0xCB, 0xB0], 2),
            0xCBB1 =>             (vec![0xCB, 0xB1], 2),
            0xCBB2 =>             (vec![0xCB, 0xB2], 2),
            0xCBB3 =>             (vec![0xCB, 0xB3], 2),
            0xCBB4 =>             (vec![0xCB, 0xB4], 2),
            0xCBB5 =>             (vec![0xCB, 0xB5], 2),
            0xCBB6 =>             (vec![0xCB, 0xB6], 2),
            0xCBB7 =>             (vec![0xCB, 0xB7], 2),
            0xCBB8 =>             (vec![0xCB, 0xB8], 2),
            0xCBB9 =>             (vec![0xCB, 0xB9], 2),
            0xCBBA =>             (vec![0xCB, 0xBA], 2),
            0xCBBB =>             (vec![0xCB, 0xBB], 2),
            0xCBBC =>             (vec![0xCB, 0xBC], 2),
            0xCBBD =>             (vec![0xCB, 0xBD], 2),
            0xCBBE =>             (vec![0xCB, 0xBE], 2),
            0xCBBF =>             (vec![0xCB, 0xBF], 2),
            0xCBC0 =>             (vec![0xCB, 0xC0], 2),
            0xCBC1 =>             (vec![0xCB, 0xC1], 2),
            0xCBC2 =>             (vec![0xCB, 0xC2], 2),
            0xCBC3 =>             (vec![0xCB, 0xC3], 2),
            0xCBC4 =>             (vec![0xCB, 0xC4], 2),
            0xCBC5 =>             (vec![0xCB, 0xC5], 2),
            0xCBC6 =>             (vec![0xCB, 0xC6], 2),
            0xCBC7 =>             (vec![0xCB, 0xC7], 2),
            0xCBC8 =>             (vec![0xCB, 0xC8], 2),
            0xCBC9 =>             (vec![0xCB, 0xC9], 2),
            0xCBCA =>             (vec![0xCB, 0xCA], 2),
            0xCBCB =>             (vec![0xCB, 0xCB], 2),
            0xCBCC =>             (vec![0xCB, 0xCC], 2),
            0xCBCD =>             (vec![0xCB, 0xCD], 2),
            0xCBCE =>             (vec![0xCB, 0xCE], 2),
            0xCBCF =>             (vec![0xCB, 0xCF], 2),
            0xCBD0 =>             (vec![0xCB, 0xD0], 2),
            0xCBD1 =>             (vec![0xCB, 0xD1], 2),
            0xCBD2 =>             (vec![0xCB, 0xD2], 2),
            0xCBD3 =>             (vec![0xCB, 0xD3], 2),
            0xCBD4 =>             (vec![0xCB, 0xD4], 2),
            0xCBD5 =>             (vec![0xCB, 0xD5], 2),
            0xCBD6 =>             (vec![0xCB, 0xD6], 2),
            0xCBD7 =>             (vec![0xCB, 0xD7], 2),
            0xCBD8 =>             (vec![0xCB, 0xD8], 2),
            0xCBD9 =>             (vec![0xCB, 0xD9], 2),
            0xCBDA =>             (vec![0xCB, 0xDA], 2),
            0xCBDB =>             (vec![0xCB, 0xDB], 2),
            0xCBDC =>             (vec![0xCB, 0xDC], 2),
            0xCBDD =>             (vec![0xCB, 0xDD], 2),
            0xCBDE =>             (vec![0xCB, 0xDE], 2),
            0xCBDF =>             (vec![0xCB, 0xDF], 2),
            0xCBE0 =>             (vec![0xCB, 0xE0], 2),
            0xCBE1 =>             (vec![0xCB, 0xE1], 2),
            0xCBE2 =>             (vec![0xCB, 0xE2], 2),
            0xCBE3 =>             (vec![0xCB, 0xE3], 2),
            0xCBE4 =>             (vec![0xCB, 0xE4], 2),
            0xCBE5 =>             (vec![0xCB, 0xE5], 2),
            0xCBE6 =>             (vec![0xCB, 0xE6], 2),
            0xCBE7 =>             (vec![0xCB, 0xE7], 2),
            0xCBE8 =>             (vec![0xCB, 0xE8], 2),
            0xCBE9 =>             (vec![0xCB, 0xE9], 2),
            0xCBEA =>             (vec![0xCB, 0xEA], 2),
            0xCBEB =>             (vec![0xCB, 0xEB], 2),
            0xCBEC =>             (vec![0xCB, 0xEC], 2),
            0xCBED =>             (vec![0xCB, 0xED], 2),
            0xCBEE =>             (vec![0xCB, 0xEE], 2),
            0xCBEF =>             (vec![0xCB, 0xEF], 2),
            0xCBF0 =>             (vec![0xCB, 0xF0], 2),
            0xCBF1 =>             (vec![0xCB, 0xF1], 2),
            0xCBF2 =>             (vec![0xCB, 0xF2], 2),
            0xCBF3 =>             (vec![0xCB, 0xF3], 2),
            0xCBF4 =>             (vec![0xCB, 0xF4], 2),
            0xCBF5 =>             (vec![0xCB, 0xF5], 2),
            0xCBF6 =>             (vec![0xCB, 0xF6], 2),
            0xCBF7 =>             (vec![0xCB, 0xF7], 2),
            0xCBF8 =>             (vec![0xCB, 0xF8], 2),
            0xCBF9 =>             (vec![0xCB, 0xF9], 2),
            0xCBFA =>             (vec![0xCB, 0xFA], 2),
            0xCBFB =>             (vec![0xCB, 0xFB], 2),
            0xCBFC =>             (vec![0xCB, 0xFC], 2),
            0xCBFD =>             (vec![0xCB, 0xFD], 2),
            0xCBFE =>             (vec![0xCB, 0xFE], 2),
            0xCBFF =>             (vec![0xCB, 0xFF], 2),
            _ => return None
        },
        _ => return None
    };

    let op_size = instr.len();
    instr.resize(size, 0);
    stream.read(&mut instr[op_size..]).unwrap();
    return Some(instr);
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
