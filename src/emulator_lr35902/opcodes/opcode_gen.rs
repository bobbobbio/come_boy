use emulator_common::Register8080;
use emulator_common::InstructionOption;
use emulator_common::InstructionOption::*;
use emulator_lr35902::opcodes::InstructionPrinterLR35902;
use util::{read_u16, read_u8};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcode_gen.py
 */

pub trait InstructionSetLR35902 {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Register8080);
    fn halt_until_button_press(&mut self);
    fn store_sp_plus_immediate(&mut self, data1: u8);
    fn complement_accumulator(&mut self);
    fn jump_relative(&mut self, data1: u8);
    fn shift_register_left(&mut self, register1: Register8080);
    fn rotate_register_left_through_carry(&mut self, register1: Register8080);
    fn add_immediate_to_sp(&mut self, data1: u8);
    fn set_carry(&mut self);
    fn store_sp_direct(&mut self, address1: u16);
    fn rotate_register_left(&mut self, register1: Register8080);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn double_add(&mut self, register1: Register8080);
    fn rotate_accumulator_right(&mut self);
    fn store_accumulator_direct_two_bytes(&mut self, address1: u16);
    fn move_and_increment_hl(&mut self, register1: Register8080, register2: Register8080);
    fn move_and_decrement_hl(&mut self, register1: Register8080, register2: Register8080);
    fn return_and_enable_interrupts(&mut self);
    fn jump_relative_if_no_carry(&mut self, data1: u8);
    fn set_bit(&mut self, implicit_data1: u8, register2: Register8080);
    fn load_accumulator_direct_one_byte(&mut self, data1: u8);
    fn rotate_register_right(&mut self, register1: Register8080);
    fn decimal_adjust_accumulator(&mut self);
    fn shift_register_right_signed(&mut self, register1: Register8080);
    fn shift_register_right(&mut self, register1: Register8080);
    fn rotate_register_right_through_carry(&mut self, register1: Register8080);
    fn jump_relative_if_zero(&mut self, data1: u8);
    fn swap_register(&mut self, register1: Register8080);
    fn jump_relative_if_not_zero(&mut self, data1: u8);
    fn jump_relative_if_carry(&mut self, data1: u8);
    fn complement_carry(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn test_bit(&mut self, implicit_data1: u8, register2: Register8080);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn store_accumulator_direct_one_byte(&mut self, data1: u8);
}

