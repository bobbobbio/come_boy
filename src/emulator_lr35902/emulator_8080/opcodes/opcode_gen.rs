
use emulator_lr35902::emulator_8080::opcodes::{
    read_u16, read_u8, Register8080, OpcodePrinter8080};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcodes/opcode_gen.py
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
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
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

pub fn dispatch_8080_opcode<I: InstructionSet8080>(
    mut stream: &[u8],
    machine: &mut I)
{
    match read_u8(&mut stream).ok().expect("") {
        0x3e => machine.move_immediate_data(Register8080::A, read_u8(&mut stream).ok().expect("")),
        0x3d => machine.decrement_register_or_memory(Register8080::A),
        0xe4 => machine.call_if_parity_odd(read_u16(&mut stream).ok().expect("")),
        0x3f => machine.complement_carry(),
        0x3a => machine.load_accumulator_direct(read_u16(&mut stream).ok().expect("")),
        0x3c => machine.increment_register_or_memory(Register8080::A),
        0x3b => machine.decrement_register_pair(Register8080::SP),
        0xff => machine.restart(7 as u8),
        0xfa => machine.jump_if_minus(read_u16(&mut stream).ok().expect("")),
        0xda => machine.jump_if_carry(read_u16(&mut stream).ok().expect("")),
        0xec => machine.call_if_parity_even(read_u16(&mut stream).ok().expect("")),
        0x28 => machine.not_implemented(),
        0x29 => machine.double_add(Register8080::H),
        0xcf => machine.restart(1 as u8),
        0xf8 => machine.return_if_minus(),
        0xeb => machine.exchange_registers(),
        0x22 => machine.store_h_and_l_direct(read_u16(&mut stream).ok().expect("")),
        0x23 => machine.increment_register_pair(Register8080::H),
        0x20 => machine.rim(),
        0x21 => machine.load_register_pair_immediate(Register8080::H, read_u16(&mut stream).ok().expect("")),
        0x26 => machine.move_immediate_data(Register8080::H, read_u8(&mut stream).ok().expect("")),
        0x27 => machine.decimal_adjust_accumulator(),
        0x24 => machine.increment_register_or_memory(Register8080::H),
        0x25 => machine.decrement_register_or_memory(Register8080::H),
        0xdb => machine.input(read_u8(&mut stream).ok().expect("")),
        0xef => machine.restart(5 as u8),
        0xe2 => machine.jump_if_parity_odd(read_u16(&mut stream).ok().expect("")),
        0xee => machine.exclusive_or_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")),
        0xed => machine.not_implemented(),
        0xdc => machine.call_if_carry(read_u16(&mut stream).ok().expect("")),
        0x35 => machine.decrement_register_or_memory(Register8080::M),
        0x34 => machine.increment_register_or_memory(Register8080::M),
        0x37 => machine.set_carry(),
        0x36 => machine.move_immediate_data(Register8080::M, read_u8(&mut stream).ok().expect("")),
        0x31 => machine.load_register_pair_immediate(Register8080::SP, read_u16(&mut stream).ok().expect("")),
        0x30 => machine.sim(),
        0x33 => machine.increment_register_pair(Register8080::SP),
        0x32 => machine.store_accumulator_direct(read_u16(&mut stream).ok().expect("")),
        0xd4 => machine.call_if_no_carry(read_u16(&mut stream).ok().expect("")),
        0xe8 => machine.return_if_parity_even(),
        0x39 => machine.double_add(Register8080::SP),
        0x38 => machine.not_implemented(),
        0xc0 => machine.return_if_not_zero(),
        0xe1 => machine.pop_data_off_stack(Register8080::H),
        0xfe => machine.compare_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")),
        0x88 => machine.add_to_accumulator_with_carry(Register8080::B),
        0xdd => machine.not_implemented(),
        0x89 => machine.add_to_accumulator_with_carry(Register8080::C),
        0x2b => machine.decrement_register_pair(Register8080::H),
        0x2c => machine.increment_register_or_memory(Register8080::L),
        0xfd => machine.not_implemented(),
        0x2a => machine.load_h_and_l_direct(read_u16(&mut stream).ok().expect("")),
        0x2f => machine.complement_accumulator(),
        0xfc => machine.call_if_minus(read_u16(&mut stream).ok().expect("")),
        0x2d => machine.decrement_register_or_memory(Register8080::L),
        0x2e => machine.move_immediate_data(Register8080::L, read_u8(&mut stream).ok().expect("")),
        0x5c => machine.move_data(Register8080::E, Register8080::H),
        0x5b => machine.move_data(Register8080::E, Register8080::E),
        0x5a => machine.move_data(Register8080::E, Register8080::D),
        0xba => machine.compare_with_accumulator(Register8080::D),
        0x5f => machine.move_data(Register8080::E, Register8080::A),
        0x5e => machine.move_data(Register8080::E, Register8080::M),
        0x5d => machine.move_data(Register8080::E, Register8080::L),
        0xc9 => machine.return_unconditionally(),
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
        0xaf => machine.logical_exclusive_or_with_accumulator(Register8080::A),
        0xae => machine.logical_exclusive_or_with_accumulator(Register8080::M),
        0xad => machine.logical_exclusive_or_with_accumulator(Register8080::L),
        0xac => machine.logical_exclusive_or_with_accumulator(Register8080::H),
        0xab => machine.logical_exclusive_or_with_accumulator(Register8080::E),
        0xaa => machine.logical_exclusive_or_with_accumulator(Register8080::D),
        0xe6 => machine.and_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")),
        0xea => machine.jump_if_parity_even(read_u16(&mut stream).ok().expect("")),
        0x4a => machine.move_data(Register8080::C, Register8080::D),
        0x4b => machine.move_data(Register8080::C, Register8080::E),
        0x4c => machine.move_data(Register8080::C, Register8080::H),
        0x4d => machine.move_data(Register8080::C, Register8080::L),
        0x4e => machine.move_data(Register8080::C, Register8080::M),
        0x4f => machine.move_data(Register8080::C, Register8080::A),
        0x53 => machine.move_data(Register8080::D, Register8080::E),
        0x52 => machine.move_data(Register8080::D, Register8080::D),
        0x51 => machine.move_data(Register8080::D, Register8080::C),
        0x50 => machine.move_data(Register8080::D, Register8080::B),
        0x57 => machine.move_data(Register8080::D, Register8080::A),
        0x56 => machine.move_data(Register8080::D, Register8080::M),
        0x55 => machine.move_data(Register8080::D, Register8080::L),
        0x54 => machine.move_data(Register8080::D, Register8080::H),
        0xe5 => machine.push_data_onto_stack(Register8080::H),
        0x59 => machine.move_data(Register8080::E, Register8080::C),
        0x58 => machine.move_data(Register8080::E, Register8080::B),
        0xf4 => machine.call_if_plus(read_u16(&mut stream).ok().expect("")),
        0xfb => machine.enable_interrupts(),
        0xf9 => machine.load_sp_from_h_and_l(),
        0xf6 => machine.or_immediate_with_accumulator(read_u8(&mut stream).ok().expect("")),
        0xa9 => machine.logical_exclusive_or_with_accumulator(Register8080::C),
        0xa8 => machine.logical_exclusive_or_with_accumulator(Register8080::B),
        0xa7 => machine.logical_and_with_accumulator(Register8080::A),
        0xa6 => machine.logical_and_with_accumulator(Register8080::M),
        0xa5 => machine.logical_and_with_accumulator(Register8080::L),
        0xa4 => machine.logical_and_with_accumulator(Register8080::H),
        0xa3 => machine.logical_and_with_accumulator(Register8080::E),
        0xa2 => machine.logical_and_with_accumulator(Register8080::D),
        0xa1 => machine.logical_and_with_accumulator(Register8080::C),
        0xa0 => machine.logical_and_with_accumulator(Register8080::B),
        0xf5 => machine.push_data_onto_stack(Register8080::PSW),
        0x7a => machine.move_data(Register8080::A, Register8080::D),
        0xf2 => machine.jump_if_positive(read_u16(&mut stream).ok().expect("")),
        0x7c => machine.move_data(Register8080::A, Register8080::H),
        0x7b => machine.move_data(Register8080::A, Register8080::E),
        0x7e => machine.move_data(Register8080::A, Register8080::M),
        0x7d => machine.move_data(Register8080::A, Register8080::L),
        0x7f => machine.move_data(Register8080::A, Register8080::A),
        0xf0 => machine.return_if_plus(),
        0x68 => machine.move_data(Register8080::L, Register8080::B),
        0x69 => machine.move_data(Register8080::L, Register8080::C),
        0x66 => machine.move_data(Register8080::H, Register8080::M),
        0x67 => machine.move_data(Register8080::H, Register8080::A),
        0x64 => machine.move_data(Register8080::H, Register8080::H),
        0x65 => machine.move_data(Register8080::H, Register8080::L),
        0x62 => machine.move_data(Register8080::H, Register8080::D),
        0x63 => machine.move_data(Register8080::H, Register8080::E),
        0x60 => machine.move_data(Register8080::H, Register8080::B),
        0x61 => machine.move_data(Register8080::H, Register8080::C),
        0x99 => machine.subtract_from_accumulator_with_borrow(Register8080::C),
        0xd5 => machine.push_data_onto_stack(Register8080::D),
        0xce => machine.add_immediate_to_accumulator_with_carry(read_u8(&mut stream).ok().expect("")),
        0xcd => machine.call(read_u16(&mut stream).ok().expect("")),
        0xb8 => machine.compare_with_accumulator(Register8080::B),
        0xb9 => machine.compare_with_accumulator(Register8080::C),
        0xca => machine.jump_if_zero(read_u16(&mut stream).ok().expect("")),
        0xcc => machine.call_if_zero(read_u16(&mut stream).ok().expect("")),
        0xcb => machine.not_implemented(),
        0xb2 => machine.logical_or_with_accumulator(Register8080::D),
        0xb3 => machine.logical_or_with_accumulator(Register8080::E),
        0xb0 => machine.logical_or_with_accumulator(Register8080::B),
        0xb1 => machine.logical_or_with_accumulator(Register8080::C),
        0xb6 => machine.logical_or_with_accumulator(Register8080::M),
        0xb7 => machine.logical_or_with_accumulator(Register8080::A),
        0xb4 => machine.logical_or_with_accumulator(Register8080::H),
        0xb5 => machine.logical_or_with_accumulator(Register8080::L),
        0xe3 => machine.exchange_stack(),
        0xd6 => machine.subtract_immediate_from_accumulator(read_u8(&mut stream).ok().expect("")),
        0x6f => machine.move_data(Register8080::L, Register8080::A),
        0x6d => machine.move_data(Register8080::L, Register8080::L),
        0x6e => machine.move_data(Register8080::L, Register8080::M),
        0x6b => machine.move_data(Register8080::L, Register8080::E),
        0x6c => machine.move_data(Register8080::L, Register8080::H),
        0x6a => machine.move_data(Register8080::L, Register8080::D),
        0x79 => machine.move_data(Register8080::A, Register8080::C),
        0x78 => machine.move_data(Register8080::A, Register8080::B),
        0x71 => machine.move_data(Register8080::M, Register8080::C),
        0x70 => machine.move_data(Register8080::M, Register8080::B),
        0x73 => machine.move_data(Register8080::M, Register8080::E),
        0x72 => machine.move_data(Register8080::M, Register8080::D),
        0x75 => machine.move_data(Register8080::M, Register8080::L),
        0x74 => machine.move_data(Register8080::M, Register8080::H),
        0x77 => machine.move_data(Register8080::M, Register8080::A),
        0x76 => machine.halt(),
        0xc5 => machine.push_data_onto_stack(Register8080::B),
        0xc4 => machine.call_if_not_zero(read_u16(&mut stream).ok().expect("")),
        0xc7 => machine.restart(0 as u8),
        0xc6 => machine.add_immediate_to_accumulator(read_u8(&mut stream).ok().expect("")),
        0xc1 => machine.pop_data_off_stack(Register8080::B),
        0x8b => machine.add_to_accumulator_with_carry(Register8080::E),
        0xc3 => machine.jump(read_u16(&mut stream).ok().expect("")),
        0xc2 => machine.jump_if_not_zero(read_u16(&mut stream).ok().expect("")),
        0xbb => machine.compare_with_accumulator(Register8080::E),
        0xbc => machine.compare_with_accumulator(Register8080::H),
        0x8c => machine.add_to_accumulator_with_carry(Register8080::H),
        0xbf => machine.compare_with_accumulator(Register8080::A),
        0xc8 => machine.return_if_zero(),
        0xbd => machine.compare_with_accumulator(Register8080::L),
        0xbe => machine.compare_with_accumulator(Register8080::M),
        0xf1 => machine.pop_data_off_stack(Register8080::PSW),
        0xe9 => machine.load_program_counter(),
        0xd8 => machine.return_if_carry(),
        0xd9 => machine.not_implemented(),
        0xf7 => machine.restart(6 as u8),
        0xf3 => machine.disable_interrupts(),
        0xd0 => machine.return_if_no_carry(),
        0x9f => machine.subtract_from_accumulator_with_borrow(Register8080::A),
        0x9e => machine.subtract_from_accumulator_with_borrow(Register8080::M),
        0x9d => machine.subtract_from_accumulator_with_borrow(Register8080::L),
        0x08 => machine.not_implemented(),
        0x09 => machine.double_add(Register8080::B),
        0x9a => machine.subtract_from_accumulator_with_borrow(Register8080::D),
        0xd7 => machine.restart(2 as u8),
        0x04 => machine.increment_register_or_memory(Register8080::B),
        0x05 => machine.decrement_register_or_memory(Register8080::B),
        0x06 => machine.move_immediate_data(Register8080::B, read_u8(&mut stream).ok().expect("")),
        0x07 => machine.rotate_accumulator_left(),
        0x00 => machine.no_instruction(),
        0x01 => machine.load_register_pair_immediate(Register8080::B, read_u16(&mut stream).ok().expect("")),
        0x02 => machine.store_accumulator(Register8080::B),
        0x03 => machine.increment_register_pair(Register8080::B),
        0x84 => machine.add_to_accumulator(Register8080::H),
        0x85 => machine.add_to_accumulator(Register8080::L),
        0x86 => machine.add_to_accumulator(Register8080::M),
        0x87 => machine.add_to_accumulator(Register8080::A),
        0x80 => machine.add_to_accumulator(Register8080::B),
        0x81 => machine.add_to_accumulator(Register8080::C),
        0x82 => machine.add_to_accumulator(Register8080::D),
        0x83 => machine.add_to_accumulator(Register8080::E),
        0x1f => machine.rotate_accumulator_right_through_carry(),
        0x1e => machine.move_immediate_data(Register8080::E, read_u8(&mut stream).ok().expect("")),
        0x1d => machine.decrement_register_or_memory(Register8080::E),
        0x1c => machine.increment_register_or_memory(Register8080::E),
        0x1b => machine.decrement_register_pair(Register8080::D),
        0x1a => machine.load_accumulator(Register8080::D),
        0xde => machine.subtract_immediate_from_accumulator_with_borrow(read_u8(&mut stream).ok().expect("")),
        0xdf => machine.restart(3 as u8),
        0xd1 => machine.pop_data_off_stack(Register8080::D),
        0xd2 => machine.jump_if_no_carry(read_u16(&mut stream).ok().expect("")),
        0xd3 => machine.output(read_u8(&mut stream).ok().expect("")),
        0x9c => machine.subtract_from_accumulator_with_borrow(Register8080::H),
        0x9b => machine.subtract_from_accumulator_with_borrow(Register8080::E),
        0x8d => machine.add_to_accumulator_with_carry(Register8080::L),
        0x8e => machine.add_to_accumulator_with_carry(Register8080::M),
        0x8f => machine.add_to_accumulator_with_carry(Register8080::A),
        0xe0 => machine.return_if_parity_odd(),
        0xe7 => machine.restart(4 as u8),
        0x8a => machine.add_to_accumulator_with_carry(Register8080::D),
        0x19 => machine.double_add(Register8080::D),
        0x18 => machine.not_implemented(),
        0x17 => machine.rotate_accumulator_left_through_carry(),
        0x16 => machine.move_immediate_data(Register8080::D, read_u8(&mut stream).ok().expect("")),
        0x15 => machine.decrement_register_or_memory(Register8080::D),
        0x14 => machine.increment_register_or_memory(Register8080::D),
        0x13 => machine.increment_register_pair(Register8080::D),
        0x12 => machine.store_accumulator(Register8080::D),
        0x11 => machine.load_register_pair_immediate(Register8080::D, read_u16(&mut stream).ok().expect("")),
        0x10 => machine.not_implemented(),
        0x97 => machine.subtract_from_accumulator(Register8080::A),
        0x96 => machine.subtract_from_accumulator(Register8080::M),
        0x95 => machine.subtract_from_accumulator(Register8080::L),
        0x94 => machine.subtract_from_accumulator(Register8080::H),
        0x93 => machine.subtract_from_accumulator(Register8080::E),
        0x92 => machine.subtract_from_accumulator(Register8080::D),
        0x91 => machine.subtract_from_accumulator(Register8080::C),
        0x90 => machine.subtract_from_accumulator(Register8080::B),
        0x0d => machine.decrement_register_or_memory(Register8080::C),
        0x0e => machine.move_immediate_data(Register8080::C, read_u8(&mut stream).ok().expect("")),
        0x0f => machine.rotate_accumulator_right(),
        0x98 => machine.subtract_from_accumulator_with_borrow(Register8080::B),
        0x0a => machine.load_accumulator(Register8080::B),
        0x0b => machine.decrement_register_pair(Register8080::B),
        0x0c => machine.increment_register_or_memory(Register8080::C),

        _ => panic!("Unknown opcode")
   };
}

