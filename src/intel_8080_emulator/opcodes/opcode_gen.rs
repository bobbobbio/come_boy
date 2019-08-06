use crate::emulator_common::Intel8080Register;
use crate::intel_8080_emulator::opcodes::Intel8080InstructionPrinter;
use crate::util::{read_u16, read_u8};
use std::io;
pub trait Intel8080InstructionSet {
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn call_if_carry(&mut self, address1: u16);
    fn call_if_minus(&mut self, address1: u16);
    fn call_if_no_carry(&mut self, address1: u16);
    fn call_if_not_zero(&mut self, address1: u16);
    fn call_if_parity_even(&mut self, address1: u16);
    fn call_if_parity_odd(&mut self, address1: u16);
    fn call_if_plus(&mut self, address1: u16);
    fn call_if_zero(&mut self, address1: u16);
    fn compare_immediate_with_accumulator(&mut self, data1: u8);
    fn compare_with_accumulator(&mut self, register1: Intel8080Register);
    fn complement_accumulator(&mut self);
    fn complement_carry(&mut self);
    fn decimal_adjust_accumulator(&mut self);
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register);
    fn decrement_register_pair(&mut self, register1: Intel8080Register);
    fn disable_interrupts(&mut self);
    fn double_add(&mut self, register1: Intel8080Register);
    fn enable_interrupts(&mut self);
    fn exchange_registers(&mut self);
    fn exchange_stack(&mut self);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn halt(&mut self);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn input(&mut self, data1: u8);
    fn jump(&mut self, address1: u16);
    fn jump_if_carry(&mut self, address1: u16);
    fn jump_if_minus(&mut self, address1: u16);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_parity_even(&mut self, address1: u16);
    fn jump_if_parity_odd(&mut self, address1: u16);
    fn jump_if_positive(&mut self, address1: u16);
    fn jump_if_zero(&mut self, address1: u16);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn load_h_and_l_direct(&mut self, address1: u16);
    fn load_program_counter(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn no_operation(&mut self);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn output(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn restart(&mut self, data1: u8);
    fn return_if_carry(&mut self);
    fn return_if_minus(&mut self);
    fn return_if_no_carry(&mut self);
    fn return_if_not_zero(&mut self);
    fn return_if_parity_even(&mut self);
    fn return_if_parity_odd(&mut self);
    fn return_if_plus(&mut self);
    fn return_if_zero(&mut self);
    fn return_unconditionally(&mut self);
    fn rim(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn set_carry(&mut self);
    fn sim(&mut self);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn store_h_and_l_direct(&mut self, address1: u16);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
}
pub fn dispatch_intel8080_instruction<I: Intel8080InstructionSet>(
    mut stream: &[u8],
    machine: &mut I,
) -> u8 {
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x00 => {
            machine.no_operation();
            0u8
        }
        0x01 => {
            machine
                .load_register_pair_immediate(Intel8080Register::B, read_u16(&mut stream).unwrap());
            0u8
        }
        0x02 => {
            machine.store_accumulator(Intel8080Register::B);
            0u8
        }
        0x03 => {
            machine.increment_register_pair(Intel8080Register::B);
            0u8
        }
        0x04 => {
            machine.increment_register_or_memory(Intel8080Register::B);
            0u8
        }
        0x05 => {
            machine.decrement_register_or_memory(Intel8080Register::B);
            0u8
        }
        0x06 => {
            machine.move_immediate_data(Intel8080Register::B, read_u8(&mut stream).unwrap());
            0u8
        }
        0x07 => {
            machine.rotate_accumulator_left();
            0u8
        }
        0x09 => {
            machine.double_add(Intel8080Register::B);
            0u8
        }
        0x0A => {
            machine.load_accumulator(Intel8080Register::B);
            0u8
        }
        0x0B => {
            machine.decrement_register_pair(Intel8080Register::B);
            0u8
        }
        0x0C => {
            machine.increment_register_or_memory(Intel8080Register::C);
            0u8
        }
        0x0D => {
            machine.decrement_register_or_memory(Intel8080Register::C);
            0u8
        }
        0x0E => {
            machine.move_immediate_data(Intel8080Register::C, read_u8(&mut stream).unwrap());
            0u8
        }
        0x0F => {
            machine.rotate_accumulator_right();
            0u8
        }
        0x11 => {
            machine
                .load_register_pair_immediate(Intel8080Register::D, read_u16(&mut stream).unwrap());
            0u8
        }
        0x12 => {
            machine.store_accumulator(Intel8080Register::D);
            0u8
        }
        0x13 => {
            machine.increment_register_pair(Intel8080Register::D);
            0u8
        }
        0x14 => {
            machine.increment_register_or_memory(Intel8080Register::D);
            0u8
        }
        0x15 => {
            machine.decrement_register_or_memory(Intel8080Register::D);
            0u8
        }
        0x16 => {
            machine.move_immediate_data(Intel8080Register::D, read_u8(&mut stream).unwrap());
            0u8
        }
        0x17 => {
            machine.rotate_accumulator_left_through_carry();
            0u8
        }
        0x19 => {
            machine.double_add(Intel8080Register::D);
            0u8
        }
        0x1A => {
            machine.load_accumulator(Intel8080Register::D);
            0u8
        }
        0x1B => {
            machine.decrement_register_pair(Intel8080Register::D);
            0u8
        }
        0x1C => {
            machine.increment_register_or_memory(Intel8080Register::E);
            0u8
        }
        0x1D => {
            machine.decrement_register_or_memory(Intel8080Register::E);
            0u8
        }
        0x1E => {
            machine.move_immediate_data(Intel8080Register::E, read_u8(&mut stream).unwrap());
            0u8
        }
        0x1F => {
            machine.rotate_accumulator_right_through_carry();
            0u8
        }
        0x20 => {
            machine.rim();
            0u8
        }
        0x21 => {
            machine
                .load_register_pair_immediate(Intel8080Register::H, read_u16(&mut stream).unwrap());
            0u8
        }
        0x22 => {
            machine.store_h_and_l_direct(read_u16(&mut stream).unwrap());
            0u8
        }
        0x23 => {
            machine.increment_register_pair(Intel8080Register::H);
            0u8
        }
        0x24 => {
            machine.increment_register_or_memory(Intel8080Register::H);
            0u8
        }
        0x25 => {
            machine.decrement_register_or_memory(Intel8080Register::H);
            0u8
        }
        0x26 => {
            machine.move_immediate_data(Intel8080Register::H, read_u8(&mut stream).unwrap());
            0u8
        }
        0x27 => {
            machine.decimal_adjust_accumulator();
            0u8
        }
        0x29 => {
            machine.double_add(Intel8080Register::H);
            0u8
        }
        0x2A => {
            machine.load_h_and_l_direct(read_u16(&mut stream).unwrap());
            0u8
        }
        0x2B => {
            machine.decrement_register_pair(Intel8080Register::H);
            0u8
        }
        0x2C => {
            machine.increment_register_or_memory(Intel8080Register::L);
            0u8
        }
        0x2D => {
            machine.decrement_register_or_memory(Intel8080Register::L);
            0u8
        }
        0x2E => {
            machine.move_immediate_data(Intel8080Register::L, read_u8(&mut stream).unwrap());
            0u8
        }
        0x2F => {
            machine.complement_accumulator();
            0u8
        }
        0x30 => {
            machine.sim();
            0u8
        }
        0x31 => {
            machine.load_register_pair_immediate(
                Intel8080Register::SP,
                read_u16(&mut stream).unwrap(),
            );
            0u8
        }
        0x32 => {
            machine.store_accumulator_direct(read_u16(&mut stream).unwrap());
            0u8
        }
        0x33 => {
            machine.increment_register_pair(Intel8080Register::SP);
            0u8
        }
        0x34 => {
            machine.increment_register_or_memory(Intel8080Register::M);
            0u8
        }
        0x35 => {
            machine.decrement_register_or_memory(Intel8080Register::M);
            0u8
        }
        0x36 => {
            machine.move_immediate_data(Intel8080Register::M, read_u8(&mut stream).unwrap());
            0u8
        }
        0x37 => {
            machine.set_carry();
            0u8
        }
        0x39 => {
            machine.double_add(Intel8080Register::SP);
            0u8
        }
        0x3A => {
            machine.load_accumulator_direct(read_u16(&mut stream).unwrap());
            0u8
        }
        0x3B => {
            machine.decrement_register_pair(Intel8080Register::SP);
            0u8
        }
        0x3C => {
            machine.increment_register_or_memory(Intel8080Register::A);
            0u8
        }
        0x3D => {
            machine.decrement_register_or_memory(Intel8080Register::A);
            0u8
        }
        0x3E => {
            machine.move_immediate_data(Intel8080Register::A, read_u8(&mut stream).unwrap());
            0u8
        }
        0x3F => {
            machine.complement_carry();
            0u8
        }
        0x40 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::B);
            0u8
        }
        0x41 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::C);
            0u8
        }
        0x42 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::D);
            0u8
        }
        0x43 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::E);
            0u8
        }
        0x44 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::H);
            0u8
        }
        0x45 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::L);
            0u8
        }
        0x46 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::M);
            0u8
        }
        0x47 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::A);
            0u8
        }
        0x48 => {
            machine.move_data(Intel8080Register::C, Intel8080Register::B);
            0u8
        }
        0x49 => {
            machine.move_data(Intel8080Register::C, Intel8080Register::C);
            0u8
        }
        0x4A => {
            machine.move_data(Intel8080Register::C, Intel8080Register::D);
            0u8
        }
        0x4B => {
            machine.move_data(Intel8080Register::C, Intel8080Register::E);
            0u8
        }
        0x4C => {
            machine.move_data(Intel8080Register::C, Intel8080Register::H);
            0u8
        }
        0x4D => {
            machine.move_data(Intel8080Register::C, Intel8080Register::L);
            0u8
        }
        0x4E => {
            machine.move_data(Intel8080Register::C, Intel8080Register::M);
            0u8
        }
        0x4F => {
            machine.move_data(Intel8080Register::C, Intel8080Register::A);
            0u8
        }
        0x50 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::B);
            0u8
        }
        0x51 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::C);
            0u8
        }
        0x52 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::D);
            0u8
        }
        0x53 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::E);
            0u8
        }
        0x54 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::H);
            0u8
        }
        0x55 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::L);
            0u8
        }
        0x56 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::M);
            0u8
        }
        0x57 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::A);
            0u8
        }
        0x58 => {
            machine.move_data(Intel8080Register::E, Intel8080Register::B);
            0u8
        }
        0x59 => {
            machine.move_data(Intel8080Register::E, Intel8080Register::C);
            0u8
        }
        0x5A => {
            machine.move_data(Intel8080Register::E, Intel8080Register::D);
            0u8
        }
        0x5B => {
            machine.move_data(Intel8080Register::E, Intel8080Register::E);
            0u8
        }
        0x5C => {
            machine.move_data(Intel8080Register::E, Intel8080Register::H);
            0u8
        }
        0x5D => {
            machine.move_data(Intel8080Register::E, Intel8080Register::L);
            0u8
        }
        0x5E => {
            machine.move_data(Intel8080Register::E, Intel8080Register::M);
            0u8
        }
        0x5F => {
            machine.move_data(Intel8080Register::E, Intel8080Register::A);
            0u8
        }
        0x60 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::B);
            0u8
        }
        0x61 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::C);
            0u8
        }
        0x62 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::D);
            0u8
        }
        0x63 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::E);
            0u8
        }
        0x64 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::H);
            0u8
        }
        0x65 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::L);
            0u8
        }
        0x66 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::M);
            0u8
        }
        0x67 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::A);
            0u8
        }
        0x68 => {
            machine.move_data(Intel8080Register::L, Intel8080Register::B);
            0u8
        }
        0x69 => {
            machine.move_data(Intel8080Register::L, Intel8080Register::C);
            0u8
        }
        0x6A => {
            machine.move_data(Intel8080Register::L, Intel8080Register::D);
            0u8
        }
        0x6B => {
            machine.move_data(Intel8080Register::L, Intel8080Register::E);
            0u8
        }
        0x6C => {
            machine.move_data(Intel8080Register::L, Intel8080Register::H);
            0u8
        }
        0x6D => {
            machine.move_data(Intel8080Register::L, Intel8080Register::L);
            0u8
        }
        0x6E => {
            machine.move_data(Intel8080Register::L, Intel8080Register::M);
            0u8
        }
        0x6F => {
            machine.move_data(Intel8080Register::L, Intel8080Register::A);
            0u8
        }
        0x70 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::B);
            0u8
        }
        0x71 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::C);
            0u8
        }
        0x72 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::D);
            0u8
        }
        0x73 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::E);
            0u8
        }
        0x74 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::H);
            0u8
        }
        0x75 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::L);
            0u8
        }
        0x76 => {
            machine.halt();
            0u8
        }
        0x77 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::A);
            0u8
        }
        0x78 => {
            machine.move_data(Intel8080Register::A, Intel8080Register::B);
            0u8
        }
        0x79 => {
            machine.move_data(Intel8080Register::A, Intel8080Register::C);
            0u8
        }
        0x7A => {
            machine.move_data(Intel8080Register::A, Intel8080Register::D);
            0u8
        }
        0x7B => {
            machine.move_data(Intel8080Register::A, Intel8080Register::E);
            0u8
        }
        0x7C => {
            machine.move_data(Intel8080Register::A, Intel8080Register::H);
            0u8
        }
        0x7D => {
            machine.move_data(Intel8080Register::A, Intel8080Register::L);
            0u8
        }
        0x7E => {
            machine.move_data(Intel8080Register::A, Intel8080Register::M);
            0u8
        }
        0x7F => {
            machine.move_data(Intel8080Register::A, Intel8080Register::A);
            0u8
        }
        0x80 => {
            machine.add_to_accumulator(Intel8080Register::B);
            0u8
        }
        0x81 => {
            machine.add_to_accumulator(Intel8080Register::C);
            0u8
        }
        0x82 => {
            machine.add_to_accumulator(Intel8080Register::D);
            0u8
        }
        0x83 => {
            machine.add_to_accumulator(Intel8080Register::E);
            0u8
        }
        0x84 => {
            machine.add_to_accumulator(Intel8080Register::H);
            0u8
        }
        0x85 => {
            machine.add_to_accumulator(Intel8080Register::L);
            0u8
        }
        0x86 => {
            machine.add_to_accumulator(Intel8080Register::M);
            0u8
        }
        0x87 => {
            machine.add_to_accumulator(Intel8080Register::A);
            0u8
        }
        0x88 => {
            machine.add_to_accumulator_with_carry(Intel8080Register::B);
            0u8
        }
        0x89 => {
            machine.add_to_accumulator_with_carry(Intel8080Register::C);
            0u8
        }
        0x8A => {
            machine.add_to_accumulator_with_carry(Intel8080Register::D);
            0u8
        }
        0x8B => {
            machine.add_to_accumulator_with_carry(Intel8080Register::E);
            0u8
        }
        0x8C => {
            machine.add_to_accumulator_with_carry(Intel8080Register::H);
            0u8
        }
        0x8D => {
            machine.add_to_accumulator_with_carry(Intel8080Register::L);
            0u8
        }
        0x8E => {
            machine.add_to_accumulator_with_carry(Intel8080Register::M);
            0u8
        }
        0x8F => {
            machine.add_to_accumulator_with_carry(Intel8080Register::A);
            0u8
        }
        0x90 => {
            machine.subtract_from_accumulator(Intel8080Register::B);
            0u8
        }
        0x91 => {
            machine.subtract_from_accumulator(Intel8080Register::C);
            0u8
        }
        0x92 => {
            machine.subtract_from_accumulator(Intel8080Register::D);
            0u8
        }
        0x93 => {
            machine.subtract_from_accumulator(Intel8080Register::E);
            0u8
        }
        0x94 => {
            machine.subtract_from_accumulator(Intel8080Register::H);
            0u8
        }
        0x95 => {
            machine.subtract_from_accumulator(Intel8080Register::L);
            0u8
        }
        0x96 => {
            machine.subtract_from_accumulator(Intel8080Register::M);
            0u8
        }
        0x97 => {
            machine.subtract_from_accumulator(Intel8080Register::A);
            0u8
        }
        0x98 => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::B);
            0u8
        }
        0x99 => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::C);
            0u8
        }
        0x9A => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::D);
            0u8
        }
        0x9B => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::E);
            0u8
        }
        0x9C => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::H);
            0u8
        }
        0x9D => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::L);
            0u8
        }
        0x9E => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::M);
            0u8
        }
        0x9F => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::A);
            0u8
        }
        0xA0 => {
            machine.logical_and_with_accumulator(Intel8080Register::B);
            0u8
        }
        0xA1 => {
            machine.logical_and_with_accumulator(Intel8080Register::C);
            0u8
        }
        0xA2 => {
            machine.logical_and_with_accumulator(Intel8080Register::D);
            0u8
        }
        0xA3 => {
            machine.logical_and_with_accumulator(Intel8080Register::E);
            0u8
        }
        0xA4 => {
            machine.logical_and_with_accumulator(Intel8080Register::H);
            0u8
        }
        0xA5 => {
            machine.logical_and_with_accumulator(Intel8080Register::L);
            0u8
        }
        0xA6 => {
            machine.logical_and_with_accumulator(Intel8080Register::M);
            0u8
        }
        0xA7 => {
            machine.logical_and_with_accumulator(Intel8080Register::A);
            0u8
        }
        0xA8 => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::B);
            0u8
        }
        0xA9 => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::C);
            0u8
        }
        0xAA => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::D);
            0u8
        }
        0xAB => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::E);
            0u8
        }
        0xAC => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::H);
            0u8
        }
        0xAD => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::L);
            0u8
        }
        0xAE => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::M);
            0u8
        }
        0xAF => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::A);
            0u8
        }
        0xB0 => {
            machine.logical_or_with_accumulator(Intel8080Register::B);
            0u8
        }
        0xB1 => {
            machine.logical_or_with_accumulator(Intel8080Register::C);
            0u8
        }
        0xB2 => {
            machine.logical_or_with_accumulator(Intel8080Register::D);
            0u8
        }
        0xB3 => {
            machine.logical_or_with_accumulator(Intel8080Register::E);
            0u8
        }
        0xB4 => {
            machine.logical_or_with_accumulator(Intel8080Register::H);
            0u8
        }
        0xB5 => {
            machine.logical_or_with_accumulator(Intel8080Register::L);
            0u8
        }
        0xB6 => {
            machine.logical_or_with_accumulator(Intel8080Register::M);
            0u8
        }
        0xB7 => {
            machine.logical_or_with_accumulator(Intel8080Register::A);
            0u8
        }
        0xB8 => {
            machine.compare_with_accumulator(Intel8080Register::B);
            0u8
        }
        0xB9 => {
            machine.compare_with_accumulator(Intel8080Register::C);
            0u8
        }
        0xBA => {
            machine.compare_with_accumulator(Intel8080Register::D);
            0u8
        }
        0xBB => {
            machine.compare_with_accumulator(Intel8080Register::E);
            0u8
        }
        0xBC => {
            machine.compare_with_accumulator(Intel8080Register::H);
            0u8
        }
        0xBD => {
            machine.compare_with_accumulator(Intel8080Register::L);
            0u8
        }
        0xBE => {
            machine.compare_with_accumulator(Intel8080Register::M);
            0u8
        }
        0xBF => {
            machine.compare_with_accumulator(Intel8080Register::A);
            0u8
        }
        0xC0 => {
            machine.return_if_not_zero();
            0u8
        }
        0xC1 => {
            machine.pop_data_off_stack(Intel8080Register::B);
            0u8
        }
        0xC2 => {
            machine.jump_if_not_zero(read_u16(&mut stream).unwrap());
            0u8
        }
        0xC3 => {
            machine.jump(read_u16(&mut stream).unwrap());
            0u8
        }
        0xC4 => {
            machine.call_if_not_zero(read_u16(&mut stream).unwrap());
            0u8
        }
        0xC5 => {
            machine.push_data_onto_stack(Intel8080Register::B);
            0u8
        }
        0xC6 => {
            machine.add_immediate_to_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xC7 => {
            machine.restart(0u8);
            0u8
        }
        0xC8 => {
            machine.return_if_zero();
            0u8
        }
        0xC9 => {
            machine.return_unconditionally();
            0u8
        }
        0xCA => {
            machine.jump_if_zero(read_u16(&mut stream).unwrap());
            0u8
        }
        0xCC => {
            machine.call_if_zero(read_u16(&mut stream).unwrap());
            0u8
        }
        0xCD => {
            machine.call(read_u16(&mut stream).unwrap());
            0u8
        }
        0xCE => {
            machine.add_immediate_to_accumulator_with_carry(read_u8(&mut stream).unwrap());
            0u8
        }
        0xCF => {
            machine.restart(1u8);
            0u8
        }
        0xD0 => {
            machine.return_if_no_carry();
            0u8
        }
        0xD1 => {
            machine.pop_data_off_stack(Intel8080Register::D);
            0u8
        }
        0xD2 => {
            machine.jump_if_no_carry(read_u16(&mut stream).unwrap());
            0u8
        }
        0xD3 => {
            machine.output(read_u8(&mut stream).unwrap());
            0u8
        }
        0xD4 => {
            machine.call_if_no_carry(read_u16(&mut stream).unwrap());
            0u8
        }
        0xD5 => {
            machine.push_data_onto_stack(Intel8080Register::D);
            0u8
        }
        0xD6 => {
            machine.subtract_immediate_from_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xD7 => {
            machine.restart(2u8);
            0u8
        }
        0xD8 => {
            machine.return_if_carry();
            0u8
        }
        0xDA => {
            machine.jump_if_carry(read_u16(&mut stream).unwrap());
            0u8
        }
        0xDB => {
            machine.input(read_u8(&mut stream).unwrap());
            0u8
        }
        0xDC => {
            machine.call_if_carry(read_u16(&mut stream).unwrap());
            0u8
        }
        0xDE => {
            machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).unwrap());
            0u8
        }
        0xDF => {
            machine.restart(3u8);
            0u8
        }
        0xE0 => {
            machine.return_if_parity_odd();
            0u8
        }
        0xE1 => {
            machine.pop_data_off_stack(Intel8080Register::H);
            0u8
        }
        0xE2 => {
            machine.jump_if_parity_odd(read_u16(&mut stream).unwrap());
            0u8
        }
        0xE3 => {
            machine.exchange_stack();
            0u8
        }
        0xE4 => {
            machine.call_if_parity_odd(read_u16(&mut stream).unwrap());
            0u8
        }
        0xE5 => {
            machine.push_data_onto_stack(Intel8080Register::H);
            0u8
        }
        0xE6 => {
            machine.and_immediate_with_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xE7 => {
            machine.restart(4u8);
            0u8
        }
        0xE8 => {
            machine.return_if_parity_even();
            0u8
        }
        0xE9 => {
            machine.load_program_counter();
            0u8
        }
        0xEA => {
            machine.jump_if_parity_even(read_u16(&mut stream).unwrap());
            0u8
        }
        0xEB => {
            machine.exchange_registers();
            0u8
        }
        0xEC => {
            machine.call_if_parity_even(read_u16(&mut stream).unwrap());
            0u8
        }
        0xEE => {
            machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xEF => {
            machine.restart(5u8);
            0u8
        }
        0xF0 => {
            machine.return_if_plus();
            0u8
        }
        0xF1 => {
            machine.pop_data_off_stack(Intel8080Register::PSW);
            0u8
        }
        0xF2 => {
            machine.jump_if_positive(read_u16(&mut stream).unwrap());
            0u8
        }
        0xF3 => {
            machine.disable_interrupts();
            0u8
        }
        0xF4 => {
            machine.call_if_plus(read_u16(&mut stream).unwrap());
            0u8
        }
        0xF5 => {
            machine.push_data_onto_stack(Intel8080Register::PSW);
            0u8
        }
        0xF6 => {
            machine.or_immediate_with_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xF7 => {
            machine.restart(6u8);
            0u8
        }
        0xF8 => {
            machine.return_if_minus();
            0u8
        }
        0xF9 => {
            machine.load_sp_from_h_and_l();
            0u8
        }
        0xFA => {
            machine.jump_if_minus(read_u16(&mut stream).unwrap());
            0u8
        }
        0xFB => {
            machine.enable_interrupts();
            0u8
        }
        0xFC => {
            machine.call_if_minus(read_u16(&mut stream).unwrap());
            0u8
        }
        0xFE => {
            machine.compare_immediate_with_accumulator(read_u8(&mut stream).unwrap());
            0u8
        }
        0xFF => {
            machine.restart(7u8);
            0u8
        }
        v => panic!("Unknown opcode {}", v),
    }
}
pub fn get_intel8080_instruction<R: io::Read>(mut stream: R) -> Option<Vec<u8>> {
    let (mut instr, size) = match read_u8(&mut stream).unwrap() {
        0x00 => (vec![0x00], 1u8),
        0x01 => (vec![0x01], 3u8),
        0x02 => (vec![0x02], 1u8),
        0x03 => (vec![0x03], 1u8),
        0x04 => (vec![0x04], 1u8),
        0x05 => (vec![0x05], 1u8),
        0x06 => (vec![0x06], 2u8),
        0x07 => (vec![0x07], 1u8),
        0x09 => (vec![0x09], 1u8),
        0x0A => (vec![0x0A], 1u8),
        0x0B => (vec![0x0B], 1u8),
        0x0C => (vec![0x0C], 1u8),
        0x0D => (vec![0x0D], 1u8),
        0x0E => (vec![0x0E], 2u8),
        0x0F => (vec![0x0F], 1u8),
        0x11 => (vec![0x11], 3u8),
        0x12 => (vec![0x12], 1u8),
        0x13 => (vec![0x13], 1u8),
        0x14 => (vec![0x14], 1u8),
        0x15 => (vec![0x15], 1u8),
        0x16 => (vec![0x16], 2u8),
        0x17 => (vec![0x17], 1u8),
        0x19 => (vec![0x19], 1u8),
        0x1A => (vec![0x1A], 1u8),
        0x1B => (vec![0x1B], 1u8),
        0x1C => (vec![0x1C], 1u8),
        0x1D => (vec![0x1D], 1u8),
        0x1E => (vec![0x1E], 2u8),
        0x1F => (vec![0x1F], 1u8),
        0x20 => (vec![0x20], 1u8),
        0x21 => (vec![0x21], 3u8),
        0x22 => (vec![0x22], 3u8),
        0x23 => (vec![0x23], 1u8),
        0x24 => (vec![0x24], 1u8),
        0x25 => (vec![0x25], 1u8),
        0x26 => (vec![0x26], 2u8),
        0x27 => (vec![0x27], 1u8),
        0x29 => (vec![0x29], 1u8),
        0x2A => (vec![0x2A], 3u8),
        0x2B => (vec![0x2B], 1u8),
        0x2C => (vec![0x2C], 1u8),
        0x2D => (vec![0x2D], 1u8),
        0x2E => (vec![0x2E], 2u8),
        0x2F => (vec![0x2F], 1u8),
        0x30 => (vec![0x30], 1u8),
        0x31 => (vec![0x31], 3u8),
        0x32 => (vec![0x32], 3u8),
        0x33 => (vec![0x33], 1u8),
        0x34 => (vec![0x34], 1u8),
        0x35 => (vec![0x35], 1u8),
        0x36 => (vec![0x36], 2u8),
        0x37 => (vec![0x37], 1u8),
        0x39 => (vec![0x39], 1u8),
        0x3A => (vec![0x3A], 3u8),
        0x3B => (vec![0x3B], 1u8),
        0x3C => (vec![0x3C], 1u8),
        0x3D => (vec![0x3D], 1u8),
        0x3E => (vec![0x3E], 2u8),
        0x3F => (vec![0x3F], 1u8),
        0x40 => (vec![0x40], 1u8),
        0x41 => (vec![0x41], 1u8),
        0x42 => (vec![0x42], 1u8),
        0x43 => (vec![0x43], 1u8),
        0x44 => (vec![0x44], 1u8),
        0x45 => (vec![0x45], 1u8),
        0x46 => (vec![0x46], 1u8),
        0x47 => (vec![0x47], 1u8),
        0x48 => (vec![0x48], 1u8),
        0x49 => (vec![0x49], 1u8),
        0x4A => (vec![0x4A], 1u8),
        0x4B => (vec![0x4B], 1u8),
        0x4C => (vec![0x4C], 1u8),
        0x4D => (vec![0x4D], 1u8),
        0x4E => (vec![0x4E], 1u8),
        0x4F => (vec![0x4F], 1u8),
        0x50 => (vec![0x50], 1u8),
        0x51 => (vec![0x51], 1u8),
        0x52 => (vec![0x52], 1u8),
        0x53 => (vec![0x53], 1u8),
        0x54 => (vec![0x54], 1u8),
        0x55 => (vec![0x55], 1u8),
        0x56 => (vec![0x56], 1u8),
        0x57 => (vec![0x57], 1u8),
        0x58 => (vec![0x58], 1u8),
        0x59 => (vec![0x59], 1u8),
        0x5A => (vec![0x5A], 1u8),
        0x5B => (vec![0x5B], 1u8),
        0x5C => (vec![0x5C], 1u8),
        0x5D => (vec![0x5D], 1u8),
        0x5E => (vec![0x5E], 1u8),
        0x5F => (vec![0x5F], 1u8),
        0x60 => (vec![0x60], 1u8),
        0x61 => (vec![0x61], 1u8),
        0x62 => (vec![0x62], 1u8),
        0x63 => (vec![0x63], 1u8),
        0x64 => (vec![0x64], 1u8),
        0x65 => (vec![0x65], 1u8),
        0x66 => (vec![0x66], 1u8),
        0x67 => (vec![0x67], 1u8),
        0x68 => (vec![0x68], 1u8),
        0x69 => (vec![0x69], 1u8),
        0x6A => (vec![0x6A], 1u8),
        0x6B => (vec![0x6B], 1u8),
        0x6C => (vec![0x6C], 1u8),
        0x6D => (vec![0x6D], 1u8),
        0x6E => (vec![0x6E], 1u8),
        0x6F => (vec![0x6F], 1u8),
        0x70 => (vec![0x70], 1u8),
        0x71 => (vec![0x71], 1u8),
        0x72 => (vec![0x72], 1u8),
        0x73 => (vec![0x73], 1u8),
        0x74 => (vec![0x74], 1u8),
        0x75 => (vec![0x75], 1u8),
        0x76 => (vec![0x76], 1u8),
        0x77 => (vec![0x77], 1u8),
        0x78 => (vec![0x78], 1u8),
        0x79 => (vec![0x79], 1u8),
        0x7A => (vec![0x7A], 1u8),
        0x7B => (vec![0x7B], 1u8),
        0x7C => (vec![0x7C], 1u8),
        0x7D => (vec![0x7D], 1u8),
        0x7E => (vec![0x7E], 1u8),
        0x7F => (vec![0x7F], 1u8),
        0x80 => (vec![0x80], 1u8),
        0x81 => (vec![0x81], 1u8),
        0x82 => (vec![0x82], 1u8),
        0x83 => (vec![0x83], 1u8),
        0x84 => (vec![0x84], 1u8),
        0x85 => (vec![0x85], 1u8),
        0x86 => (vec![0x86], 1u8),
        0x87 => (vec![0x87], 1u8),
        0x88 => (vec![0x88], 1u8),
        0x89 => (vec![0x89], 1u8),
        0x8A => (vec![0x8A], 1u8),
        0x8B => (vec![0x8B], 1u8),
        0x8C => (vec![0x8C], 1u8),
        0x8D => (vec![0x8D], 1u8),
        0x8E => (vec![0x8E], 1u8),
        0x8F => (vec![0x8F], 1u8),
        0x90 => (vec![0x90], 1u8),
        0x91 => (vec![0x91], 1u8),
        0x92 => (vec![0x92], 1u8),
        0x93 => (vec![0x93], 1u8),
        0x94 => (vec![0x94], 1u8),
        0x95 => (vec![0x95], 1u8),
        0x96 => (vec![0x96], 1u8),
        0x97 => (vec![0x97], 1u8),
        0x98 => (vec![0x98], 1u8),
        0x99 => (vec![0x99], 1u8),
        0x9A => (vec![0x9A], 1u8),
        0x9B => (vec![0x9B], 1u8),
        0x9C => (vec![0x9C], 1u8),
        0x9D => (vec![0x9D], 1u8),
        0x9E => (vec![0x9E], 1u8),
        0x9F => (vec![0x9F], 1u8),
        0xA0 => (vec![0xA0], 1u8),
        0xA1 => (vec![0xA1], 1u8),
        0xA2 => (vec![0xA2], 1u8),
        0xA3 => (vec![0xA3], 1u8),
        0xA4 => (vec![0xA4], 1u8),
        0xA5 => (vec![0xA5], 1u8),
        0xA6 => (vec![0xA6], 1u8),
        0xA7 => (vec![0xA7], 1u8),
        0xA8 => (vec![0xA8], 1u8),
        0xA9 => (vec![0xA9], 1u8),
        0xAA => (vec![0xAA], 1u8),
        0xAB => (vec![0xAB], 1u8),
        0xAC => (vec![0xAC], 1u8),
        0xAD => (vec![0xAD], 1u8),
        0xAE => (vec![0xAE], 1u8),
        0xAF => (vec![0xAF], 1u8),
        0xB0 => (vec![0xB0], 1u8),
        0xB1 => (vec![0xB1], 1u8),
        0xB2 => (vec![0xB2], 1u8),
        0xB3 => (vec![0xB3], 1u8),
        0xB4 => (vec![0xB4], 1u8),
        0xB5 => (vec![0xB5], 1u8),
        0xB6 => (vec![0xB6], 1u8),
        0xB7 => (vec![0xB7], 1u8),
        0xB8 => (vec![0xB8], 1u8),
        0xB9 => (vec![0xB9], 1u8),
        0xBA => (vec![0xBA], 1u8),
        0xBB => (vec![0xBB], 1u8),
        0xBC => (vec![0xBC], 1u8),
        0xBD => (vec![0xBD], 1u8),
        0xBE => (vec![0xBE], 1u8),
        0xBF => (vec![0xBF], 1u8),
        0xC0 => (vec![0xC0], 1u8),
        0xC1 => (vec![0xC1], 1u8),
        0xC2 => (vec![0xC2], 3u8),
        0xC3 => (vec![0xC3], 3u8),
        0xC4 => (vec![0xC4], 3u8),
        0xC5 => (vec![0xC5], 1u8),
        0xC6 => (vec![0xC6], 2u8),
        0xC7 => (vec![0xC7], 1u8),
        0xC8 => (vec![0xC8], 1u8),
        0xC9 => (vec![0xC9], 1u8),
        0xCA => (vec![0xCA], 3u8),
        0xCC => (vec![0xCC], 3u8),
        0xCD => (vec![0xCD], 3u8),
        0xCE => (vec![0xCE], 2u8),
        0xCF => (vec![0xCF], 1u8),
        0xD0 => (vec![0xD0], 1u8),
        0xD1 => (vec![0xD1], 1u8),
        0xD2 => (vec![0xD2], 3u8),
        0xD3 => (vec![0xD3], 2u8),
        0xD4 => (vec![0xD4], 3u8),
        0xD5 => (vec![0xD5], 1u8),
        0xD6 => (vec![0xD6], 2u8),
        0xD7 => (vec![0xD7], 1u8),
        0xD8 => (vec![0xD8], 1u8),
        0xDA => (vec![0xDA], 3u8),
        0xDB => (vec![0xDB], 2u8),
        0xDC => (vec![0xDC], 3u8),
        0xDE => (vec![0xDE], 2u8),
        0xDF => (vec![0xDF], 1u8),
        0xE0 => (vec![0xE0], 1u8),
        0xE1 => (vec![0xE1], 1u8),
        0xE2 => (vec![0xE2], 3u8),
        0xE3 => (vec![0xE3], 1u8),
        0xE4 => (vec![0xE4], 3u8),
        0xE5 => (vec![0xE5], 1u8),
        0xE6 => (vec![0xE6], 2u8),
        0xE7 => (vec![0xE7], 1u8),
        0xE8 => (vec![0xE8], 1u8),
        0xE9 => (vec![0xE9], 1u8),
        0xEA => (vec![0xEA], 3u8),
        0xEB => (vec![0xEB], 1u8),
        0xEC => (vec![0xEC], 3u8),
        0xEE => (vec![0xEE], 2u8),
        0xEF => (vec![0xEF], 1u8),
        0xF0 => (vec![0xF0], 1u8),
        0xF1 => (vec![0xF1], 1u8),
        0xF2 => (vec![0xF2], 3u8),
        0xF3 => (vec![0xF3], 1u8),
        0xF4 => (vec![0xF4], 3u8),
        0xF5 => (vec![0xF5], 1u8),
        0xF6 => (vec![0xF6], 2u8),
        0xF7 => (vec![0xF7], 1u8),
        0xF8 => (vec![0xF8], 1u8),
        0xF9 => (vec![0xF9], 1u8),
        0xFA => (vec![0xFA], 3u8),
        0xFB => (vec![0xFB], 1u8),
        0xFC => (vec![0xFC], 3u8),
        0xFE => (vec![0xFE], 2u8),
        0xFF => (vec![0xFF], 1u8),
        _ => return None,
    };
    let op_size = instr.len();
    instr.resize(size as usize, 0);
    stream.read(&mut instr[op_size..]).unwrap();
    return Some(instr);
}
impl<'a> Intel8080InstructionSet for Intel8080InstructionPrinter<'a> {
    fn add_immediate_to_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADI", data1);
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ACI", data1);
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADD", register1);
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADC", register1);
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ANI", data1);
    }
    fn call(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CALL", address1);
    }
    fn call_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CC", address1);
    }
    fn call_if_minus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CM", address1);
    }
    fn call_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNC", address1);
    }
    fn call_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNZ", address1);
    }
    fn call_if_parity_even(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPE", address1);
    }
    fn call_if_parity_odd(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPO", address1);
    }
    fn call_if_plus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CP", address1);
    }
    fn call_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CZ", address1);
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "CPI", data1);
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "CMP", register1);
    }
    fn complement_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CMA");
    }
    fn complement_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CMC");
    }
    fn decimal_adjust_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCR", register1);
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCX", register1);
    }
    fn disable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DI");
    }
    fn double_add(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DAD", register1);
    }
    fn enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "EI");
    }
    fn exchange_registers(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "XCHG");
    }
    fn exchange_stack(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "XTHL");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "XRI", data1);
    }
    fn halt(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INR", register1);
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INX", register1);
    }
    fn input(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "IN", data1);
    }
    fn jump(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JMP", address1);
    }
    fn jump_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JC", address1);
    }
    fn jump_if_minus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JM", address1);
    }
    fn jump_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNC", address1);
    }
    fn jump_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNZ", address1);
    }
    fn jump_if_parity_even(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPE", address1);
    }
    fn jump_if_parity_odd(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPO", address1);
    }
    fn jump_if_positive(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JP", address1);
    }
    fn jump_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JZ", address1);
    }
    fn load_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "LDAX", register1);
    }
    fn load_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDA", address1);
    }
    fn load_h_and_l_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LHLD", address1);
    }
    fn load_program_counter(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "PCHL");
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} #${:02x}",
            "LXI", register1, data2
        );
    }
    fn load_sp_from_h_and_l(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SPHL");
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ANA", register1);
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "XRA", register1);
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ORA", register1);
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} {:?}",
            "MOV", register1, register2
        );
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} #${:02x}",
            "MVI", register1, data2
        );
    }
    fn no_operation(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "NOP");
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ORI", data1);
    }
    fn output(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "OUT", data1);
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "POP", register1);
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "PUSH", register1);
    }
    fn restart(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} {}", "RST", data1);
    }
    fn return_if_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn return_if_minus(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RM");
    }
    fn return_if_no_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn return_if_not_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn return_if_parity_even(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RPE");
    }
    fn return_if_parity_odd(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RPO");
    }
    fn return_if_plus(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RP");
    }
    fn return_if_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn return_unconditionally(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RET");
    }
    fn rim(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RIM");
    }
    fn rotate_accumulator_left(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RLC");
    }
    fn rotate_accumulator_left_through_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RAL");
    }
    fn rotate_accumulator_right(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RRC");
    }
    fn rotate_accumulator_right_through_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RAR");
    }
    fn set_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STC");
    }
    fn sim(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SIM");
    }
    fn store_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "STAX", register1);
    }
    fn store_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn store_h_and_l_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SHLD", address1);
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SUB", register1);
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SBB", register1);
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SUI", data1);
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SBI", data1);
    }
}