pub fn dispatch_lr35902_instruction<I: InstructionSetLR35902>(
    mut stream: &[u8],
    machine: &mut I)
{
    let opcode = read_u8(&mut stream).unwrap();
    match opcode {
        0x07 => machine.rotate_accumulator_left(),
        0x08 => machine.store_sp_direct(read_u16(&mut stream).unwrap()),
        0x09 => machine.double_add(Register8080::B),
        0x0F => machine.rotate_accumulator_right(),
        0x17 => machine.rotate_accumulator_left_through_carry(),
        0x18 => machine.jump_relative(read_u8(&mut stream).unwrap()),
        0x19 => machine.double_add(Register8080::D),
        0x1F => machine.rotate_accumulator_right_through_carry(),
        0x20 => machine.jump_relative_if_not_zero(read_u8(&mut stream).unwrap()),
        0x22 => machine.move_and_increment_hl(Register8080::M, Register8080::A),
        0x27 => machine.decimal_adjust_accumulator(),
        0x28 => machine.jump_relative_if_zero(read_u8(&mut stream).unwrap()),
        0x2A => machine.move_and_increment_hl(Register8080::A, Register8080::M),
        0x2F => machine.complement_accumulator(),
        0x30 => machine.jump_relative_if_no_carry(read_u8(&mut stream).unwrap()),
        0x32 => machine.move_and_decrement_hl(Register8080::M, Register8080::A),
        0x37 => machine.set_carry(),
        0x38 => machine.jump_relative_if_carry(read_u8(&mut stream).unwrap()),
        0x3A => machine.move_and_decrement_hl(Register8080::A, Register8080::M),
        0x3F => machine.complement_carry(),
        0xD9 => machine.return_and_enable_interrupts(),
        0xE0 => machine.store_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()),
        0xE8 => machine.add_immediate_to_sp(read_u8(&mut stream).unwrap()),
        0xEA => machine.store_accumulator_direct_two_bytes(read_u16(&mut stream).unwrap()),
        0xF0 => machine.load_accumulator_direct_one_byte(read_u8(&mut stream).unwrap()),
        0xF8 => machine.store_sp_plus_immediate(read_u8(&mut stream).unwrap()),
        0xFA => machine.load_accumulator_direct(read_u16(&mut stream).unwrap()),
        0x10 => match (0x10 as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0x1000 => machine.halt_until_button_press(),
            v => panic!("Unknown opcode {}", v)
        },
        0xCB => match (0xCB as u16) << 8 |
            read_u8(&mut stream).unwrap() as u16 {
            0xCB00 => machine.rotate_register_left(Register8080::B),
            0xCB01 => machine.rotate_register_left(Register8080::C),
            0xCB02 => machine.rotate_register_left(Register8080::D),
            0xCB03 => machine.rotate_register_left(Register8080::E),
            0xCB04 => machine.rotate_register_left(Register8080::H),
            0xCB05 => machine.rotate_register_left(Register8080::L),
            0xCB06 => machine.rotate_register_left(Register8080::M),
            0xCB07 => machine.rotate_register_left(Register8080::A),
            0xCB08 => machine.rotate_register_right(Register8080::B),
            0xCB09 => machine.rotate_register_right(Register8080::C),
            0xCB0A => machine.rotate_register_right(Register8080::D),
            0xCB0B => machine.rotate_register_right(Register8080::E),
            0xCB0C => machine.rotate_register_right(Register8080::H),
            0xCB0D => machine.rotate_register_right(Register8080::L),
            0xCB0E => machine.rotate_register_right(Register8080::M),
            0xCB0F => machine.rotate_register_right(Register8080::A),
            0xCB10 => machine.rotate_register_left_through_carry(Register8080::B),
            0xCB11 => machine.rotate_register_left_through_carry(Register8080::C),
            0xCB12 => machine.rotate_register_left_through_carry(Register8080::D),
            0xCB13 => machine.rotate_register_left_through_carry(Register8080::E),
            0xCB14 => machine.rotate_register_left_through_carry(Register8080::H),
            0xCB15 => machine.rotate_register_left_through_carry(Register8080::L),
            0xCB16 => machine.rotate_register_left_through_carry(Register8080::M),
            0xCB17 => machine.rotate_register_left_through_carry(Register8080::A),
            0xCB18 => machine.rotate_register_right_through_carry(Register8080::B),
            0xCB19 => machine.rotate_register_right_through_carry(Register8080::C),
            0xCB1A => machine.rotate_register_right_through_carry(Register8080::D),
            0xCB1B => machine.rotate_register_right_through_carry(Register8080::E),
            0xCB1C => machine.rotate_register_right_through_carry(Register8080::H),
            0xCB1D => machine.rotate_register_right_through_carry(Register8080::L),
            0xCB1E => machine.rotate_register_right_through_carry(Register8080::M),
            0xCB1F => machine.rotate_register_right_through_carry(Register8080::A),
            0xCB20 => machine.shift_register_left(Register8080::B),
            0xCB21 => machine.shift_register_left(Register8080::C),
            0xCB22 => machine.shift_register_left(Register8080::D),
            0xCB23 => machine.shift_register_left(Register8080::E),
            0xCB24 => machine.shift_register_left(Register8080::H),
            0xCB25 => machine.shift_register_left(Register8080::L),
            0xCB26 => machine.shift_register_left(Register8080::M),
            0xCB27 => machine.shift_register_left(Register8080::A),
            0xCB28 => machine.shift_register_right_signed(Register8080::B),
            0xCB29 => machine.shift_register_right_signed(Register8080::C),
            0xCB2A => machine.shift_register_right_signed(Register8080::D),
            0xCB2B => machine.shift_register_right_signed(Register8080::E),
            0xCB2C => machine.shift_register_right_signed(Register8080::H),
            0xCB2D => machine.shift_register_right_signed(Register8080::L),
            0xCB2E => machine.shift_register_right_signed(Register8080::M),
            0xCB2F => machine.shift_register_right_signed(Register8080::A),
            0xCB30 => machine.swap_register(Register8080::B),
            0xCB31 => machine.swap_register(Register8080::C),
            0xCB32 => machine.swap_register(Register8080::D),
            0xCB33 => machine.swap_register(Register8080::E),
            0xCB34 => machine.swap_register(Register8080::H),
            0xCB35 => machine.swap_register(Register8080::L),
            0xCB36 => machine.swap_register(Register8080::M),
            0xCB37 => machine.swap_register(Register8080::A),
            0xCB38 => machine.shift_register_right(Register8080::B),
            0xCB39 => machine.shift_register_right(Register8080::C),
            0xCB3A => machine.shift_register_right(Register8080::D),
            0xCB3B => machine.shift_register_right(Register8080::E),
            0xCB3C => machine.shift_register_right(Register8080::H),
            0xCB3D => machine.shift_register_right(Register8080::L),
            0xCB3E => machine.shift_register_right(Register8080::M),
            0xCB3F => machine.shift_register_right(Register8080::A),
            0xCB40 => machine.test_bit(0 as u8, Register8080::B),
            0xCB41 => machine.test_bit(0 as u8, Register8080::C),
            0xCB42 => machine.test_bit(0 as u8, Register8080::D),
            0xCB43 => machine.test_bit(0 as u8, Register8080::E),
            0xCB44 => machine.test_bit(0 as u8, Register8080::H),
            0xCB45 => machine.test_bit(0 as u8, Register8080::L),
            0xCB46 => machine.test_bit(0 as u8, Register8080::M),
            0xCB47 => machine.test_bit(0 as u8, Register8080::A),
            0xCB48 => machine.test_bit(1 as u8, Register8080::B),
            0xCB49 => machine.test_bit(1 as u8, Register8080::C),
            0xCB4A => machine.test_bit(1 as u8, Register8080::D),
            0xCB4B => machine.test_bit(1 as u8, Register8080::E),
            0xCB4C => machine.test_bit(1 as u8, Register8080::H),
            0xCB4D => machine.test_bit(1 as u8, Register8080::L),
            0xCB4E => machine.test_bit(1 as u8, Register8080::M),
            0xCB4F => machine.test_bit(1 as u8, Register8080::A),
            0xCB50 => machine.test_bit(2 as u8, Register8080::B),
            0xCB51 => machine.test_bit(2 as u8, Register8080::C),
            0xCB52 => machine.test_bit(2 as u8, Register8080::D),
            0xCB53 => machine.test_bit(2 as u8, Register8080::E),
            0xCB54 => machine.test_bit(2 as u8, Register8080::H),
            0xCB55 => machine.test_bit(2 as u8, Register8080::L),
            0xCB56 => machine.test_bit(2 as u8, Register8080::M),
            0xCB57 => machine.test_bit(2 as u8, Register8080::A),
            0xCB58 => machine.test_bit(3 as u8, Register8080::B),
            0xCB59 => machine.test_bit(3 as u8, Register8080::C),
            0xCB5A => machine.test_bit(3 as u8, Register8080::D),
            0xCB5B => machine.test_bit(3 as u8, Register8080::E),
            0xCB5C => machine.test_bit(3 as u8, Register8080::H),
            0xCB5D => machine.test_bit(3 as u8, Register8080::L),
            0xCB5E => machine.test_bit(3 as u8, Register8080::M),
            0xCB5F => machine.test_bit(3 as u8, Register8080::A),
            0xCB60 => machine.test_bit(4 as u8, Register8080::B),
            0xCB61 => machine.test_bit(4 as u8, Register8080::C),
            0xCB62 => machine.test_bit(4 as u8, Register8080::D),
            0xCB63 => machine.test_bit(4 as u8, Register8080::E),
            0xCB64 => machine.test_bit(4 as u8, Register8080::H),
            0xCB65 => machine.test_bit(4 as u8, Register8080::L),
            0xCB66 => machine.test_bit(4 as u8, Register8080::M),
            0xCB67 => machine.test_bit(4 as u8, Register8080::A),
            0xCB68 => machine.test_bit(5 as u8, Register8080::B),
            0xCB69 => machine.test_bit(5 as u8, Register8080::C),
            0xCB6A => machine.test_bit(5 as u8, Register8080::D),
            0xCB6B => machine.test_bit(5 as u8, Register8080::E),
            0xCB6C => machine.test_bit(5 as u8, Register8080::H),
            0xCB6D => machine.test_bit(5 as u8, Register8080::L),
            0xCB6E => machine.test_bit(5 as u8, Register8080::M),
            0xCB6F => machine.test_bit(5 as u8, Register8080::A),
            0xCB70 => machine.test_bit(6 as u8, Register8080::B),
            0xCB71 => machine.test_bit(6 as u8, Register8080::C),
            0xCB72 => machine.test_bit(6 as u8, Register8080::D),
            0xCB73 => machine.test_bit(6 as u8, Register8080::E),
            0xCB74 => machine.test_bit(6 as u8, Register8080::H),
            0xCB75 => machine.test_bit(6 as u8, Register8080::L),
            0xCB76 => machine.test_bit(6 as u8, Register8080::M),
            0xCB77 => machine.test_bit(6 as u8, Register8080::A),
            0xCB78 => machine.test_bit(7 as u8, Register8080::B),
            0xCB79 => machine.test_bit(7 as u8, Register8080::C),
            0xCB7A => machine.test_bit(7 as u8, Register8080::D),
            0xCB7B => machine.test_bit(7 as u8, Register8080::E),
            0xCB7C => machine.test_bit(7 as u8, Register8080::H),
            0xCB7D => machine.test_bit(7 as u8, Register8080::L),
            0xCB7E => machine.test_bit(7 as u8, Register8080::M),
            0xCB7F => machine.test_bit(7 as u8, Register8080::A),
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
            0xCB90 => machine.reset_bit(2 as u8, Register8080::B),
            0xCB91 => machine.reset_bit(2 as u8, Register8080::C),
            0xCB92 => machine.reset_bit(2 as u8, Register8080::D),
            0xCB93 => machine.reset_bit(2 as u8, Register8080::E),
            0xCB94 => machine.reset_bit(2 as u8, Register8080::H),
            0xCB95 => machine.reset_bit(2 as u8, Register8080::L),
            0xCB96 => machine.reset_bit(2 as u8, Register8080::M),
            0xCB97 => machine.reset_bit(2 as u8, Register8080::A),
            0xCB98 => machine.reset_bit(3 as u8, Register8080::B),
            0xCB99 => machine.reset_bit(3 as u8, Register8080::C),
            0xCB9A => machine.reset_bit(3 as u8, Register8080::D),
            0xCB9B => machine.reset_bit(3 as u8, Register8080::E),
            0xCB9C => machine.reset_bit(3 as u8, Register8080::H),
            0xCB9D => machine.reset_bit(3 as u8, Register8080::L),
            0xCB9E => machine.reset_bit(3 as u8, Register8080::M),
            0xCB9F => machine.reset_bit(3 as u8, Register8080::A),
            0xCBA0 => machine.reset_bit(4 as u8, Register8080::B),
            0xCBA1 => machine.reset_bit(4 as u8, Register8080::C),
            0xCBA2 => machine.reset_bit(4 as u8, Register8080::D),
            0xCBA3 => machine.reset_bit(4 as u8, Register8080::E),
            0xCBA4 => machine.reset_bit(4 as u8, Register8080::H),
            0xCBA5 => machine.reset_bit(4 as u8, Register8080::L),
            0xCBA6 => machine.reset_bit(4 as u8, Register8080::M),
            0xCBA7 => machine.reset_bit(4 as u8, Register8080::A),
            0xCBA8 => machine.reset_bit(5 as u8, Register8080::B),
            0xCBA9 => machine.reset_bit(5 as u8, Register8080::C),
            0xCBAA => machine.reset_bit(5 as u8, Register8080::D),
            0xCBAB => machine.reset_bit(5 as u8, Register8080::E),
            0xCBAC => machine.reset_bit(5 as u8, Register8080::H),
            0xCBAD => machine.reset_bit(5 as u8, Register8080::L),
            0xCBAE => machine.reset_bit(5 as u8, Register8080::M),
            0xCBAF => machine.reset_bit(5 as u8, Register8080::A),
            0xCBB0 => machine.reset_bit(6 as u8, Register8080::B),
            0xCBB1 => machine.reset_bit(6 as u8, Register8080::C),
            0xCBB2 => machine.reset_bit(6 as u8, Register8080::D),
            0xCBB3 => machine.reset_bit(6 as u8, Register8080::E),
            0xCBB4 => machine.reset_bit(6 as u8, Register8080::H),
            0xCBB5 => machine.reset_bit(6 as u8, Register8080::L),
            0xCBB6 => machine.reset_bit(6 as u8, Register8080::M),
            0xCBB7 => machine.reset_bit(6 as u8, Register8080::A),
            0xCBB8 => machine.reset_bit(7 as u8, Register8080::B),
            0xCBB9 => machine.reset_bit(7 as u8, Register8080::C),
            0xCBBA => machine.reset_bit(7 as u8, Register8080::D),
            0xCBBB => machine.reset_bit(7 as u8, Register8080::E),
            0xCBBC => machine.reset_bit(7 as u8, Register8080::H),
            0xCBBD => machine.reset_bit(7 as u8, Register8080::L),
            0xCBBE => machine.reset_bit(7 as u8, Register8080::M),
            0xCBBF => machine.reset_bit(7 as u8, Register8080::A),
            0xCBC0 => machine.set_bit(0 as u8, Register8080::B),
            0xCBC1 => machine.set_bit(0 as u8, Register8080::C),
            0xCBC2 => machine.set_bit(0 as u8, Register8080::D),
            0xCBC3 => machine.set_bit(0 as u8, Register8080::E),
            0xCBC4 => machine.set_bit(0 as u8, Register8080::H),
            0xCBC5 => machine.set_bit(0 as u8, Register8080::L),
            0xCBC6 => machine.set_bit(0 as u8, Register8080::M),
            0xCBC7 => machine.set_bit(0 as u8, Register8080::A),
            0xCBC8 => machine.set_bit(1 as u8, Register8080::B),
            0xCBC9 => machine.set_bit(1 as u8, Register8080::C),
            0xCBCA => machine.set_bit(1 as u8, Register8080::D),
            0xCBCB => machine.set_bit(1 as u8, Register8080::E),
            0xCBCC => machine.set_bit(1 as u8, Register8080::H),
            0xCBCD => machine.set_bit(1 as u8, Register8080::L),
            0xCBCE => machine.set_bit(1 as u8, Register8080::M),
            0xCBCF => machine.set_bit(1 as u8, Register8080::A),
            0xCBD0 => machine.set_bit(2 as u8, Register8080::B),
            0xCBD1 => machine.set_bit(2 as u8, Register8080::C),
            0xCBD2 => machine.set_bit(2 as u8, Register8080::D),
            0xCBD3 => machine.set_bit(2 as u8, Register8080::E),
            0xCBD4 => machine.set_bit(2 as u8, Register8080::H),
            0xCBD5 => machine.set_bit(2 as u8, Register8080::L),
            0xCBD6 => machine.set_bit(2 as u8, Register8080::M),
            0xCBD7 => machine.set_bit(2 as u8, Register8080::A),
            0xCBD8 => machine.set_bit(3 as u8, Register8080::B),
            0xCBD9 => machine.set_bit(3 as u8, Register8080::C),
            0xCBDA => machine.set_bit(3 as u8, Register8080::D),
            0xCBDB => machine.set_bit(3 as u8, Register8080::E),
            0xCBDC => machine.set_bit(3 as u8, Register8080::H),
            0xCBDD => machine.set_bit(3 as u8, Register8080::L),
            0xCBDE => machine.set_bit(3 as u8, Register8080::M),
            0xCBDF => machine.set_bit(3 as u8, Register8080::A),
            0xCBE0 => machine.set_bit(4 as u8, Register8080::B),
            0xCBE1 => machine.set_bit(4 as u8, Register8080::C),
            0xCBE2 => machine.set_bit(4 as u8, Register8080::D),
            0xCBE3 => machine.set_bit(4 as u8, Register8080::E),
            0xCBE4 => machine.set_bit(4 as u8, Register8080::H),
            0xCBE5 => machine.set_bit(4 as u8, Register8080::L),
            0xCBE6 => machine.set_bit(4 as u8, Register8080::M),
            0xCBE7 => machine.set_bit(4 as u8, Register8080::A),
            0xCBE8 => machine.set_bit(5 as u8, Register8080::B),
            0xCBE9 => machine.set_bit(5 as u8, Register8080::C),
            0xCBEA => machine.set_bit(5 as u8, Register8080::D),
            0xCBEB => machine.set_bit(5 as u8, Register8080::E),
            0xCBEC => machine.set_bit(5 as u8, Register8080::H),
            0xCBED => machine.set_bit(5 as u8, Register8080::L),
            0xCBEE => machine.set_bit(5 as u8, Register8080::M),
            0xCBEF => machine.set_bit(5 as u8, Register8080::A),
            0xCBF0 => machine.set_bit(6 as u8, Register8080::B),
            0xCBF1 => machine.set_bit(6 as u8, Register8080::C),
            0xCBF2 => machine.set_bit(6 as u8, Register8080::D),
            0xCBF3 => machine.set_bit(6 as u8, Register8080::E),
            0xCBF4 => machine.set_bit(6 as u8, Register8080::H),
            0xCBF5 => machine.set_bit(6 as u8, Register8080::L),
            0xCBF6 => machine.set_bit(6 as u8, Register8080::M),
            0xCBF7 => machine.set_bit(6 as u8, Register8080::A),
            0xCBF8 => machine.set_bit(7 as u8, Register8080::B),
            0xCBF9 => machine.set_bit(7 as u8, Register8080::C),
            0xCBFA => machine.set_bit(7 as u8, Register8080::D),
            0xCBFB => machine.set_bit(7 as u8, Register8080::E),
            0xCBFC => machine.set_bit(7 as u8, Register8080::H),
            0xCBFD => machine.set_bit(7 as u8, Register8080::L),
            0xCBFE => machine.set_bit(7 as u8, Register8080::M),
            0xCBFF => machine.set_bit(7 as u8, Register8080::A),
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
        0x07 =>         1,
        0x08 =>         3,
        0x09 =>         1,
        0x0F =>         1,
        0x17 =>         1,
        0x18 =>         2,
        0x19 =>         1,
        0x1F =>         1,
        0x20 =>         2,
        0x22 =>         1,
        0x27 =>         1,
        0x28 =>         2,
        0x2A =>         1,
        0x2F =>         1,
        0x30 =>         2,
        0x32 =>         1,
        0x37 =>         1,
        0x38 =>         2,
        0x3A =>         1,
        0x3F =>         1,
        0xD3 =>         return NotImplemented,
        0xD9 =>         1,
        0xDB =>         return NotImplemented,
        0xDD =>         return NotImplemented,
        0xE0 =>         2,
        0xE3 =>         return NotImplemented,
        0xE4 =>         return NotImplemented,
        0xE8 =>         2,
        0xEA =>         3,
        0xEB =>         return NotImplemented,
        0xEC =>         return NotImplemented,
        0xED =>         return NotImplemented,
        0xF0 =>         2,
        0xF4 =>         return NotImplemented,
        0xF8 =>         2,
        0xFA =>         3,
        0xFC =>         return NotImplemented,
        0xFD =>         return NotImplemented,
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

impl<'a> InstructionSetLR35902 for InstructionPrinterLR35902<'a> {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "RES", implicit_data1, register2);
    }
    fn halt_until_button_press(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "STOP");
    }
    fn store_sp_plus_immediate(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "STSP", data1);
    }
    fn complement_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CPL");
    }
    fn jump_relative(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JRN", data1);
    }
    fn shift_register_left(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SLA", register1);
    }
    fn rotate_register_left_through_carry(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RL", register1);
    }
    fn add_immediate_to_sp(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADDS", data1);
    }
    fn set_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "SCF");
    }
    fn store_sp_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SSPD", address1);
    }
    fn rotate_register_left(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RLC", register1);
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RAL");
    }
    fn double_add(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "DAD", register1);
    }
    fn rotate_accumulator_right(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RRC");
    }
    fn store_accumulator_direct_two_bytes(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn move_and_increment_hl(&mut self, register1: Register8080, register2: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MVM+", register1, register2);
    }
    fn move_and_decrement_hl(&mut self, register1: Register8080, register2: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?} {:?}", "MVM-", register1, register2);
    }
    fn return_and_enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RETI");
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn set_bit(&mut self, implicit_data1: u8, register2: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {} {:?}", "SET", implicit_data1, register2);
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "LDAB", data1);
    }
    fn rotate_register_right(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RRC", register1);
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn shift_register_right_signed(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRA", register1);
    }
    fn shift_register_right(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SRL", register1);
    }
    fn rotate_register_right_through_carry(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "RR", register1);
    }
    fn jump_relative_if_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn swap_register(&mut self, register1: Register8080)
    {
        self.error = write!(self.stream_out, "{:04} {:?}", "SWAP", register1);
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn jump_relative_if_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "JR", data1);
    }
    fn complement_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "CCF");
    }
    fn rotate_accumulator_left(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "RLC");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDAD", address1);
    }
    fn test_bit(&mut self, implicit_data1: u8, register2: Register8080)
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
}