pub fn get_8080_opcode_size(opcode: u8) -> u8
{
    match opcode {
        0x3e => 2,
        0x3d => 1,
        0xe4 => 3,
        0x3f => 1,
        0x3a => 3,
        0x3c => 1,
        0x3b => 1,
        0xff => 1,
        0xfa => 3,
        0xda => 3,
        0xec => 3,
        0x28 => 1,
        0x29 => 1,
        0xcf => 1,
        0xf8 => 1,
        0xeb => 1,
        0x22 => 3,
        0x23 => 1,
        0x20 => 1,
        0x21 => 3,
        0x26 => 2,
        0x27 => 1,
        0x24 => 1,
        0x25 => 1,
        0xdb => 2,
        0xef => 1,
        0xe2 => 3,
        0xee => 2,
        0xed => 1,
        0xdc => 3,
        0x35 => 1,
        0x34 => 1,
        0x37 => 1,
        0x36 => 2,
        0x31 => 3,
        0x30 => 1,
        0x33 => 1,
        0x32 => 3,
        0xd4 => 3,
        0xe8 => 1,
        0x39 => 1,
        0x38 => 1,
        0xc0 => 1,
        0xe1 => 1,
        0xfe => 2,
        0x88 => 1,
        0xdd => 1,
        0x89 => 1,
        0x2b => 1,
        0x2c => 1,
        0xfd => 1,
        0x2a => 3,
        0x2f => 1,
        0xfc => 3,
        0x2d => 1,
        0x2e => 2,
        0x5c => 1,
        0x5b => 1,
        0x5a => 1,
        0xba => 1,
        0x5f => 1,
        0x5e => 1,
        0x5d => 1,
        0xc9 => 1,
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
        0xaf => 1,
        0xae => 1,
        0xad => 1,
        0xac => 1,
        0xab => 1,
        0xaa => 1,
        0xe6 => 2,
        0xea => 3,
        0x4a => 1,
        0x4b => 1,
        0x4c => 1,
        0x4d => 1,
        0x4e => 1,
        0x4f => 1,
        0x53 => 1,
        0x52 => 1,
        0x51 => 1,
        0x50 => 1,
        0x57 => 1,
        0x56 => 1,
        0x55 => 1,
        0x54 => 1,
        0xe5 => 1,
        0x59 => 1,
        0x58 => 1,
        0xf4 => 3,
        0xfb => 1,
        0xf9 => 1,
        0xf6 => 2,
        0xa9 => 1,
        0xa8 => 1,
        0xa7 => 1,
        0xa6 => 1,
        0xa5 => 1,
        0xa4 => 1,
        0xa3 => 1,
        0xa2 => 1,
        0xa1 => 1,
        0xa0 => 1,
        0xf5 => 1,
        0x7a => 1,
        0xf2 => 3,
        0x7c => 1,
        0x7b => 1,
        0x7e => 1,
        0x7d => 1,
        0x7f => 1,
        0xf0 => 1,
        0x68 => 1,
        0x69 => 1,
        0x66 => 1,
        0x67 => 1,
        0x64 => 1,
        0x65 => 1,
        0x62 => 1,
        0x63 => 1,
        0x60 => 1,
        0x61 => 1,
        0x99 => 1,
        0xd5 => 1,
        0xce => 2,
        0xcd => 3,
        0xb8 => 1,
        0xb9 => 1,
        0xca => 3,
        0xcc => 3,
        0xcb => 1,
        0xb2 => 1,
        0xb3 => 1,
        0xb0 => 1,
        0xb1 => 1,
        0xb6 => 1,
        0xb7 => 1,
        0xb4 => 1,
        0xb5 => 1,
        0xe3 => 1,
        0xd6 => 2,
        0x6f => 1,
        0x6d => 1,
        0x6e => 1,
        0x6b => 1,
        0x6c => 1,
        0x6a => 1,
        0x79 => 1,
        0x78 => 1,
        0x71 => 1,
        0x70 => 1,
        0x73 => 1,
        0x72 => 1,
        0x75 => 1,
        0x74 => 1,
        0x77 => 1,
        0x76 => 1,
        0xc5 => 1,
        0xc4 => 3,
        0xc7 => 1,
        0xc6 => 2,
        0xc1 => 1,
        0x8b => 1,
        0xc3 => 3,
        0xc2 => 3,
        0xbb => 1,
        0xbc => 1,
        0x8c => 1,
        0xbf => 1,
        0xc8 => 1,
        0xbd => 1,
        0xbe => 1,
        0xf1 => 1,
        0xe9 => 1,
        0xd8 => 1,
        0xd9 => 1,
        0xf7 => 1,
        0xf3 => 1,
        0xd0 => 1,
        0x9f => 1,
        0x9e => 1,
        0x9d => 1,
        0x08 => 1,
        0x09 => 1,
        0x9a => 1,
        0xd7 => 1,
        0x04 => 1,
        0x05 => 1,
        0x06 => 2,
        0x07 => 1,
        0x00 => 1,
        0x01 => 3,
        0x02 => 1,
        0x03 => 1,
        0x84 => 1,
        0x85 => 1,
        0x86 => 1,
        0x87 => 1,
        0x80 => 1,
        0x81 => 1,
        0x82 => 1,
        0x83 => 1,
        0x1f => 1,
        0x1e => 2,
        0x1d => 1,
        0x1c => 1,
        0x1b => 1,
        0x1a => 1,
        0xde => 2,
        0xdf => 1,
        0xd1 => 1,
        0xd2 => 3,
        0xd3 => 2,
        0x9c => 1,
        0x9b => 1,
        0x8d => 1,
        0x8e => 1,
        0x8f => 1,
        0xe0 => 1,
        0xe7 => 1,
        0x8a => 1,
        0x19 => 1,
        0x18 => 1,
        0x17 => 1,
        0x16 => 2,
        0x15 => 1,
        0x14 => 1,
        0x13 => 1,
        0x12 => 1,
        0x11 => 3,
        0x10 => 1,
        0x97 => 1,
        0x96 => 1,
        0x95 => 1,
        0x94 => 1,
        0x93 => 1,
        0x92 => 1,
        0x91 => 1,
        0x90 => 1,
        0x0d => 1,
        0x0e => 2,
        0x0f => 1,
        0x98 => 1,
        0x0a => 1,
        0x0b => 1,
        0x0c => 1,

        _ => panic!("Unknown opcode")
   }
}

impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
    fn return_if_not_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNZ").ok().expect("");
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ADI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn pop_data_off_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "POP").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn add_to_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADD").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn jump_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPE").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CZ").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn double_add(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DAD").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ORI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CC").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn jump(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JMP").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn subtract_from_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SUB").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn rim(&mut self)
    {
        write!(self.stream_out, "{:04}", "RIM").ok().expect("");
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "SBI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn call_if_parity_even(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPE").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn jump_if_positive(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JP").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "XRA").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn move_data(&mut self, register1: Register8080, register2: Register8080)
    {
        write!(self.stream_out, "{:04}", "MOV").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
        write!(self.stream_out, " {:?}", register2).ok().expect("");
    }
    fn no_instruction(&mut self)
    {
        write!(self.stream_out, "{:04}", "NOP").ok().expect("");
    }
    fn halt(&mut self)
    {
        write!(self.stream_out, "{:04}", "HLT").ok().expect("");
    }
    fn set_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "STC").ok().expect("");
    }
    fn compare_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "CMP").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNZ").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn call_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CPO").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "SUI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAL").ok().expect("");
    }
    fn disable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "DI").ok().expect("");
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        write!(self.stream_out, "{:04}", "SPHL").ok().expect("");
    }
    fn logical_and_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ANA").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn load_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LHLD").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "XRI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn call(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CALL").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn enable_interrupts(&mut self)
    {
        write!(self.stream_out, "{:04}", "EI").ok().expect("");
    }
    fn load_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "LDAX").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn input(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "IN").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn jump_if_parity_odd(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JPO").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn increment_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INX").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn return_if_no_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RNC").ok().expect("");
    }
    fn logical_or_with_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ORA").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn exchange_registers(&mut self)
    {
        write!(self.stream_out, "{:04}", "XCHG").ok().expect("");
    }
    fn rotate_accumulator_right(&mut self)
    {
        write!(self.stream_out, "{:04}", "RRC").ok().expect("");
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CNC").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn return_if_parity_even(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPE").ok().expect("");
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ACI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "ANI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn call_if_plus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CP").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn increment_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "INR").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "CPI").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn load_program_counter(&mut self)
    {
        write!(self.stream_out, "{:04}", "PCHL").ok().expect("");
    }
    fn return_if_minus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RM").ok().expect("");
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JC").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn call_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "CM").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "DAA").ok().expect("");
    }
    fn load_register_pair_immediate(&mut self, register1: Register8080, data2: u16)
    {
        write!(self.stream_out, "{:04}", "LXI").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
        write!(self.stream_out, " #${:02x}", data2).ok().expect("");
    }
    fn move_immediate_data(&mut self, register1: Register8080, data2: u8)
    {
        write!(self.stream_out, "{:04}", "MVI").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
        write!(self.stream_out, " #${:02x}", data2).ok().expect("");
    }
    fn return_if_plus(&mut self)
    {
        write!(self.stream_out, "{:04}", "RP").ok().expect("");
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        write!(self.stream_out, "{:04}", "RST").ok().expect("");
        write!(self.stream_out, " {}", implicit_data1).ok().expect("");
    }
    fn return_if_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RC").ok().expect("");
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "STA").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNZ").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn jump_if_minus(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JM").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn decrement_register_or_memory(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCR").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn output(&mut self, data1: u8)
    {
        write!(self.stream_out, "{:04}", "OUT").ok().expect("");
        write!(self.stream_out, " #${:02x}", data1).ok().expect("");
    }
    fn store_accumulator(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "STAX").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "ADC").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JZ").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn complement_accumulator(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMA").ok().expect("");
    }
    fn return_if_zero(&mut self)
    {
        write!(self.stream_out, "{:04}", "RZ").ok().expect("");
    }
    fn return_if_parity_odd(&mut self)
    {
        write!(self.stream_out, "{:04}", "RPO").ok().expect("");
    }
    fn return_unconditionally(&mut self)
    {
        write!(self.stream_out, "{:04}", "RET").ok().expect("");
    }
    fn store_h_and_l_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "SHLD").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "SBB").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn not_implemented(&mut self)
    {
        write!(self.stream_out, "{:04}", "-").ok().expect("");
    }
    fn push_data_onto_stack(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "PUSH").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "JNC").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn sim(&mut self)
    {
        write!(self.stream_out, "{:04}", "SIM").ok().expect("");
    }
    fn decrement_register_pair(&mut self, register1: Register8080)
    {
        write!(self.stream_out, "{:04}", "DCX").ok().expect("");
        write!(self.stream_out, " {:?}", register1).ok().expect("");
    }
    fn complement_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "CMC").ok().expect("");
    }
    fn rotate_accumulator_left(&mut self)
    {
        write!(self.stream_out, "{:04}", "RLC").ok().expect("");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        write!(self.stream_out, "{:04}", "LDA").ok().expect("");
        write!(self.stream_out, " ${:02x}", address1).ok().expect("");
    }
    fn exchange_stack(&mut self)
    {
        write!(self.stream_out, "{:04}", "XTHL").ok().expect("");
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        write!(self.stream_out, "{:04}", "RAR").ok().expect("");
    }
}