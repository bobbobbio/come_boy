use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::opcodes::LR35902InstructionPrinter;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
pub trait LR35902InstructionSet {
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn add_immediate_to_sp(&mut self, data1: u8);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn call_if_carry(&mut self, address1: u16);
    fn call_if_no_carry(&mut self, address1: u16);
    fn call_if_not_zero(&mut self, address1: u16);
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
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn halt(&mut self);
    fn halt_until_button_press(&mut self);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn jump(&mut self, address1: u16);
    fn jump_if_carry(&mut self, address1: u16);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_zero(&mut self, address1: u16);
    fn jump_relative(&mut self, data1: u8);
    fn jump_relative_if_carry(&mut self, data1: u8);
    fn jump_relative_if_no_carry(&mut self, data1: u8);
    fn jump_relative_if_not_zero(&mut self, data1: u8);
    fn jump_relative_if_zero(&mut self, data1: u8);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn load_accumulator_direct_one_byte(&mut self, data1: u8);
    fn load_accumulator_one_byte(&mut self);
    fn load_program_counter(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn move_and_decrement_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_and_increment_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn no_operation(&mut self);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn reset_bit(&mut self, data1: u8, register2: Intel8080Register);
    fn restart(&mut self, data1: u8);
    fn return_and_enable_interrupts(&mut self);
    fn return_if_carry(&mut self);
    fn return_if_no_carry(&mut self);
    fn return_if_not_zero(&mut self);
    fn return_if_zero(&mut self);
    fn return_unconditionally(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn rotate_register_left(&mut self, register1: Intel8080Register);
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register);
    fn rotate_register_right(&mut self, register1: Intel8080Register);
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register);
    fn set_bit(&mut self, data1: u8, register2: Intel8080Register);
    fn set_carry(&mut self);
    fn shift_register_left(&mut self, register1: Intel8080Register);
    fn shift_register_right(&mut self, register1: Intel8080Register);
    fn shift_register_right_signed(&mut self, register1: Intel8080Register);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn store_accumulator_direct_one_byte(&mut self, data1: u8);
    fn store_accumulator_one_byte(&mut self);
    fn store_sp_direct(&mut self, address1: u16);
    fn store_sp_plus_immediate(&mut self, data1: u8);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
    fn swap_register(&mut self, register1: Intel8080Register);
    fn test_bit(&mut self, data1: u8, register2: Intel8080Register);
}
pub fn dispatch_lr35902_instruction<I: LR35902InstructionSet>(
    mut stream: &[u8],
    machine: &mut I,
) -> u8 {
    let opcode = stream.read_u8().unwrap();
    match opcode {
        0x00 => {
            machine.no_operation();
            4u8
        }
        0x01 => {
            machine.load_register_pair_immediate(
                Intel8080Register::B,
                stream.read_u16::<LittleEndian>().unwrap(),
            );
            12u8
        }
        0x02 => {
            machine.store_accumulator(Intel8080Register::B);
            8u8
        }
        0x03 => {
            machine.increment_register_pair(Intel8080Register::B);
            8u8
        }
        0x04 => {
            machine.increment_register_or_memory(Intel8080Register::B);
            4u8
        }
        0x05 => {
            machine.decrement_register_or_memory(Intel8080Register::B);
            4u8
        }
        0x06 => {
            machine.move_immediate_data(Intel8080Register::B, stream.read_u8().unwrap());
            8u8
        }
        0x07 => {
            machine.rotate_accumulator_left();
            4u8
        }
        0x08 => {
            machine.store_sp_direct(stream.read_u16::<LittleEndian>().unwrap());
            20u8
        }
        0x09 => {
            machine.double_add(Intel8080Register::B);
            8u8
        }
        0x0A => {
            machine.load_accumulator(Intel8080Register::B);
            8u8
        }
        0x0B => {
            machine.decrement_register_pair(Intel8080Register::B);
            8u8
        }
        0x0C => {
            machine.increment_register_or_memory(Intel8080Register::C);
            4u8
        }
        0x0D => {
            machine.decrement_register_or_memory(Intel8080Register::C);
            4u8
        }
        0x0E => {
            machine.move_immediate_data(Intel8080Register::C, stream.read_u8().unwrap());
            8u8
        }
        0x0F => {
            machine.rotate_accumulator_right();
            4u8
        }
        0x10 => match (0x10 as u16) << 8 | stream.read_u8().unwrap() as u16 {
            0x1000 => {
                machine.halt_until_button_press();
                4u8
            }
            v => panic!("Unknown opcode {}", v),
        },
        0x11 => {
            machine.load_register_pair_immediate(
                Intel8080Register::D,
                stream.read_u16::<LittleEndian>().unwrap(),
            );
            12u8
        }
        0x12 => {
            machine.store_accumulator(Intel8080Register::D);
            8u8
        }
        0x13 => {
            machine.increment_register_pair(Intel8080Register::D);
            8u8
        }
        0x14 => {
            machine.increment_register_or_memory(Intel8080Register::D);
            4u8
        }
        0x15 => {
            machine.decrement_register_or_memory(Intel8080Register::D);
            4u8
        }
        0x16 => {
            machine.move_immediate_data(Intel8080Register::D, stream.read_u8().unwrap());
            8u8
        }
        0x17 => {
            machine.rotate_accumulator_left_through_carry();
            4u8
        }
        0x18 => {
            machine.jump_relative(stream.read_u8().unwrap());
            12u8
        }
        0x19 => {
            machine.double_add(Intel8080Register::D);
            8u8
        }
        0x1A => {
            machine.load_accumulator(Intel8080Register::D);
            8u8
        }
        0x1B => {
            machine.decrement_register_pair(Intel8080Register::D);
            8u8
        }
        0x1C => {
            machine.increment_register_or_memory(Intel8080Register::E);
            4u8
        }
        0x1D => {
            machine.decrement_register_or_memory(Intel8080Register::E);
            4u8
        }
        0x1E => {
            machine.move_immediate_data(Intel8080Register::E, stream.read_u8().unwrap());
            8u8
        }
        0x1F => {
            machine.rotate_accumulator_right_through_carry();
            4u8
        }
        0x20 => {
            machine.jump_relative_if_not_zero(stream.read_u8().unwrap());
            8u8
        }
        0x21 => {
            machine.load_register_pair_immediate(
                Intel8080Register::H,
                stream.read_u16::<LittleEndian>().unwrap(),
            );
            12u8
        }
        0x22 => {
            machine.move_and_increment_hl(Intel8080Register::M, Intel8080Register::A);
            8u8
        }
        0x23 => {
            machine.increment_register_pair(Intel8080Register::H);
            8u8
        }
        0x24 => {
            machine.increment_register_or_memory(Intel8080Register::H);
            4u8
        }
        0x25 => {
            machine.decrement_register_or_memory(Intel8080Register::H);
            4u8
        }
        0x26 => {
            machine.move_immediate_data(Intel8080Register::H, stream.read_u8().unwrap());
            8u8
        }
        0x27 => {
            machine.decimal_adjust_accumulator();
            4u8
        }
        0x28 => {
            machine.jump_relative_if_zero(stream.read_u8().unwrap());
            8u8
        }
        0x29 => {
            machine.double_add(Intel8080Register::H);
            8u8
        }
        0x2A => {
            machine.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M);
            8u8
        }
        0x2B => {
            machine.decrement_register_pair(Intel8080Register::H);
            8u8
        }
        0x2C => {
            machine.increment_register_or_memory(Intel8080Register::L);
            4u8
        }
        0x2D => {
            machine.decrement_register_or_memory(Intel8080Register::L);
            4u8
        }
        0x2E => {
            machine.move_immediate_data(Intel8080Register::L, stream.read_u8().unwrap());
            8u8
        }
        0x2F => {
            machine.complement_accumulator();
            4u8
        }
        0x30 => {
            machine.jump_relative_if_no_carry(stream.read_u8().unwrap());
            8u8
        }
        0x31 => {
            machine.load_register_pair_immediate(
                Intel8080Register::SP,
                stream.read_u16::<LittleEndian>().unwrap(),
            );
            12u8
        }
        0x32 => {
            machine.move_and_decrement_hl(Intel8080Register::M, Intel8080Register::A);
            8u8
        }
        0x33 => {
            machine.increment_register_pair(Intel8080Register::SP);
            8u8
        }
        0x34 => {
            machine.increment_register_or_memory(Intel8080Register::M);
            12u8
        }
        0x35 => {
            machine.decrement_register_or_memory(Intel8080Register::M);
            12u8
        }
        0x36 => {
            machine.move_immediate_data(Intel8080Register::M, stream.read_u8().unwrap());
            12u8
        }
        0x37 => {
            machine.set_carry();
            4u8
        }
        0x38 => {
            machine.jump_relative_if_carry(stream.read_u8().unwrap());
            8u8
        }
        0x39 => {
            machine.double_add(Intel8080Register::SP);
            8u8
        }
        0x3A => {
            machine.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M);
            8u8
        }
        0x3B => {
            machine.decrement_register_pair(Intel8080Register::SP);
            8u8
        }
        0x3C => {
            machine.increment_register_or_memory(Intel8080Register::A);
            4u8
        }
        0x3D => {
            machine.decrement_register_or_memory(Intel8080Register::A);
            4u8
        }
        0x3E => {
            machine.move_immediate_data(Intel8080Register::A, stream.read_u8().unwrap());
            8u8
        }
        0x3F => {
            machine.complement_carry();
            4u8
        }
        0x40 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::B);
            4u8
        }
        0x41 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::C);
            4u8
        }
        0x42 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::D);
            4u8
        }
        0x43 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::E);
            4u8
        }
        0x44 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::H);
            4u8
        }
        0x45 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::L);
            4u8
        }
        0x46 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::M);
            8u8
        }
        0x47 => {
            machine.move_data(Intel8080Register::B, Intel8080Register::A);
            4u8
        }
        0x48 => {
            machine.move_data(Intel8080Register::C, Intel8080Register::B);
            4u8
        }
        0x49 => {
            machine.move_data(Intel8080Register::C, Intel8080Register::C);
            4u8
        }
        0x4A => {
            machine.move_data(Intel8080Register::C, Intel8080Register::D);
            4u8
        }
        0x4B => {
            machine.move_data(Intel8080Register::C, Intel8080Register::E);
            4u8
        }
        0x4C => {
            machine.move_data(Intel8080Register::C, Intel8080Register::H);
            4u8
        }
        0x4D => {
            machine.move_data(Intel8080Register::C, Intel8080Register::L);
            4u8
        }
        0x4E => {
            machine.move_data(Intel8080Register::C, Intel8080Register::M);
            8u8
        }
        0x4F => {
            machine.move_data(Intel8080Register::C, Intel8080Register::A);
            4u8
        }
        0x50 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::B);
            4u8
        }
        0x51 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::C);
            4u8
        }
        0x52 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::D);
            4u8
        }
        0x53 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::E);
            4u8
        }
        0x54 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::H);
            4u8
        }
        0x55 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::L);
            4u8
        }
        0x56 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::M);
            8u8
        }
        0x57 => {
            machine.move_data(Intel8080Register::D, Intel8080Register::A);
            4u8
        }
        0x58 => {
            machine.move_data(Intel8080Register::E, Intel8080Register::B);
            4u8
        }
        0x59 => {
            machine.move_data(Intel8080Register::E, Intel8080Register::C);
            4u8
        }
        0x5A => {
            machine.move_data(Intel8080Register::E, Intel8080Register::D);
            4u8
        }
        0x5B => {
            machine.move_data(Intel8080Register::E, Intel8080Register::E);
            4u8
        }
        0x5C => {
            machine.move_data(Intel8080Register::E, Intel8080Register::H);
            4u8
        }
        0x5D => {
            machine.move_data(Intel8080Register::E, Intel8080Register::L);
            4u8
        }
        0x5E => {
            machine.move_data(Intel8080Register::E, Intel8080Register::M);
            8u8
        }
        0x5F => {
            machine.move_data(Intel8080Register::E, Intel8080Register::A);
            4u8
        }
        0x60 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::B);
            4u8
        }
        0x61 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::C);
            4u8
        }
        0x62 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::D);
            4u8
        }
        0x63 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::E);
            4u8
        }
        0x64 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::H);
            4u8
        }
        0x65 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::L);
            4u8
        }
        0x66 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::M);
            8u8
        }
        0x67 => {
            machine.move_data(Intel8080Register::H, Intel8080Register::A);
            4u8
        }
        0x68 => {
            machine.move_data(Intel8080Register::L, Intel8080Register::B);
            4u8
        }
        0x69 => {
            machine.move_data(Intel8080Register::L, Intel8080Register::C);
            4u8
        }
        0x6A => {
            machine.move_data(Intel8080Register::L, Intel8080Register::D);
            4u8
        }
        0x6B => {
            machine.move_data(Intel8080Register::L, Intel8080Register::E);
            4u8
        }
        0x6C => {
            machine.move_data(Intel8080Register::L, Intel8080Register::H);
            4u8
        }
        0x6D => {
            machine.move_data(Intel8080Register::L, Intel8080Register::L);
            4u8
        }
        0x6E => {
            machine.move_data(Intel8080Register::L, Intel8080Register::M);
            8u8
        }
        0x6F => {
            machine.move_data(Intel8080Register::L, Intel8080Register::A);
            4u8
        }
        0x70 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::B);
            8u8
        }
        0x71 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::C);
            8u8
        }
        0x72 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::D);
            8u8
        }
        0x73 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::E);
            8u8
        }
        0x74 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::H);
            8u8
        }
        0x75 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::L);
            8u8
        }
        0x76 => {
            machine.halt();
            4u8
        }
        0x77 => {
            machine.move_data(Intel8080Register::M, Intel8080Register::A);
            8u8
        }
        0x78 => {
            machine.move_data(Intel8080Register::A, Intel8080Register::B);
            4u8
        }
        0x79 => {
            machine.move_data(Intel8080Register::A, Intel8080Register::C);
            4u8
        }
        0x7A => {
            machine.move_data(Intel8080Register::A, Intel8080Register::D);
            4u8
        }
        0x7B => {
            machine.move_data(Intel8080Register::A, Intel8080Register::E);
            4u8
        }
        0x7C => {
            machine.move_data(Intel8080Register::A, Intel8080Register::H);
            4u8
        }
        0x7D => {
            machine.move_data(Intel8080Register::A, Intel8080Register::L);
            4u8
        }
        0x7E => {
            machine.move_data(Intel8080Register::A, Intel8080Register::M);
            8u8
        }
        0x7F => {
            machine.move_data(Intel8080Register::A, Intel8080Register::A);
            4u8
        }
        0x80 => {
            machine.add_to_accumulator(Intel8080Register::B);
            4u8
        }
        0x81 => {
            machine.add_to_accumulator(Intel8080Register::C);
            4u8
        }
        0x82 => {
            machine.add_to_accumulator(Intel8080Register::D);
            4u8
        }
        0x83 => {
            machine.add_to_accumulator(Intel8080Register::E);
            4u8
        }
        0x84 => {
            machine.add_to_accumulator(Intel8080Register::H);
            4u8
        }
        0x85 => {
            machine.add_to_accumulator(Intel8080Register::L);
            4u8
        }
        0x86 => {
            machine.add_to_accumulator(Intel8080Register::M);
            8u8
        }
        0x87 => {
            machine.add_to_accumulator(Intel8080Register::A);
            4u8
        }
        0x88 => {
            machine.add_to_accumulator_with_carry(Intel8080Register::B);
            4u8
        }
        0x89 => {
            machine.add_to_accumulator_with_carry(Intel8080Register::C);
            4u8
        }
        0x8A => {
            machine.add_to_accumulator_with_carry(Intel8080Register::D);
            4u8
        }
        0x8B => {
            machine.add_to_accumulator_with_carry(Intel8080Register::E);
            4u8
        }
        0x8C => {
            machine.add_to_accumulator_with_carry(Intel8080Register::H);
            4u8
        }
        0x8D => {
            machine.add_to_accumulator_with_carry(Intel8080Register::L);
            4u8
        }
        0x8E => {
            machine.add_to_accumulator_with_carry(Intel8080Register::M);
            8u8
        }
        0x8F => {
            machine.add_to_accumulator_with_carry(Intel8080Register::A);
            4u8
        }
        0x90 => {
            machine.subtract_from_accumulator(Intel8080Register::B);
            4u8
        }
        0x91 => {
            machine.subtract_from_accumulator(Intel8080Register::C);
            4u8
        }
        0x92 => {
            machine.subtract_from_accumulator(Intel8080Register::D);
            4u8
        }
        0x93 => {
            machine.subtract_from_accumulator(Intel8080Register::E);
            4u8
        }
        0x94 => {
            machine.subtract_from_accumulator(Intel8080Register::H);
            4u8
        }
        0x95 => {
            machine.subtract_from_accumulator(Intel8080Register::L);
            4u8
        }
        0x96 => {
            machine.subtract_from_accumulator(Intel8080Register::M);
            8u8
        }
        0x97 => {
            machine.subtract_from_accumulator(Intel8080Register::A);
            4u8
        }
        0x98 => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::B);
            4u8
        }
        0x99 => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::C);
            4u8
        }
        0x9A => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::D);
            4u8
        }
        0x9B => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::E);
            4u8
        }
        0x9C => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::H);
            4u8
        }
        0x9D => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::L);
            4u8
        }
        0x9E => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::M);
            8u8
        }
        0x9F => {
            machine.subtract_from_accumulator_with_borrow(Intel8080Register::A);
            4u8
        }
        0xA0 => {
            machine.logical_and_with_accumulator(Intel8080Register::B);
            4u8
        }
        0xA1 => {
            machine.logical_and_with_accumulator(Intel8080Register::C);
            4u8
        }
        0xA2 => {
            machine.logical_and_with_accumulator(Intel8080Register::D);
            4u8
        }
        0xA3 => {
            machine.logical_and_with_accumulator(Intel8080Register::E);
            4u8
        }
        0xA4 => {
            machine.logical_and_with_accumulator(Intel8080Register::H);
            4u8
        }
        0xA5 => {
            machine.logical_and_with_accumulator(Intel8080Register::L);
            4u8
        }
        0xA6 => {
            machine.logical_and_with_accumulator(Intel8080Register::M);
            8u8
        }
        0xA7 => {
            machine.logical_and_with_accumulator(Intel8080Register::A);
            4u8
        }
        0xA8 => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::B);
            4u8
        }
        0xA9 => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::C);
            4u8
        }
        0xAA => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::D);
            4u8
        }
        0xAB => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::E);
            4u8
        }
        0xAC => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::H);
            4u8
        }
        0xAD => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::L);
            4u8
        }
        0xAE => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::M);
            8u8
        }
        0xAF => {
            machine.logical_exclusive_or_with_accumulator(Intel8080Register::A);
            4u8
        }
        0xB0 => {
            machine.logical_or_with_accumulator(Intel8080Register::B);
            4u8
        }
        0xB1 => {
            machine.logical_or_with_accumulator(Intel8080Register::C);
            4u8
        }
        0xB2 => {
            machine.logical_or_with_accumulator(Intel8080Register::D);
            4u8
        }
        0xB3 => {
            machine.logical_or_with_accumulator(Intel8080Register::E);
            4u8
        }
        0xB4 => {
            machine.logical_or_with_accumulator(Intel8080Register::H);
            4u8
        }
        0xB5 => {
            machine.logical_or_with_accumulator(Intel8080Register::L);
            4u8
        }
        0xB6 => {
            machine.logical_or_with_accumulator(Intel8080Register::M);
            8u8
        }
        0xB7 => {
            machine.logical_or_with_accumulator(Intel8080Register::A);
            4u8
        }
        0xB8 => {
            machine.compare_with_accumulator(Intel8080Register::B);
            4u8
        }
        0xB9 => {
            machine.compare_with_accumulator(Intel8080Register::C);
            4u8
        }
        0xBA => {
            machine.compare_with_accumulator(Intel8080Register::D);
            4u8
        }
        0xBB => {
            machine.compare_with_accumulator(Intel8080Register::E);
            4u8
        }
        0xBC => {
            machine.compare_with_accumulator(Intel8080Register::H);
            4u8
        }
        0xBD => {
            machine.compare_with_accumulator(Intel8080Register::L);
            4u8
        }
        0xBE => {
            machine.compare_with_accumulator(Intel8080Register::M);
            8u8
        }
        0xBF => {
            machine.compare_with_accumulator(Intel8080Register::A);
            4u8
        }
        0xC0 => {
            machine.return_if_not_zero();
            8u8
        }
        0xC1 => {
            machine.pop_data_off_stack(Intel8080Register::B);
            12u8
        }
        0xC2 => {
            machine.jump_if_not_zero(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xC3 => {
            machine.jump(stream.read_u16::<LittleEndian>().unwrap());
            16u8
        }
        0xC4 => {
            machine.call_if_not_zero(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xC5 => {
            machine.push_data_onto_stack(Intel8080Register::B);
            16u8
        }
        0xC6 => {
            machine.add_immediate_to_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xC7 => {
            machine.restart(0u8);
            16u8
        }
        0xC8 => {
            machine.return_if_zero();
            8u8
        }
        0xC9 => {
            machine.return_unconditionally();
            16u8
        }
        0xCA => {
            machine.jump_if_zero(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xCB => match (0xCB as u16) << 8 | stream.read_u8().unwrap() as u16 {
            0xCB00 => {
                machine.rotate_register_left(Intel8080Register::B);
                8u8
            }
            0xCB01 => {
                machine.rotate_register_left(Intel8080Register::C);
                8u8
            }
            0xCB02 => {
                machine.rotate_register_left(Intel8080Register::D);
                8u8
            }
            0xCB03 => {
                machine.rotate_register_left(Intel8080Register::E);
                8u8
            }
            0xCB04 => {
                machine.rotate_register_left(Intel8080Register::H);
                8u8
            }
            0xCB05 => {
                machine.rotate_register_left(Intel8080Register::L);
                8u8
            }
            0xCB06 => {
                machine.rotate_register_left(Intel8080Register::M);
                16u8
            }
            0xCB07 => {
                machine.rotate_register_left(Intel8080Register::A);
                8u8
            }
            0xCB08 => {
                machine.rotate_register_right(Intel8080Register::B);
                8u8
            }
            0xCB09 => {
                machine.rotate_register_right(Intel8080Register::C);
                8u8
            }
            0xCB0A => {
                machine.rotate_register_right(Intel8080Register::D);
                8u8
            }
            0xCB0B => {
                machine.rotate_register_right(Intel8080Register::E);
                8u8
            }
            0xCB0C => {
                machine.rotate_register_right(Intel8080Register::H);
                8u8
            }
            0xCB0D => {
                machine.rotate_register_right(Intel8080Register::L);
                8u8
            }
            0xCB0E => {
                machine.rotate_register_right(Intel8080Register::M);
                16u8
            }
            0xCB0F => {
                machine.rotate_register_right(Intel8080Register::A);
                8u8
            }
            0xCB10 => {
                machine.rotate_register_left_through_carry(Intel8080Register::B);
                8u8
            }
            0xCB11 => {
                machine.rotate_register_left_through_carry(Intel8080Register::C);
                8u8
            }
            0xCB12 => {
                machine.rotate_register_left_through_carry(Intel8080Register::D);
                8u8
            }
            0xCB13 => {
                machine.rotate_register_left_through_carry(Intel8080Register::E);
                8u8
            }
            0xCB14 => {
                machine.rotate_register_left_through_carry(Intel8080Register::H);
                8u8
            }
            0xCB15 => {
                machine.rotate_register_left_through_carry(Intel8080Register::L);
                8u8
            }
            0xCB16 => {
                machine.rotate_register_left_through_carry(Intel8080Register::M);
                16u8
            }
            0xCB17 => {
                machine.rotate_register_left_through_carry(Intel8080Register::A);
                8u8
            }
            0xCB18 => {
                machine.rotate_register_right_through_carry(Intel8080Register::B);
                8u8
            }
            0xCB19 => {
                machine.rotate_register_right_through_carry(Intel8080Register::C);
                8u8
            }
            0xCB1A => {
                machine.rotate_register_right_through_carry(Intel8080Register::D);
                8u8
            }
            0xCB1B => {
                machine.rotate_register_right_through_carry(Intel8080Register::E);
                8u8
            }
            0xCB1C => {
                machine.rotate_register_right_through_carry(Intel8080Register::H);
                8u8
            }
            0xCB1D => {
                machine.rotate_register_right_through_carry(Intel8080Register::L);
                8u8
            }
            0xCB1E => {
                machine.rotate_register_right_through_carry(Intel8080Register::M);
                16u8
            }
            0xCB1F => {
                machine.rotate_register_right_through_carry(Intel8080Register::A);
                8u8
            }
            0xCB20 => {
                machine.shift_register_left(Intel8080Register::B);
                8u8
            }
            0xCB21 => {
                machine.shift_register_left(Intel8080Register::C);
                8u8
            }
            0xCB22 => {
                machine.shift_register_left(Intel8080Register::D);
                8u8
            }
            0xCB23 => {
                machine.shift_register_left(Intel8080Register::E);
                8u8
            }
            0xCB24 => {
                machine.shift_register_left(Intel8080Register::H);
                8u8
            }
            0xCB25 => {
                machine.shift_register_left(Intel8080Register::L);
                8u8
            }
            0xCB26 => {
                machine.shift_register_left(Intel8080Register::M);
                16u8
            }
            0xCB27 => {
                machine.shift_register_left(Intel8080Register::A);
                8u8
            }
            0xCB28 => {
                machine.shift_register_right_signed(Intel8080Register::B);
                8u8
            }
            0xCB29 => {
                machine.shift_register_right_signed(Intel8080Register::C);
                8u8
            }
            0xCB2A => {
                machine.shift_register_right_signed(Intel8080Register::D);
                8u8
            }
            0xCB2B => {
                machine.shift_register_right_signed(Intel8080Register::E);
                8u8
            }
            0xCB2C => {
                machine.shift_register_right_signed(Intel8080Register::H);
                8u8
            }
            0xCB2D => {
                machine.shift_register_right_signed(Intel8080Register::L);
                8u8
            }
            0xCB2E => {
                machine.shift_register_right_signed(Intel8080Register::M);
                16u8
            }
            0xCB2F => {
                machine.shift_register_right_signed(Intel8080Register::A);
                8u8
            }
            0xCB30 => {
                machine.swap_register(Intel8080Register::B);
                8u8
            }
            0xCB31 => {
                machine.swap_register(Intel8080Register::C);
                8u8
            }
            0xCB32 => {
                machine.swap_register(Intel8080Register::D);
                8u8
            }
            0xCB33 => {
                machine.swap_register(Intel8080Register::E);
                8u8
            }
            0xCB34 => {
                machine.swap_register(Intel8080Register::H);
                8u8
            }
            0xCB35 => {
                machine.swap_register(Intel8080Register::L);
                8u8
            }
            0xCB36 => {
                machine.swap_register(Intel8080Register::M);
                16u8
            }
            0xCB37 => {
                machine.swap_register(Intel8080Register::A);
                8u8
            }
            0xCB38 => {
                machine.shift_register_right(Intel8080Register::B);
                8u8
            }
            0xCB39 => {
                machine.shift_register_right(Intel8080Register::C);
                8u8
            }
            0xCB3A => {
                machine.shift_register_right(Intel8080Register::D);
                8u8
            }
            0xCB3B => {
                machine.shift_register_right(Intel8080Register::E);
                8u8
            }
            0xCB3C => {
                machine.shift_register_right(Intel8080Register::H);
                8u8
            }
            0xCB3D => {
                machine.shift_register_right(Intel8080Register::L);
                8u8
            }
            0xCB3E => {
                machine.shift_register_right(Intel8080Register::M);
                16u8
            }
            0xCB3F => {
                machine.shift_register_right(Intel8080Register::A);
                8u8
            }
            0xCB40 => {
                machine.test_bit(0u8, Intel8080Register::B);
                8u8
            }
            0xCB41 => {
                machine.test_bit(0u8, Intel8080Register::C);
                8u8
            }
            0xCB42 => {
                machine.test_bit(0u8, Intel8080Register::D);
                8u8
            }
            0xCB43 => {
                machine.test_bit(0u8, Intel8080Register::E);
                8u8
            }
            0xCB44 => {
                machine.test_bit(0u8, Intel8080Register::H);
                8u8
            }
            0xCB45 => {
                machine.test_bit(0u8, Intel8080Register::L);
                8u8
            }
            0xCB46 => {
                machine.test_bit(0u8, Intel8080Register::M);
                16u8
            }
            0xCB47 => {
                machine.test_bit(0u8, Intel8080Register::A);
                8u8
            }
            0xCB48 => {
                machine.test_bit(1u8, Intel8080Register::B);
                8u8
            }
            0xCB49 => {
                machine.test_bit(1u8, Intel8080Register::C);
                8u8
            }
            0xCB4A => {
                machine.test_bit(1u8, Intel8080Register::D);
                8u8
            }
            0xCB4B => {
                machine.test_bit(1u8, Intel8080Register::E);
                8u8
            }
            0xCB4C => {
                machine.test_bit(1u8, Intel8080Register::H);
                8u8
            }
            0xCB4D => {
                machine.test_bit(1u8, Intel8080Register::L);
                8u8
            }
            0xCB4E => {
                machine.test_bit(1u8, Intel8080Register::M);
                16u8
            }
            0xCB4F => {
                machine.test_bit(1u8, Intel8080Register::A);
                8u8
            }
            0xCB50 => {
                machine.test_bit(2u8, Intel8080Register::B);
                8u8
            }
            0xCB51 => {
                machine.test_bit(2u8, Intel8080Register::C);
                8u8
            }
            0xCB52 => {
                machine.test_bit(2u8, Intel8080Register::D);
                8u8
            }
            0xCB53 => {
                machine.test_bit(2u8, Intel8080Register::E);
                8u8
            }
            0xCB54 => {
                machine.test_bit(2u8, Intel8080Register::H);
                8u8
            }
            0xCB55 => {
                machine.test_bit(2u8, Intel8080Register::L);
                8u8
            }
            0xCB56 => {
                machine.test_bit(2u8, Intel8080Register::M);
                16u8
            }
            0xCB57 => {
                machine.test_bit(2u8, Intel8080Register::A);
                8u8
            }
            0xCB58 => {
                machine.test_bit(3u8, Intel8080Register::B);
                8u8
            }
            0xCB59 => {
                machine.test_bit(3u8, Intel8080Register::C);
                8u8
            }
            0xCB5A => {
                machine.test_bit(3u8, Intel8080Register::D);
                8u8
            }
            0xCB5B => {
                machine.test_bit(3u8, Intel8080Register::E);
                8u8
            }
            0xCB5C => {
                machine.test_bit(3u8, Intel8080Register::H);
                8u8
            }
            0xCB5D => {
                machine.test_bit(3u8, Intel8080Register::L);
                8u8
            }
            0xCB5E => {
                machine.test_bit(3u8, Intel8080Register::M);
                16u8
            }
            0xCB5F => {
                machine.test_bit(3u8, Intel8080Register::A);
                8u8
            }
            0xCB60 => {
                machine.test_bit(4u8, Intel8080Register::B);
                8u8
            }
            0xCB61 => {
                machine.test_bit(4u8, Intel8080Register::C);
                8u8
            }
            0xCB62 => {
                machine.test_bit(4u8, Intel8080Register::D);
                8u8
            }
            0xCB63 => {
                machine.test_bit(4u8, Intel8080Register::E);
                8u8
            }
            0xCB64 => {
                machine.test_bit(4u8, Intel8080Register::H);
                8u8
            }
            0xCB65 => {
                machine.test_bit(4u8, Intel8080Register::L);
                8u8
            }
            0xCB66 => {
                machine.test_bit(4u8, Intel8080Register::M);
                16u8
            }
            0xCB67 => {
                machine.test_bit(4u8, Intel8080Register::A);
                8u8
            }
            0xCB68 => {
                machine.test_bit(5u8, Intel8080Register::B);
                8u8
            }
            0xCB69 => {
                machine.test_bit(5u8, Intel8080Register::C);
                8u8
            }
            0xCB6A => {
                machine.test_bit(5u8, Intel8080Register::D);
                8u8
            }
            0xCB6B => {
                machine.test_bit(5u8, Intel8080Register::E);
                8u8
            }
            0xCB6C => {
                machine.test_bit(5u8, Intel8080Register::H);
                8u8
            }
            0xCB6D => {
                machine.test_bit(5u8, Intel8080Register::L);
                8u8
            }
            0xCB6E => {
                machine.test_bit(5u8, Intel8080Register::M);
                16u8
            }
            0xCB6F => {
                machine.test_bit(5u8, Intel8080Register::A);
                8u8
            }
            0xCB70 => {
                machine.test_bit(6u8, Intel8080Register::B);
                8u8
            }
            0xCB71 => {
                machine.test_bit(6u8, Intel8080Register::C);
                8u8
            }
            0xCB72 => {
                machine.test_bit(6u8, Intel8080Register::D);
                8u8
            }
            0xCB73 => {
                machine.test_bit(6u8, Intel8080Register::E);
                8u8
            }
            0xCB74 => {
                machine.test_bit(6u8, Intel8080Register::H);
                8u8
            }
            0xCB75 => {
                machine.test_bit(6u8, Intel8080Register::L);
                8u8
            }
            0xCB76 => {
                machine.test_bit(6u8, Intel8080Register::M);
                16u8
            }
            0xCB77 => {
                machine.test_bit(6u8, Intel8080Register::A);
                8u8
            }
            0xCB78 => {
                machine.test_bit(7u8, Intel8080Register::B);
                8u8
            }
            0xCB79 => {
                machine.test_bit(7u8, Intel8080Register::C);
                8u8
            }
            0xCB7A => {
                machine.test_bit(7u8, Intel8080Register::D);
                8u8
            }
            0xCB7B => {
                machine.test_bit(7u8, Intel8080Register::E);
                8u8
            }
            0xCB7C => {
                machine.test_bit(7u8, Intel8080Register::H);
                8u8
            }
            0xCB7D => {
                machine.test_bit(7u8, Intel8080Register::L);
                8u8
            }
            0xCB7E => {
                machine.test_bit(7u8, Intel8080Register::M);
                16u8
            }
            0xCB7F => {
                machine.test_bit(7u8, Intel8080Register::A);
                8u8
            }
            0xCB80 => {
                machine.reset_bit(0u8, Intel8080Register::B);
                8u8
            }
            0xCB81 => {
                machine.reset_bit(0u8, Intel8080Register::C);
                8u8
            }
            0xCB82 => {
                machine.reset_bit(0u8, Intel8080Register::D);
                8u8
            }
            0xCB83 => {
                machine.reset_bit(0u8, Intel8080Register::E);
                8u8
            }
            0xCB84 => {
                machine.reset_bit(0u8, Intel8080Register::H);
                8u8
            }
            0xCB85 => {
                machine.reset_bit(0u8, Intel8080Register::L);
                8u8
            }
            0xCB86 => {
                machine.reset_bit(0u8, Intel8080Register::M);
                16u8
            }
            0xCB87 => {
                machine.reset_bit(0u8, Intel8080Register::A);
                8u8
            }
            0xCB88 => {
                machine.reset_bit(1u8, Intel8080Register::B);
                8u8
            }
            0xCB89 => {
                machine.reset_bit(1u8, Intel8080Register::C);
                8u8
            }
            0xCB8A => {
                machine.reset_bit(1u8, Intel8080Register::D);
                8u8
            }
            0xCB8B => {
                machine.reset_bit(1u8, Intel8080Register::E);
                8u8
            }
            0xCB8C => {
                machine.reset_bit(1u8, Intel8080Register::H);
                8u8
            }
            0xCB8D => {
                machine.reset_bit(1u8, Intel8080Register::L);
                8u8
            }
            0xCB8E => {
                machine.reset_bit(1u8, Intel8080Register::M);
                16u8
            }
            0xCB8F => {
                machine.reset_bit(1u8, Intel8080Register::A);
                8u8
            }
            0xCB90 => {
                machine.reset_bit(2u8, Intel8080Register::B);
                8u8
            }
            0xCB91 => {
                machine.reset_bit(2u8, Intel8080Register::C);
                8u8
            }
            0xCB92 => {
                machine.reset_bit(2u8, Intel8080Register::D);
                8u8
            }
            0xCB93 => {
                machine.reset_bit(2u8, Intel8080Register::E);
                8u8
            }
            0xCB94 => {
                machine.reset_bit(2u8, Intel8080Register::H);
                8u8
            }
            0xCB95 => {
                machine.reset_bit(2u8, Intel8080Register::L);
                8u8
            }
            0xCB96 => {
                machine.reset_bit(2u8, Intel8080Register::M);
                16u8
            }
            0xCB97 => {
                machine.reset_bit(2u8, Intel8080Register::A);
                8u8
            }
            0xCB98 => {
                machine.reset_bit(3u8, Intel8080Register::B);
                8u8
            }
            0xCB99 => {
                machine.reset_bit(3u8, Intel8080Register::C);
                8u8
            }
            0xCB9A => {
                machine.reset_bit(3u8, Intel8080Register::D);
                8u8
            }
            0xCB9B => {
                machine.reset_bit(3u8, Intel8080Register::E);
                8u8
            }
            0xCB9C => {
                machine.reset_bit(3u8, Intel8080Register::H);
                8u8
            }
            0xCB9D => {
                machine.reset_bit(3u8, Intel8080Register::L);
                8u8
            }
            0xCB9E => {
                machine.reset_bit(3u8, Intel8080Register::M);
                16u8
            }
            0xCB9F => {
                machine.reset_bit(3u8, Intel8080Register::A);
                8u8
            }
            0xCBA0 => {
                machine.reset_bit(4u8, Intel8080Register::B);
                8u8
            }
            0xCBA1 => {
                machine.reset_bit(4u8, Intel8080Register::C);
                8u8
            }
            0xCBA2 => {
                machine.reset_bit(4u8, Intel8080Register::D);
                8u8
            }
            0xCBA3 => {
                machine.reset_bit(4u8, Intel8080Register::E);
                8u8
            }
            0xCBA4 => {
                machine.reset_bit(4u8, Intel8080Register::H);
                8u8
            }
            0xCBA5 => {
                machine.reset_bit(4u8, Intel8080Register::L);
                8u8
            }
            0xCBA6 => {
                machine.reset_bit(4u8, Intel8080Register::M);
                16u8
            }
            0xCBA7 => {
                machine.reset_bit(4u8, Intel8080Register::A);
                8u8
            }
            0xCBA8 => {
                machine.reset_bit(5u8, Intel8080Register::B);
                8u8
            }
            0xCBA9 => {
                machine.reset_bit(5u8, Intel8080Register::C);
                8u8
            }
            0xCBAA => {
                machine.reset_bit(5u8, Intel8080Register::D);
                8u8
            }
            0xCBAB => {
                machine.reset_bit(5u8, Intel8080Register::E);
                8u8
            }
            0xCBAC => {
                machine.reset_bit(5u8, Intel8080Register::H);
                8u8
            }
            0xCBAD => {
                machine.reset_bit(5u8, Intel8080Register::L);
                8u8
            }
            0xCBAE => {
                machine.reset_bit(5u8, Intel8080Register::M);
                16u8
            }
            0xCBAF => {
                machine.reset_bit(5u8, Intel8080Register::A);
                8u8
            }
            0xCBB0 => {
                machine.reset_bit(6u8, Intel8080Register::B);
                8u8
            }
            0xCBB1 => {
                machine.reset_bit(6u8, Intel8080Register::C);
                8u8
            }
            0xCBB2 => {
                machine.reset_bit(6u8, Intel8080Register::D);
                8u8
            }
            0xCBB3 => {
                machine.reset_bit(6u8, Intel8080Register::E);
                8u8
            }
            0xCBB4 => {
                machine.reset_bit(6u8, Intel8080Register::H);
                8u8
            }
            0xCBB5 => {
                machine.reset_bit(6u8, Intel8080Register::L);
                8u8
            }
            0xCBB6 => {
                machine.reset_bit(6u8, Intel8080Register::M);
                16u8
            }
            0xCBB7 => {
                machine.reset_bit(6u8, Intel8080Register::A);
                8u8
            }
            0xCBB8 => {
                machine.reset_bit(7u8, Intel8080Register::B);
                8u8
            }
            0xCBB9 => {
                machine.reset_bit(7u8, Intel8080Register::C);
                8u8
            }
            0xCBBA => {
                machine.reset_bit(7u8, Intel8080Register::D);
                8u8
            }
            0xCBBB => {
                machine.reset_bit(7u8, Intel8080Register::E);
                8u8
            }
            0xCBBC => {
                machine.reset_bit(7u8, Intel8080Register::H);
                8u8
            }
            0xCBBD => {
                machine.reset_bit(7u8, Intel8080Register::L);
                8u8
            }
            0xCBBE => {
                machine.reset_bit(7u8, Intel8080Register::M);
                16u8
            }
            0xCBBF => {
                machine.reset_bit(7u8, Intel8080Register::A);
                8u8
            }
            0xCBC0 => {
                machine.set_bit(0u8, Intel8080Register::B);
                8u8
            }
            0xCBC1 => {
                machine.set_bit(0u8, Intel8080Register::C);
                8u8
            }
            0xCBC2 => {
                machine.set_bit(0u8, Intel8080Register::D);
                8u8
            }
            0xCBC3 => {
                machine.set_bit(0u8, Intel8080Register::E);
                8u8
            }
            0xCBC4 => {
                machine.set_bit(0u8, Intel8080Register::H);
                8u8
            }
            0xCBC5 => {
                machine.set_bit(0u8, Intel8080Register::L);
                8u8
            }
            0xCBC6 => {
                machine.set_bit(0u8, Intel8080Register::M);
                16u8
            }
            0xCBC7 => {
                machine.set_bit(0u8, Intel8080Register::A);
                8u8
            }
            0xCBC8 => {
                machine.set_bit(1u8, Intel8080Register::B);
                8u8
            }
            0xCBC9 => {
                machine.set_bit(1u8, Intel8080Register::C);
                8u8
            }
            0xCBCA => {
                machine.set_bit(1u8, Intel8080Register::D);
                8u8
            }
            0xCBCB => {
                machine.set_bit(1u8, Intel8080Register::E);
                8u8
            }
            0xCBCC => {
                machine.set_bit(1u8, Intel8080Register::H);
                8u8
            }
            0xCBCD => {
                machine.set_bit(1u8, Intel8080Register::L);
                8u8
            }
            0xCBCE => {
                machine.set_bit(1u8, Intel8080Register::M);
                16u8
            }
            0xCBCF => {
                machine.set_bit(1u8, Intel8080Register::A);
                8u8
            }
            0xCBD0 => {
                machine.set_bit(2u8, Intel8080Register::B);
                8u8
            }
            0xCBD1 => {
                machine.set_bit(2u8, Intel8080Register::C);
                8u8
            }
            0xCBD2 => {
                machine.set_bit(2u8, Intel8080Register::D);
                8u8
            }
            0xCBD3 => {
                machine.set_bit(2u8, Intel8080Register::E);
                8u8
            }
            0xCBD4 => {
                machine.set_bit(2u8, Intel8080Register::H);
                8u8
            }
            0xCBD5 => {
                machine.set_bit(2u8, Intel8080Register::L);
                8u8
            }
            0xCBD6 => {
                machine.set_bit(2u8, Intel8080Register::M);
                16u8
            }
            0xCBD7 => {
                machine.set_bit(2u8, Intel8080Register::A);
                8u8
            }
            0xCBD8 => {
                machine.set_bit(3u8, Intel8080Register::B);
                8u8
            }
            0xCBD9 => {
                machine.set_bit(3u8, Intel8080Register::C);
                8u8
            }
            0xCBDA => {
                machine.set_bit(3u8, Intel8080Register::D);
                8u8
            }
            0xCBDB => {
                machine.set_bit(3u8, Intel8080Register::E);
                8u8
            }
            0xCBDC => {
                machine.set_bit(3u8, Intel8080Register::H);
                8u8
            }
            0xCBDD => {
                machine.set_bit(3u8, Intel8080Register::L);
                8u8
            }
            0xCBDE => {
                machine.set_bit(3u8, Intel8080Register::M);
                16u8
            }
            0xCBDF => {
                machine.set_bit(3u8, Intel8080Register::A);
                8u8
            }
            0xCBE0 => {
                machine.set_bit(4u8, Intel8080Register::B);
                8u8
            }
            0xCBE1 => {
                machine.set_bit(4u8, Intel8080Register::C);
                8u8
            }
            0xCBE2 => {
                machine.set_bit(4u8, Intel8080Register::D);
                8u8
            }
            0xCBE3 => {
                machine.set_bit(4u8, Intel8080Register::E);
                8u8
            }
            0xCBE4 => {
                machine.set_bit(4u8, Intel8080Register::H);
                8u8
            }
            0xCBE5 => {
                machine.set_bit(4u8, Intel8080Register::L);
                8u8
            }
            0xCBE6 => {
                machine.set_bit(4u8, Intel8080Register::M);
                16u8
            }
            0xCBE7 => {
                machine.set_bit(4u8, Intel8080Register::A);
                8u8
            }
            0xCBE8 => {
                machine.set_bit(5u8, Intel8080Register::B);
                8u8
            }
            0xCBE9 => {
                machine.set_bit(5u8, Intel8080Register::C);
                8u8
            }
            0xCBEA => {
                machine.set_bit(5u8, Intel8080Register::D);
                8u8
            }
            0xCBEB => {
                machine.set_bit(5u8, Intel8080Register::E);
                8u8
            }
            0xCBEC => {
                machine.set_bit(5u8, Intel8080Register::H);
                8u8
            }
            0xCBED => {
                machine.set_bit(5u8, Intel8080Register::L);
                8u8
            }
            0xCBEE => {
                machine.set_bit(5u8, Intel8080Register::M);
                16u8
            }
            0xCBEF => {
                machine.set_bit(5u8, Intel8080Register::A);
                8u8
            }
            0xCBF0 => {
                machine.set_bit(6u8, Intel8080Register::B);
                8u8
            }
            0xCBF1 => {
                machine.set_bit(6u8, Intel8080Register::C);
                8u8
            }
            0xCBF2 => {
                machine.set_bit(6u8, Intel8080Register::D);
                8u8
            }
            0xCBF3 => {
                machine.set_bit(6u8, Intel8080Register::E);
                8u8
            }
            0xCBF4 => {
                machine.set_bit(6u8, Intel8080Register::H);
                8u8
            }
            0xCBF5 => {
                machine.set_bit(6u8, Intel8080Register::L);
                8u8
            }
            0xCBF6 => {
                machine.set_bit(6u8, Intel8080Register::M);
                16u8
            }
            0xCBF7 => {
                machine.set_bit(6u8, Intel8080Register::A);
                8u8
            }
            0xCBF8 => {
                machine.set_bit(7u8, Intel8080Register::B);
                8u8
            }
            0xCBF9 => {
                machine.set_bit(7u8, Intel8080Register::C);
                8u8
            }
            0xCBFA => {
                machine.set_bit(7u8, Intel8080Register::D);
                8u8
            }
            0xCBFB => {
                machine.set_bit(7u8, Intel8080Register::E);
                8u8
            }
            0xCBFC => {
                machine.set_bit(7u8, Intel8080Register::H);
                8u8
            }
            0xCBFD => {
                machine.set_bit(7u8, Intel8080Register::L);
                8u8
            }
            0xCBFE => {
                machine.set_bit(7u8, Intel8080Register::M);
                16u8
            }
            0xCBFF => {
                machine.set_bit(7u8, Intel8080Register::A);
                8u8
            }
            v => panic!("Unknown opcode {}", v),
        },
        0xCC => {
            machine.call_if_zero(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xCD => {
            machine.call(stream.read_u16::<LittleEndian>().unwrap());
            24u8
        }
        0xCE => {
            machine.add_immediate_to_accumulator_with_carry(stream.read_u8().unwrap());
            8u8
        }
        0xCF => {
            machine.restart(1u8);
            16u8
        }
        0xD0 => {
            machine.return_if_no_carry();
            8u8
        }
        0xD1 => {
            machine.pop_data_off_stack(Intel8080Register::D);
            12u8
        }
        0xD2 => {
            machine.jump_if_no_carry(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xD4 => {
            machine.call_if_no_carry(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xD5 => {
            machine.push_data_onto_stack(Intel8080Register::D);
            16u8
        }
        0xD6 => {
            machine.subtract_immediate_from_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xD7 => {
            machine.restart(2u8);
            16u8
        }
        0xD8 => {
            machine.return_if_carry();
            8u8
        }
        0xD9 => {
            machine.return_and_enable_interrupts();
            16u8
        }
        0xDA => {
            machine.jump_if_carry(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xDC => {
            machine.call_if_carry(stream.read_u16::<LittleEndian>().unwrap());
            12u8
        }
        0xDE => {
            machine.subtract_immediate_from_accumulator_with_borrow(stream.read_u8().unwrap());
            8u8
        }
        0xDF => {
            machine.restart(3u8);
            16u8
        }
        0xE0 => {
            machine.store_accumulator_direct_one_byte(stream.read_u8().unwrap());
            12u8
        }
        0xE1 => {
            machine.pop_data_off_stack(Intel8080Register::H);
            12u8
        }
        0xE2 => {
            machine.store_accumulator_one_byte();
            8u8
        }
        0xE5 => {
            machine.push_data_onto_stack(Intel8080Register::H);
            16u8
        }
        0xE6 => {
            machine.and_immediate_with_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xE7 => {
            machine.restart(4u8);
            16u8
        }
        0xE8 => {
            machine.add_immediate_to_sp(stream.read_u8().unwrap());
            16u8
        }
        0xE9 => {
            machine.load_program_counter();
            4u8
        }
        0xEA => {
            machine.store_accumulator_direct(stream.read_u16::<LittleEndian>().unwrap());
            16u8
        }
        0xEE => {
            machine.exclusive_or_immediate_with_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xEF => {
            machine.restart(5u8);
            16u8
        }
        0xF0 => {
            machine.load_accumulator_direct_one_byte(stream.read_u8().unwrap());
            12u8
        }
        0xF1 => {
            machine.pop_data_off_stack(Intel8080Register::PSW);
            12u8
        }
        0xF2 => {
            machine.load_accumulator_one_byte();
            8u8
        }
        0xF3 => {
            machine.disable_interrupts();
            4u8
        }
        0xF5 => {
            machine.push_data_onto_stack(Intel8080Register::PSW);
            16u8
        }
        0xF6 => {
            machine.or_immediate_with_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xF7 => {
            machine.restart(6u8);
            16u8
        }
        0xF8 => {
            machine.store_sp_plus_immediate(stream.read_u8().unwrap());
            12u8
        }
        0xF9 => {
            machine.load_sp_from_h_and_l();
            8u8
        }
        0xFA => {
            machine.load_accumulator_direct(stream.read_u16::<LittleEndian>().unwrap());
            16u8
        }
        0xFB => {
            machine.enable_interrupts();
            4u8
        }
        0xFE => {
            machine.compare_immediate_with_accumulator(stream.read_u8().unwrap());
            8u8
        }
        0xFF => {
            machine.restart(7u8);
            16u8
        }
        v => panic!("Unknown opcode {}", v),
    }
}
pub fn get_lr35902_instruction<R: io::Read>(mut stream: R) -> Option<Vec<u8>> {
    let (mut instr, size) = match stream.read_u8().unwrap() {
        0x00 => (vec![0x00], 1u8),
        0x01 => (vec![0x01], 3u8),
        0x02 => (vec![0x02], 1u8),
        0x03 => (vec![0x03], 1u8),
        0x04 => (vec![0x04], 1u8),
        0x05 => (vec![0x05], 1u8),
        0x06 => (vec![0x06], 2u8),
        0x07 => (vec![0x07], 1u8),
        0x08 => (vec![0x08], 3u8),
        0x09 => (vec![0x09], 1u8),
        0x0A => (vec![0x0A], 1u8),
        0x0B => (vec![0x0B], 1u8),
        0x0C => (vec![0x0C], 1u8),
        0x0D => (vec![0x0D], 1u8),
        0x0E => (vec![0x0E], 2u8),
        0x0F => (vec![0x0F], 1u8),
        0x10 => match (0x10 as u16) << 8
            | match stream.read_u8() {
                Ok(x) => x,
                _ => return None,
            } as u16
        {
            0x1000 => (vec![0x10, 0x00], 2u8),
            _ => return None,
        },
        0x11 => (vec![0x11], 3u8),
        0x12 => (vec![0x12], 1u8),
        0x13 => (vec![0x13], 1u8),
        0x14 => (vec![0x14], 1u8),
        0x15 => (vec![0x15], 1u8),
        0x16 => (vec![0x16], 2u8),
        0x17 => (vec![0x17], 1u8),
        0x18 => (vec![0x18], 2u8),
        0x19 => (vec![0x19], 1u8),
        0x1A => (vec![0x1A], 1u8),
        0x1B => (vec![0x1B], 1u8),
        0x1C => (vec![0x1C], 1u8),
        0x1D => (vec![0x1D], 1u8),
        0x1E => (vec![0x1E], 2u8),
        0x1F => (vec![0x1F], 1u8),
        0x20 => (vec![0x20], 2u8),
        0x21 => (vec![0x21], 3u8),
        0x22 => (vec![0x22], 1u8),
        0x23 => (vec![0x23], 1u8),
        0x24 => (vec![0x24], 1u8),
        0x25 => (vec![0x25], 1u8),
        0x26 => (vec![0x26], 2u8),
        0x27 => (vec![0x27], 1u8),
        0x28 => (vec![0x28], 2u8),
        0x29 => (vec![0x29], 1u8),
        0x2A => (vec![0x2A], 1u8),
        0x2B => (vec![0x2B], 1u8),
        0x2C => (vec![0x2C], 1u8),
        0x2D => (vec![0x2D], 1u8),
        0x2E => (vec![0x2E], 2u8),
        0x2F => (vec![0x2F], 1u8),
        0x30 => (vec![0x30], 2u8),
        0x31 => (vec![0x31], 3u8),
        0x32 => (vec![0x32], 1u8),
        0x33 => (vec![0x33], 1u8),
        0x34 => (vec![0x34], 1u8),
        0x35 => (vec![0x35], 1u8),
        0x36 => (vec![0x36], 2u8),
        0x37 => (vec![0x37], 1u8),
        0x38 => (vec![0x38], 2u8),
        0x39 => (vec![0x39], 1u8),
        0x3A => (vec![0x3A], 1u8),
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
        0xCB => match (0xCB as u16) << 8
            | match stream.read_u8() {
                Ok(x) => x,
                _ => return None,
            } as u16
        {
            0xCB00 => (vec![0xCB, 0x00], 2u8),
            0xCB01 => (vec![0xCB, 0x01], 2u8),
            0xCB02 => (vec![0xCB, 0x02], 2u8),
            0xCB03 => (vec![0xCB, 0x03], 2u8),
            0xCB04 => (vec![0xCB, 0x04], 2u8),
            0xCB05 => (vec![0xCB, 0x05], 2u8),
            0xCB06 => (vec![0xCB, 0x06], 2u8),
            0xCB07 => (vec![0xCB, 0x07], 2u8),
            0xCB08 => (vec![0xCB, 0x08], 2u8),
            0xCB09 => (vec![0xCB, 0x09], 2u8),
            0xCB0A => (vec![0xCB, 0x0A], 2u8),
            0xCB0B => (vec![0xCB, 0x0B], 2u8),
            0xCB0C => (vec![0xCB, 0x0C], 2u8),
            0xCB0D => (vec![0xCB, 0x0D], 2u8),
            0xCB0E => (vec![0xCB, 0x0E], 2u8),
            0xCB0F => (vec![0xCB, 0x0F], 2u8),
            0xCB10 => (vec![0xCB, 0x10], 2u8),
            0xCB11 => (vec![0xCB, 0x11], 2u8),
            0xCB12 => (vec![0xCB, 0x12], 2u8),
            0xCB13 => (vec![0xCB, 0x13], 2u8),
            0xCB14 => (vec![0xCB, 0x14], 2u8),
            0xCB15 => (vec![0xCB, 0x15], 2u8),
            0xCB16 => (vec![0xCB, 0x16], 2u8),
            0xCB17 => (vec![0xCB, 0x17], 2u8),
            0xCB18 => (vec![0xCB, 0x18], 2u8),
            0xCB19 => (vec![0xCB, 0x19], 2u8),
            0xCB1A => (vec![0xCB, 0x1A], 2u8),
            0xCB1B => (vec![0xCB, 0x1B], 2u8),
            0xCB1C => (vec![0xCB, 0x1C], 2u8),
            0xCB1D => (vec![0xCB, 0x1D], 2u8),
            0xCB1E => (vec![0xCB, 0x1E], 2u8),
            0xCB1F => (vec![0xCB, 0x1F], 2u8),
            0xCB20 => (vec![0xCB, 0x20], 2u8),
            0xCB21 => (vec![0xCB, 0x21], 2u8),
            0xCB22 => (vec![0xCB, 0x22], 2u8),
            0xCB23 => (vec![0xCB, 0x23], 2u8),
            0xCB24 => (vec![0xCB, 0x24], 2u8),
            0xCB25 => (vec![0xCB, 0x25], 2u8),
            0xCB26 => (vec![0xCB, 0x26], 2u8),
            0xCB27 => (vec![0xCB, 0x27], 2u8),
            0xCB28 => (vec![0xCB, 0x28], 2u8),
            0xCB29 => (vec![0xCB, 0x29], 2u8),
            0xCB2A => (vec![0xCB, 0x2A], 2u8),
            0xCB2B => (vec![0xCB, 0x2B], 2u8),
            0xCB2C => (vec![0xCB, 0x2C], 2u8),
            0xCB2D => (vec![0xCB, 0x2D], 2u8),
            0xCB2E => (vec![0xCB, 0x2E], 2u8),
            0xCB2F => (vec![0xCB, 0x2F], 2u8),
            0xCB30 => (vec![0xCB, 0x30], 2u8),
            0xCB31 => (vec![0xCB, 0x31], 2u8),
            0xCB32 => (vec![0xCB, 0x32], 2u8),
            0xCB33 => (vec![0xCB, 0x33], 2u8),
            0xCB34 => (vec![0xCB, 0x34], 2u8),
            0xCB35 => (vec![0xCB, 0x35], 2u8),
            0xCB36 => (vec![0xCB, 0x36], 2u8),
            0xCB37 => (vec![0xCB, 0x37], 2u8),
            0xCB38 => (vec![0xCB, 0x38], 2u8),
            0xCB39 => (vec![0xCB, 0x39], 2u8),
            0xCB3A => (vec![0xCB, 0x3A], 2u8),
            0xCB3B => (vec![0xCB, 0x3B], 2u8),
            0xCB3C => (vec![0xCB, 0x3C], 2u8),
            0xCB3D => (vec![0xCB, 0x3D], 2u8),
            0xCB3E => (vec![0xCB, 0x3E], 2u8),
            0xCB3F => (vec![0xCB, 0x3F], 2u8),
            0xCB40 => (vec![0xCB, 0x40], 2u8),
            0xCB41 => (vec![0xCB, 0x41], 2u8),
            0xCB42 => (vec![0xCB, 0x42], 2u8),
            0xCB43 => (vec![0xCB, 0x43], 2u8),
            0xCB44 => (vec![0xCB, 0x44], 2u8),
            0xCB45 => (vec![0xCB, 0x45], 2u8),
            0xCB46 => (vec![0xCB, 0x46], 2u8),
            0xCB47 => (vec![0xCB, 0x47], 2u8),
            0xCB48 => (vec![0xCB, 0x48], 2u8),
            0xCB49 => (vec![0xCB, 0x49], 2u8),
            0xCB4A => (vec![0xCB, 0x4A], 2u8),
            0xCB4B => (vec![0xCB, 0x4B], 2u8),
            0xCB4C => (vec![0xCB, 0x4C], 2u8),
            0xCB4D => (vec![0xCB, 0x4D], 2u8),
            0xCB4E => (vec![0xCB, 0x4E], 2u8),
            0xCB4F => (vec![0xCB, 0x4F], 2u8),
            0xCB50 => (vec![0xCB, 0x50], 2u8),
            0xCB51 => (vec![0xCB, 0x51], 2u8),
            0xCB52 => (vec![0xCB, 0x52], 2u8),
            0xCB53 => (vec![0xCB, 0x53], 2u8),
            0xCB54 => (vec![0xCB, 0x54], 2u8),
            0xCB55 => (vec![0xCB, 0x55], 2u8),
            0xCB56 => (vec![0xCB, 0x56], 2u8),
            0xCB57 => (vec![0xCB, 0x57], 2u8),
            0xCB58 => (vec![0xCB, 0x58], 2u8),
            0xCB59 => (vec![0xCB, 0x59], 2u8),
            0xCB5A => (vec![0xCB, 0x5A], 2u8),
            0xCB5B => (vec![0xCB, 0x5B], 2u8),
            0xCB5C => (vec![0xCB, 0x5C], 2u8),
            0xCB5D => (vec![0xCB, 0x5D], 2u8),
            0xCB5E => (vec![0xCB, 0x5E], 2u8),
            0xCB5F => (vec![0xCB, 0x5F], 2u8),
            0xCB60 => (vec![0xCB, 0x60], 2u8),
            0xCB61 => (vec![0xCB, 0x61], 2u8),
            0xCB62 => (vec![0xCB, 0x62], 2u8),
            0xCB63 => (vec![0xCB, 0x63], 2u8),
            0xCB64 => (vec![0xCB, 0x64], 2u8),
            0xCB65 => (vec![0xCB, 0x65], 2u8),
            0xCB66 => (vec![0xCB, 0x66], 2u8),
            0xCB67 => (vec![0xCB, 0x67], 2u8),
            0xCB68 => (vec![0xCB, 0x68], 2u8),
            0xCB69 => (vec![0xCB, 0x69], 2u8),
            0xCB6A => (vec![0xCB, 0x6A], 2u8),
            0xCB6B => (vec![0xCB, 0x6B], 2u8),
            0xCB6C => (vec![0xCB, 0x6C], 2u8),
            0xCB6D => (vec![0xCB, 0x6D], 2u8),
            0xCB6E => (vec![0xCB, 0x6E], 2u8),
            0xCB6F => (vec![0xCB, 0x6F], 2u8),
            0xCB70 => (vec![0xCB, 0x70], 2u8),
            0xCB71 => (vec![0xCB, 0x71], 2u8),
            0xCB72 => (vec![0xCB, 0x72], 2u8),
            0xCB73 => (vec![0xCB, 0x73], 2u8),
            0xCB74 => (vec![0xCB, 0x74], 2u8),
            0xCB75 => (vec![0xCB, 0x75], 2u8),
            0xCB76 => (vec![0xCB, 0x76], 2u8),
            0xCB77 => (vec![0xCB, 0x77], 2u8),
            0xCB78 => (vec![0xCB, 0x78], 2u8),
            0xCB79 => (vec![0xCB, 0x79], 2u8),
            0xCB7A => (vec![0xCB, 0x7A], 2u8),
            0xCB7B => (vec![0xCB, 0x7B], 2u8),
            0xCB7C => (vec![0xCB, 0x7C], 2u8),
            0xCB7D => (vec![0xCB, 0x7D], 2u8),
            0xCB7E => (vec![0xCB, 0x7E], 2u8),
            0xCB7F => (vec![0xCB, 0x7F], 2u8),
            0xCB80 => (vec![0xCB, 0x80], 2u8),
            0xCB81 => (vec![0xCB, 0x81], 2u8),
            0xCB82 => (vec![0xCB, 0x82], 2u8),
            0xCB83 => (vec![0xCB, 0x83], 2u8),
            0xCB84 => (vec![0xCB, 0x84], 2u8),
            0xCB85 => (vec![0xCB, 0x85], 2u8),
            0xCB86 => (vec![0xCB, 0x86], 2u8),
            0xCB87 => (vec![0xCB, 0x87], 2u8),
            0xCB88 => (vec![0xCB, 0x88], 2u8),
            0xCB89 => (vec![0xCB, 0x89], 2u8),
            0xCB8A => (vec![0xCB, 0x8A], 2u8),
            0xCB8B => (vec![0xCB, 0x8B], 2u8),
            0xCB8C => (vec![0xCB, 0x8C], 2u8),
            0xCB8D => (vec![0xCB, 0x8D], 2u8),
            0xCB8E => (vec![0xCB, 0x8E], 2u8),
            0xCB8F => (vec![0xCB, 0x8F], 2u8),
            0xCB90 => (vec![0xCB, 0x90], 2u8),
            0xCB91 => (vec![0xCB, 0x91], 2u8),
            0xCB92 => (vec![0xCB, 0x92], 2u8),
            0xCB93 => (vec![0xCB, 0x93], 2u8),
            0xCB94 => (vec![0xCB, 0x94], 2u8),
            0xCB95 => (vec![0xCB, 0x95], 2u8),
            0xCB96 => (vec![0xCB, 0x96], 2u8),
            0xCB97 => (vec![0xCB, 0x97], 2u8),
            0xCB98 => (vec![0xCB, 0x98], 2u8),
            0xCB99 => (vec![0xCB, 0x99], 2u8),
            0xCB9A => (vec![0xCB, 0x9A], 2u8),
            0xCB9B => (vec![0xCB, 0x9B], 2u8),
            0xCB9C => (vec![0xCB, 0x9C], 2u8),
            0xCB9D => (vec![0xCB, 0x9D], 2u8),
            0xCB9E => (vec![0xCB, 0x9E], 2u8),
            0xCB9F => (vec![0xCB, 0x9F], 2u8),
            0xCBA0 => (vec![0xCB, 0xA0], 2u8),
            0xCBA1 => (vec![0xCB, 0xA1], 2u8),
            0xCBA2 => (vec![0xCB, 0xA2], 2u8),
            0xCBA3 => (vec![0xCB, 0xA3], 2u8),
            0xCBA4 => (vec![0xCB, 0xA4], 2u8),
            0xCBA5 => (vec![0xCB, 0xA5], 2u8),
            0xCBA6 => (vec![0xCB, 0xA6], 2u8),
            0xCBA7 => (vec![0xCB, 0xA7], 2u8),
            0xCBA8 => (vec![0xCB, 0xA8], 2u8),
            0xCBA9 => (vec![0xCB, 0xA9], 2u8),
            0xCBAA => (vec![0xCB, 0xAA], 2u8),
            0xCBAB => (vec![0xCB, 0xAB], 2u8),
            0xCBAC => (vec![0xCB, 0xAC], 2u8),
            0xCBAD => (vec![0xCB, 0xAD], 2u8),
            0xCBAE => (vec![0xCB, 0xAE], 2u8),
            0xCBAF => (vec![0xCB, 0xAF], 2u8),
            0xCBB0 => (vec![0xCB, 0xB0], 2u8),
            0xCBB1 => (vec![0xCB, 0xB1], 2u8),
            0xCBB2 => (vec![0xCB, 0xB2], 2u8),
            0xCBB3 => (vec![0xCB, 0xB3], 2u8),
            0xCBB4 => (vec![0xCB, 0xB4], 2u8),
            0xCBB5 => (vec![0xCB, 0xB5], 2u8),
            0xCBB6 => (vec![0xCB, 0xB6], 2u8),
            0xCBB7 => (vec![0xCB, 0xB7], 2u8),
            0xCBB8 => (vec![0xCB, 0xB8], 2u8),
            0xCBB9 => (vec![0xCB, 0xB9], 2u8),
            0xCBBA => (vec![0xCB, 0xBA], 2u8),
            0xCBBB => (vec![0xCB, 0xBB], 2u8),
            0xCBBC => (vec![0xCB, 0xBC], 2u8),
            0xCBBD => (vec![0xCB, 0xBD], 2u8),
            0xCBBE => (vec![0xCB, 0xBE], 2u8),
            0xCBBF => (vec![0xCB, 0xBF], 2u8),
            0xCBC0 => (vec![0xCB, 0xC0], 2u8),
            0xCBC1 => (vec![0xCB, 0xC1], 2u8),
            0xCBC2 => (vec![0xCB, 0xC2], 2u8),
            0xCBC3 => (vec![0xCB, 0xC3], 2u8),
            0xCBC4 => (vec![0xCB, 0xC4], 2u8),
            0xCBC5 => (vec![0xCB, 0xC5], 2u8),
            0xCBC6 => (vec![0xCB, 0xC6], 2u8),
            0xCBC7 => (vec![0xCB, 0xC7], 2u8),
            0xCBC8 => (vec![0xCB, 0xC8], 2u8),
            0xCBC9 => (vec![0xCB, 0xC9], 2u8),
            0xCBCA => (vec![0xCB, 0xCA], 2u8),
            0xCBCB => (vec![0xCB, 0xCB], 2u8),
            0xCBCC => (vec![0xCB, 0xCC], 2u8),
            0xCBCD => (vec![0xCB, 0xCD], 2u8),
            0xCBCE => (vec![0xCB, 0xCE], 2u8),
            0xCBCF => (vec![0xCB, 0xCF], 2u8),
            0xCBD0 => (vec![0xCB, 0xD0], 2u8),
            0xCBD1 => (vec![0xCB, 0xD1], 2u8),
            0xCBD2 => (vec![0xCB, 0xD2], 2u8),
            0xCBD3 => (vec![0xCB, 0xD3], 2u8),
            0xCBD4 => (vec![0xCB, 0xD4], 2u8),
            0xCBD5 => (vec![0xCB, 0xD5], 2u8),
            0xCBD6 => (vec![0xCB, 0xD6], 2u8),
            0xCBD7 => (vec![0xCB, 0xD7], 2u8),
            0xCBD8 => (vec![0xCB, 0xD8], 2u8),
            0xCBD9 => (vec![0xCB, 0xD9], 2u8),
            0xCBDA => (vec![0xCB, 0xDA], 2u8),
            0xCBDB => (vec![0xCB, 0xDB], 2u8),
            0xCBDC => (vec![0xCB, 0xDC], 2u8),
            0xCBDD => (vec![0xCB, 0xDD], 2u8),
            0xCBDE => (vec![0xCB, 0xDE], 2u8),
            0xCBDF => (vec![0xCB, 0xDF], 2u8),
            0xCBE0 => (vec![0xCB, 0xE0], 2u8),
            0xCBE1 => (vec![0xCB, 0xE1], 2u8),
            0xCBE2 => (vec![0xCB, 0xE2], 2u8),
            0xCBE3 => (vec![0xCB, 0xE3], 2u8),
            0xCBE4 => (vec![0xCB, 0xE4], 2u8),
            0xCBE5 => (vec![0xCB, 0xE5], 2u8),
            0xCBE6 => (vec![0xCB, 0xE6], 2u8),
            0xCBE7 => (vec![0xCB, 0xE7], 2u8),
            0xCBE8 => (vec![0xCB, 0xE8], 2u8),
            0xCBE9 => (vec![0xCB, 0xE9], 2u8),
            0xCBEA => (vec![0xCB, 0xEA], 2u8),
            0xCBEB => (vec![0xCB, 0xEB], 2u8),
            0xCBEC => (vec![0xCB, 0xEC], 2u8),
            0xCBED => (vec![0xCB, 0xED], 2u8),
            0xCBEE => (vec![0xCB, 0xEE], 2u8),
            0xCBEF => (vec![0xCB, 0xEF], 2u8),
            0xCBF0 => (vec![0xCB, 0xF0], 2u8),
            0xCBF1 => (vec![0xCB, 0xF1], 2u8),
            0xCBF2 => (vec![0xCB, 0xF2], 2u8),
            0xCBF3 => (vec![0xCB, 0xF3], 2u8),
            0xCBF4 => (vec![0xCB, 0xF4], 2u8),
            0xCBF5 => (vec![0xCB, 0xF5], 2u8),
            0xCBF6 => (vec![0xCB, 0xF6], 2u8),
            0xCBF7 => (vec![0xCB, 0xF7], 2u8),
            0xCBF8 => (vec![0xCB, 0xF8], 2u8),
            0xCBF9 => (vec![0xCB, 0xF9], 2u8),
            0xCBFA => (vec![0xCB, 0xFA], 2u8),
            0xCBFB => (vec![0xCB, 0xFB], 2u8),
            0xCBFC => (vec![0xCB, 0xFC], 2u8),
            0xCBFD => (vec![0xCB, 0xFD], 2u8),
            0xCBFE => (vec![0xCB, 0xFE], 2u8),
            0xCBFF => (vec![0xCB, 0xFF], 2u8),
            _ => return None,
        },
        0xCC => (vec![0xCC], 3u8),
        0xCD => (vec![0xCD], 3u8),
        0xCE => (vec![0xCE], 2u8),
        0xCF => (vec![0xCF], 1u8),
        0xD0 => (vec![0xD0], 1u8),
        0xD1 => (vec![0xD1], 1u8),
        0xD2 => (vec![0xD2], 3u8),
        0xD4 => (vec![0xD4], 3u8),
        0xD5 => (vec![0xD5], 1u8),
        0xD6 => (vec![0xD6], 2u8),
        0xD7 => (vec![0xD7], 1u8),
        0xD8 => (vec![0xD8], 1u8),
        0xD9 => (vec![0xD9], 1u8),
        0xDA => (vec![0xDA], 3u8),
        0xDC => (vec![0xDC], 3u8),
        0xDE => (vec![0xDE], 2u8),
        0xDF => (vec![0xDF], 1u8),
        0xE0 => (vec![0xE0], 2u8),
        0xE1 => (vec![0xE1], 1u8),
        0xE2 => (vec![0xE2], 1u8),
        0xE5 => (vec![0xE5], 1u8),
        0xE6 => (vec![0xE6], 2u8),
        0xE7 => (vec![0xE7], 1u8),
        0xE8 => (vec![0xE8], 2u8),
        0xE9 => (vec![0xE9], 1u8),
        0xEA => (vec![0xEA], 3u8),
        0xEE => (vec![0xEE], 2u8),
        0xEF => (vec![0xEF], 1u8),
        0xF0 => (vec![0xF0], 2u8),
        0xF1 => (vec![0xF1], 1u8),
        0xF2 => (vec![0xF2], 1u8),
        0xF3 => (vec![0xF3], 1u8),
        0xF5 => (vec![0xF5], 1u8),
        0xF6 => (vec![0xF6], 2u8),
        0xF7 => (vec![0xF7], 1u8),
        0xF8 => (vec![0xF8], 2u8),
        0xF9 => (vec![0xF9], 1u8),
        0xFA => (vec![0xFA], 3u8),
        0xFB => (vec![0xFB], 1u8),
        0xFE => (vec![0xFE], 2u8),
        0xFF => (vec![0xFF], 1u8),
        _ => return None,
    };
    let op_size = instr.len();
    instr.resize(size as usize, 0);
    stream.read(&mut instr[op_size..]).unwrap();
    return Some(instr);
}
impl<'a> LR35902InstructionSet for LR35902InstructionPrinter<'a> {
    fn add_immediate_to_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADI", data1);
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ACI", data1);
    }
    fn add_immediate_to_sp(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADDS", data1);
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
    fn call_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNC", address1);
    }
    fn call_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNZ", address1);
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
        self.error = write!(self.stream_out, "{:04}", "CPL");
    }
    fn complement_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CCF");
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
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "XRI", data1);
    }
    fn halt(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn halt_until_button_press(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STOP");
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INR", register1);
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INX", register1);
    }
    fn jump(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JMP", address1);
    }
    fn jump_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JC", address1);
    }
    fn jump_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNC", address1);
    }
    fn jump_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNZ", address1);
    }
    fn jump_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JZ", address1);
    }
    fn jump_relative(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn jump_relative_if_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRC", data1);
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRNC", data1);
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRNZ", data1);
    }
    fn jump_relative_if_zero(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRZ", data1);
    }
    fn load_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "LDAX", register1);
    }
    fn load_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDAD", address1);
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "LDAB", data1);
    }
    fn load_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "LDAC");
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
    fn move_and_decrement_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} {:?}",
            "MVM-", register1, register2
        );
    }
    fn move_and_increment_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} {:?}",
            "MVM+", register1, register2
        );
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
    fn pop_data_off_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "POP", register1);
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "PUSH", register1);
    }
    fn reset_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "RES", data1, register2);
    }
    fn restart(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} {}", "RST", data1);
    }
    fn return_and_enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RETI");
    }
    fn return_if_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn return_if_no_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn return_if_not_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn return_if_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn return_unconditionally(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RET");
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
    fn rotate_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "RLC", register1);
    }
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "RL", register1);
    }
    fn rotate_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "RRC", register1);
    }
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "RR", register1);
    }
    fn set_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "SET", data1, register2);
    }
    fn set_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SCF");
    }
    fn shift_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SLA", register1);
    }
    fn shift_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRL", register1);
    }
    fn shift_register_right_signed(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRA", register1);
    }
    fn store_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "STAX", register1);
    }
    fn store_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn store_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "STAB", data1);
    }
    fn store_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STAC");
    }
    fn store_sp_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SSPD", address1);
    }
    fn store_sp_plus_immediate(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "STSP", data1);
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
    fn swap_register(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SWAP", register1);
    }
    fn test_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "BIT", data1, register2);
    }
}
