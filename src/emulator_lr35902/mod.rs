mod opcodes;

use std::collections::{HashSet, VecDeque};
use std::io::{self, Result};
use std::{fmt, str};

pub use emulator_lr35902::opcodes::disassemble_lr35902_rom;

use util::TwosComplement;
use emulator_common::{Register8080, DebuggerOps, Debugger};
use emulator_common::InstructionOption::*;
use emulator_8080::{
    Emulator8080,
    Flag8080,
    InstructionSet8080,
    InstructionSetOps8080,
    dispatch_8080_instruction,
    get_8080_instruction};
use emulator_lr35902::opcodes::create_disassembler;
use emulator_lr35902::opcodes::{
    get_lr35902_instruction, dispatch_lr35902_instruction, InstructionSetLR35902};

#[cfg(test)]
use std::fs::File;

#[cfg(test)]
use std::io::Read;

const ROM_ADDRESS: usize = 0x0100;
// const LCD_ADDRESS: usize = 0x8000;

/*
 *  _____                 _       _
 * | ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |___| | | | | | |_| | | (_| | || (_) | |
 * |_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 *
 *  _     ____  _________  ___   ___ ____
 * | |   |  _ \|___ / ___|/ _ \ / _ \___ \
 * | |   | |_) | |_ \___ \ (_) | | | |__) |
 * | |___|  _ < ___) |__) \__, | |_| / __/
 * |_____|_| \_\____/____/  /_/ \___/_____|
 *
 */

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum FlagLR35902 {
                    // 76543210
    Zero =           0b10000000,
    Subtract =       0b01000000,
    HalfCarry =      0b00010000,
    Carry =          0b00000100,
}

struct EmulatorLR35902<'a> {
    e8080: Emulator8080<'a>,
    crash_message: Option<String>,
    memory_changed: HashSet<u16>,
    pc_trail: VecDeque<u16>
}

impl<'a> EmulatorLR35902<'a> {
    fn new() -> EmulatorLR35902<'a>
    {
        return EmulatorLR35902 {
            e8080: Emulator8080::new(),
            crash_message: None,
            memory_changed: HashSet::new(),
            pc_trail: VecDeque::new()
        };
    }

    fn load_rom(&mut self, rom: &[u8])
    {
        self.e8080.main_memory[0..rom.len()].clone_from_slice(rom);
        self.set_register_pair(Register8080::SP, 0xFFFE);
        self.set_program_counter(ROM_ADDRESS as u16);
    }

    fn set_flag(&mut self, flag: FlagLR35902, value: bool)
    {
        self.e8080.set_flag_u8(flag as u8, value);
    }

    fn read_flag(&self, flag: FlagLR35902) -> bool
    {
        self.e8080.read_flag_u8(flag as u8)
    }

    fn get_relative_address(&self, n: u8) -> u16
    {
        self.read_program_counter().wrapping_add(((n as i8) as i16) as u16)
    }
}

impl<'a> InstructionSetOps8080 for EmulatorLR35902<'a> {
    fn read_memory(&self, address: u16) -> u8
    {
        self.e8080.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8)
    {
        self.e8080.set_memory(address, value);
        self.memory_changed.insert(address);
    }

    fn read_memory_u16(&self, address: u16) -> u16
    {
        self.e8080.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16)
    {
        self.e8080.set_memory_u16(address, value);
        self.memory_changed.insert(address);
        self.memory_changed.insert(address.wrapping_add(1));
    }

    fn set_flag(&mut self, flag: Flag8080, value: bool)
    {
        match flag {
            Flag8080::Zero =>           self.set_flag(FlagLR35902::Zero, value),
            Flag8080::AuxiliaryCarry => self.set_flag(FlagLR35902::HalfCarry, value),
            Flag8080::Carry =>          self.set_flag(FlagLR35902::Carry, value),
            _ => {}
        };
    }

    fn read_flag(&self, flag: Flag8080) -> bool
    {
        match flag {
            Flag8080::Zero =>           self.read_flag(FlagLR35902::Zero),
            Flag8080::AuxiliaryCarry => self.read_flag(FlagLR35902::HalfCarry),
            Flag8080::Carry =>          self.read_flag(FlagLR35902::Carry),
            flag =>                     panic!("LR35902 doesn't know about {:?}", flag)
        }
    }

    fn set_register_pair(&mut self, register: Register8080, value: u16)
    {
        self.e8080.set_register_pair(register, value);
    }

    fn read_register_pair(&self, register: Register8080) -> u16
    {
        self.e8080.read_register_pair(register)
    }

    fn set_register(&mut self, register: Register8080, value: u8)
    {
        self.e8080.set_register(register, value);
    }

    fn read_register(&self, register: Register8080) -> u8
    {
        self.e8080.read_register(register)
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8
    {
        let new_value = value_a.wrapping_add(value_b);

        self.set_flag(FlagLR35902::Zero, new_value == 0);
        if update_carry {
            self.set_flag(FlagLR35902::Carry, value_b > 0xFF - value_a);
        }
        self.set_flag(FlagLR35902::HalfCarry, value_b & 0x0F > 0x0F - (value_a & 0x0F));
        self.set_flag(FlagLR35902::Subtract, false);

        return new_value;
    }

    fn perform_addition_u16(&mut self, value_a: u16, value_b: u16, update_carry: bool) -> u16
    {
        let new_value = value_a.wrapping_add(value_b);

        if update_carry {
            self.set_flag(FlagLR35902::Carry, value_b > (0xFFFF - value_a));
        }
        self.set_flag(FlagLR35902::HalfCarry, value_b & 0x00FF > 0x00FF - (value_a & 0x00FF));
        self.set_flag(FlagLR35902::Subtract, false);

        return new_value;
    }

    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, mut value_b: u8) -> u8
    {
        value_b = value_b.twos_complement();
        let new_value = value_a.wrapping_add(value_b);

        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, true);
        self.set_flag(FlagLR35902::HalfCarry, value_b & 0x0F > 0x0F - (value_a & 0x0F));

        /*
         * When performing subtraction in two's complement arithmetic, the Carry bit is reset when
         * the result is positive.  If the Carry bit is set, the result is negative and present in
         * its two's complement form.  Thus, the Carry bit when set indicates the occurrence of a
         * "borrow."
         */
        self.set_flag(FlagLR35902::Carry, new_value & 0b10000000 != 0);

        return new_value;
    }

    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8
    {
        let new_value = value_a.wrapping_sub(value_b);
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::HalfCarry, value_b & 0x0F > 0x0F - (value_a & 0x0F));
        self.set_flag(FlagLR35902::Subtract, true);
        return new_value;
    }

    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8
    {
        let new_value = value_a & value_b;
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::Carry, false);
        self.set_flag(FlagLR35902::HalfCarry, true);
        return new_value;
    }

    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8
    {
        let new_value = value_a ^ value_b;
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::Carry, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
        return new_value;
    }

    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8
    {
        let new_value = value_a | value_b;
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::Carry, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
        return new_value;
    }

    fn read_program_counter(&self) -> u16
    {
        self.e8080.read_program_counter()
    }

    fn set_program_counter(&mut self, address: u16)
    {
        let pc = self.read_program_counter();
        self.pc_trail.push_front(pc);
        if self.pc_trail.len() > 5 {
            self.pc_trail.pop_back();
        }

        self.e8080.set_program_counter(address);
    }

    fn set_interrupts_enabled(&mut self, value: bool)
    {
        self.e8080.set_interrupts_enabled(value);
    }

    fn get_interrupts_enabled(&self) -> bool
    {
        self.e8080.get_interrupts_enabled()
    }
}

impl<'a> InstructionSetLR35902 for EmulatorLR35902<'a> {
    fn move_and_increment_hl(&mut self, dest_register: Register8080, src_register: Register8080)
    {
        self.move_data(dest_register, src_register);
        self.add_to_register_pair(Register8080::H, 1, false /* update_carry */);
    }

    fn move_and_decrement_hl(&mut self, dest_register: Register8080, src_register: Register8080)
    {
        self.move_data(dest_register, src_register);
        self.subtract_from_register_pair(Register8080::H, 1);
    }

    fn store_accumulator_direct_two_bytes(&mut self, address: u16)
    {
        self.store_accumulator_direct(address);
    }

    fn store_sp_plus_immediate(&mut self, data: u8)
    {
        let address = self.read_register_pair(Register8080::SP).wrapping_add(data as u16);
        let v = self.read_memory(address);
        self.set_register(Register8080::M, v);
    }

    fn add_immediate_to_sp(&mut self, data: u8)
    {
        self.add_to_register_pair(Register8080::SP, data as u16, true /* update_carry */);
    }

    fn store_accumulator_direct_one_byte(&mut self, relative_address: u8)
    {
        let value = self.read_register(Register8080::A);
        self.set_memory(0xFF00 | relative_address as u16, value);
    }

    fn load_accumulator_direct_one_byte(&mut self, relative_address: u8)
    {
        let value = self.read_memory(0xFF00 | relative_address as u16);
        self.set_register(Register8080::A, value);
    }

    fn return_and_enable_interrupts(&mut self)
    {
        self.return_unconditionally();
        self.enable_interrupts();
    }

    fn halt_until_button_press(&mut self)
    {
        unimplemented!();
    }

    fn jump_relative(&mut self, n: u8)
    {
        let address = self.get_relative_address(n);
        self.jump(address);
    }

    fn jump_relative_if_zero(&mut self, n: u8)
    {
        let address = self.get_relative_address(n);
        self.jump_if_zero(address);
    }

    fn jump_relative_if_not_zero(&mut self, n: u8)
    {
        let address = self.get_relative_address(n);
        self.jump_if_not_zero(address);
    }

    fn jump_relative_if_carry(&mut self, n: u8)
    {
        let address = self.get_relative_address(n);
        self.jump_if_carry(address);
    }

    fn jump_relative_if_no_carry(&mut self, n: u8)
    {
        let address = self.get_relative_address(n);
        self.jump_if_no_carry(address);
    }

    fn store_sp_direct(&mut self, address: u16)
    {
        let value = self.read_register_pair(Register8080::SP);
        self.set_memory_u16(address, value);
    }

    fn load_accumulator_direct(&mut self, address: u16)
    {
        InstructionSet8080::load_accumulator_direct(self, address);
    }

    fn reset_bit(&mut self, bit: u8, register: Register8080)
    {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value & !(1u8 << bit));
    }

    fn set_bit(&mut self, bit: u8, register: Register8080)
    {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value | (1u8 << bit));
    }

    fn test_bit(&mut self, bit: u8, register: Register8080)
    {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_flag(FlagLR35902::Zero, (value & (1u8 << bit)) == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, true);
    }

    fn shift_register_right_signed(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.set_register(register, ((value as i8) >> 1) as u8);
        self.set_flag(FlagLR35902::Carry, (value & 1) != 0);
    }

    fn shift_register_right(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.set_register(register, value >> 1);
        self.set_flag(FlagLR35902::Carry, (value & 1) != 0);
    }

    fn shift_register_left(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.set_register(register, value << 1);
        self.set_flag(FlagLR35902::Carry, (value & (1u8 << 7)) != 0);
    }

    fn swap_register(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.set_register(register, (value << 4) | (value >> 4));
    }

    fn rotate_register_right(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right(value);
        self.set_register(register, new_value);
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }

    fn rotate_register_left(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left(value);
        self.set_register(register, new_value);
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }

    fn rotate_register_right_through_carry(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }

    fn rotate_register_left_through_carry(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(FlagLR35902::Zero, new_value == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }

    fn rotate_accumulator_right(&mut self)
    {
        self.rotate_register_right(Register8080::A);
        self.set_flag(FlagLR35902::Zero, false);
    }

    fn rotate_accumulator_left(&mut self)
    {
        self.rotate_register_left(Register8080::A);
        self.set_flag(FlagLR35902::Zero, false);
    }

    fn rotate_accumulator_right_through_carry(&mut self)
    {
        self.rotate_register_right_through_carry(Register8080::A);
        self.set_flag(FlagLR35902::Zero, false);
    }

    fn rotate_accumulator_left_through_carry(&mut self)
    {
        self.rotate_register_left_through_carry(Register8080::A);
        self.set_flag(FlagLR35902::Zero, false);
    }

    fn decimal_adjust_accumulator(&mut self)
    {
        InstructionSet8080::decimal_adjust_accumulator(self);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }
}

#[test]
fn move_and_increment_hl()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::H, 0x1122);
    e.set_register(Register8080::M, 0x99);
    e.move_and_increment_hl(Register8080::A, Register8080::M);
    assert_eq!(e.read_register(Register8080::A), 0x99);
    assert_eq!(e.read_register_pair(Register8080::H), 0x1123);
}

#[test]
fn move_and_decrement_hl()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::H, 0x1122);
    e.set_register(Register8080::M, 0x99);
    e.move_and_decrement_hl(Register8080::A, Register8080::M);
    assert_eq!(e.read_register(Register8080::A), 0x99);
    assert_eq!(e.read_register_pair(Register8080::H), 0x1121);
}

#[test]
fn store_accumulator_direct_two_bytes()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x44);
    e.store_accumulator_direct(0x5588);
    assert_eq!(e.read_memory(0x5588), 0x44);
}

#[test]
fn store_sp_plus_immediate()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x4488);
    e.set_register_pair(Register8080::H, 0x4433);
    e.set_memory(0x4488 + 0x77, 0x99);
    e.store_sp_plus_immediate(0x77);
    assert_eq!(e.read_register(Register8080::M), 0x99);
}

#[test]
fn store_sp_plus_immediate_with_overflow()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0xFF88);
    e.set_register_pair(Register8080::H, 0x4433);
    e.set_memory(0xFF88u16.wrapping_add(0x77), 0x99);
    e.store_sp_plus_immediate(0x77);
    assert_eq!(e.read_register(Register8080::M), 0x99);
}

#[test]
fn add_immediate_to_sp()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x4488);
    e.add_immediate_to_sp(0x22);
    assert_eq!(e.read_register_pair(Register8080::SP), 0x44aa);
    assert!(!e.read_flag(FlagLR35902::Carry));
}

#[test]
fn add_immediate_to_sp_updates_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0xFFFF);
    e.add_immediate_to_sp(0x01);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn add_immediate_to_sp_updates_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x00FF);
    e.add_immediate_to_sp(0x01);
    assert!(e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn double_add_updates_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::H, 0x00FF);
    e.set_register_pair(Register8080::B, 0x0001);
    e.double_add(Register8080::B);
    assert!(e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn double_add_does_not_update_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::H, 0x000F);
    e.set_register_pair(Register8080::B, 0x0001);
    e.double_add(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn store_accumulator_direct_one_byte()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x34);
    e.store_accumulator_direct_one_byte(0x22);
    assert_eq!(e.read_memory(0xFF22), 0x34);
}

#[test]
fn load_accumulator_direct_one_byte()
{
    let mut e = EmulatorLR35902::new();
    e.set_memory(0xFF22, 0x34);
    e.load_accumulator_direct_one_byte(0x22);
    assert_eq!(e.read_register(Register8080::A), 0x34);
}

#[test]
fn return_and_enable_interrupts()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x0400);
    e.return_and_enable_interrupts();
    assert_eq!(e.read_program_counter(), 0x0000);
    assert_eq!(e.read_register_pair(Register8080::SP), 0x0402);
    assert!(e.get_interrupts_enabled());
}

#[test]
fn jump_relative_negative()
{
    let mut e = EmulatorLR35902::new();
    e.set_program_counter(0x1234);
    e.jump_relative(-4i8 as u8);
    assert_eq!(e.read_program_counter(), 0x1230);
}

#[test]
fn jump_relative()
{
    let mut e = EmulatorLR35902::new();
    e.set_program_counter(0x1234);
    e.jump_relative(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Zero, true);
    e.set_program_counter(0x1234);
    e.jump_relative_if_zero(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_not_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_program_counter(0x1234);
    e.jump_relative_if_not_zero(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Carry, true);
    e.set_program_counter(0x1234);
    e.jump_relative_if_carry(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_no_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_program_counter(0x1234);
    e.jump_relative_if_no_carry(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn store_sp_direct()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x9923);
    e.store_sp_direct(0x8833);
    assert_eq!(e.read_memory_u16(0x8833), 0x9923);
}

#[test]
fn store_sp_at_ffff()
{
    let mut e = EmulatorLR35902::new();
    e.set_register_pair(Register8080::SP, 0x9923);
    e.store_sp_direct(0xFFFF);

    // This address is the Interrupt Enable Flag, so this test isn't quite legit.
    assert_eq!(e.read_memory(0xFFFF), 0x99);
}

#[test]
fn reset_bit()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0xFF);
    e.reset_bit(4, Register8080::A);
    e.reset_bit(0, Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b11101110);
}

#[test]
fn set_bit()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0);
    e.set_bit(4, Register8080::A);
    e.set_bit(0, Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b00010001);
}

#[test]
fn test_bit_false()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b00010000);
    e.test_bit(4, Register8080::A);
    assert_eq!(e.read_flag(FlagLR35902::Zero), false);
}

#[test]
fn test_bit_true()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0);
    e.test_bit(4, Register8080::A);
    assert_eq!(e.read_flag(FlagLR35902::Zero), true);
}

#[test]
fn shift_register_right_signed()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.shift_register_right_signed(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b11011101);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn shift_register_right()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.shift_register_right(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b01011101);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn shift_register_left()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.shift_register_left(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b01110110);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn swap_register()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0xF8);
    e.swap_register(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0x8F);
}

#[test]
fn rotate_register_right()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_right(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b11011101);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn rotate_register_left()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_left(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b01110111);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn rotate_register_right_through_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.set_flag(FlagLR35902::Carry, false);
    e.rotate_register_right_through_carry(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b01011101);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn rotate_register_right_through_carry_clears_subtract()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_right_through_carry(Register8080::A);
    assert!(!e.read_flag(FlagLR35902::Subtract));
}

#[test]
fn rotate_register_right_through_carry_clears_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_right_through_carry(Register8080::A);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn rotate_register_left_through_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b10111011);
    e.set_flag(FlagLR35902::Carry, false);
    e.rotate_register_left_through_carry(Register8080::A);
    assert_eq!(e.read_register(Register8080::A), 0b01110110);
    assert!(e.read_flag(FlagLR35902::Carry));
}

#[test]
fn rotate_register_left_through_carry_clears_subtract()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_left_through_carry(Register8080::A);
    assert!(!e.read_flag(FlagLR35902::Subtract));
}

#[test]
fn rotate_register_left_through_carry_clears_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0b10111011);
    e.rotate_register_left_through_carry(Register8080::A);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn logical_and_with_accumulator()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b00000001);
    e.set_register(Register8080::B, 0b11000001);
    e.logical_and_with_accumulator(Register8080::B);
    assert_eq!(e.read_register(Register8080::A), 0b00000001);
}

#[test]
fn logical_and_with_accumulator_sets_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x0);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_and_with_accumulator_clears_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Zero, true);
    e.set_register(Register8080::A, 0b00110001);
    e.set_register(Register8080::B, 0b00010000);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_and_with_accumulator_clears_subtract()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Subtract));
}

#[test]
fn logical_and_with_accumulator_clears_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Carry, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Carry));
}

#[test]
fn logical_and_with_accumulator_sets_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn logical_exclusive_or_with_accumulator()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b00000001);
    e.set_register(Register8080::B, 0b11000001);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert_eq!(e.read_register(Register8080::A), 0b11000000);
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x33);
    e.set_register(Register8080::B, 0x33);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Zero, true);
    e.set_register(Register8080::A, 0b00110001);
    e.set_register(Register8080::B, 0b00010000);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_subtract()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Subtract));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Carry, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Carry));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn logical_or_with_accumulator()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0b00000001);
    e.set_register(Register8080::B, 0b11000001);
    e.logical_or_with_accumulator(Register8080::B);
    assert_eq!(e.read_register(Register8080::A), 0b11000001);
}

#[test]
fn logical_or_with_accumulator_sets_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::A, 0x0);
    e.set_register(Register8080::B, 0x0);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_or_with_accumulator_clears_zero()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Zero, true);
    e.set_register(Register8080::A, 0b00110001);
    e.set_register(Register8080::B, 0b00010000);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Zero));
}

#[test]
fn logical_or_with_accumulator_clears_subtract()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Subtract));
}

#[test]
fn logical_or_with_accumulator_clears_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Carry, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::Carry));
}

#[test]
fn logical_or_with_accumulator_clears_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0x11);
    e.set_register(Register8080::B, 0x22);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn decimal_adjust_accumulator_clears_half_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0x88);
    InstructionSetLR35902::decimal_adjust_accumulator(&mut e);
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

impl<'a> fmt::Debug for EmulatorLR35902<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        try!(writeln!(f, "B: {:x}, C: {:x}, D: {:x}, E: {:x}, H: {:x}, L: {:x}, A: {:x}",
            self.read_register(Register8080::B),
            self.read_register(Register8080::C),
            self.read_register(Register8080::D),
            self.read_register(Register8080::E),
            self.read_register(Register8080::H),
            self.read_register(Register8080::L),
            self.read_register(Register8080::A)));
        try!(writeln!(f, "Zero: {}, Subtract: {}, HalfCarry: {}, Carry: {}",
            self.read_flag(FlagLR35902::Zero),
            self.read_flag(FlagLR35902::Subtract),
            self.read_flag(FlagLR35902::HalfCarry),
            self.read_flag(FlagLR35902::Carry)));
        try!(writeln!(f, "PC: {:x}, SP: {:x}, M: {:x}",
            self.read_program_counter(),
            self.read_register_pair(Register8080::SP),
            self.read_register(Register8080::M)));
        try!(writeln!(f, "PC trail = {}", self.format_pc_trail()));

        let mut buffer = vec![];
        {
            let mut dis = create_disassembler(&self.e8080.main_memory, &mut buffer);
            dis.index = self.read_program_counter() as u64;
            dis.disassemble_one().unwrap();
        }
        try!(write!(f, "{}", str::from_utf8(&buffer).unwrap()));

        Ok(())
    }
}

impl<'a> EmulatorLR35902<'a> {
    fn crash(&mut self, message: String)
    {
        self.crash_message = Some(message);
    }
    fn crashed(&self) -> bool
    {
        self.crash_message.is_some()
    }
    fn run_lr35902_instruction(&mut self, instruction: &[u8])
    {
        let pc = self.read_program_counter() as usize;
        self.set_program_counter((pc + instruction.len()) as u16);
        dispatch_lr35902_instruction(&instruction, self);
    }

    fn run_8080_instruction(&mut self, instruction: &[u8])
    {
        let pc = self.read_program_counter() as usize;
        self.set_program_counter((pc + instruction.len()) as u16);
        dispatch_8080_instruction(&instruction, self);
    }

    fn format_pc_trail(&self) -> String
    {
        let mut message = String::new();
        for pc in &self.pc_trail {
            message.push_str(format!("{:x}, ", pc).as_str());
        }
        return message;
    }

    fn crash_from_unkown_opcode(&mut self)
    {
        let pc = self.read_program_counter();
        let pc_trail = self.format_pc_trail();
        self.crash(format!("Unknown opcode at address {:x}, trail = {}", pc, pc_trail));
    }

    fn run_one_instruction(&mut self)
    {
        self.memory_changed.clear();
        let pc = self.read_program_counter() as usize;
        let mut instr = get_lr35902_instruction(&self.e8080.main_memory[pc..]);
        match instr {
            SomeInstruction(res) => {
                self.run_lr35902_instruction(&res);
                return;
            },
            NoInstruction => { },
            NotImplemented => {
                self.crash_from_unkown_opcode();
                return;
            },
        }
        instr = get_8080_instruction(&self.e8080.main_memory[pc..]);
        match instr {
            SomeInstruction(res) => self.run_8080_instruction(&res),
            _ => self.crash_from_unkown_opcode()
        };
    }

    fn run(&mut self)
    {
        while !self.crashed() {
            self.run_one_instruction();
        }
        println!("Emulator crashed: {}", self.crash_message.as_ref().unwrap());
    }
}

#[test]
fn emulator_crashes_on_unkown_opcode()
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&[0xfc]);
    e.set_program_counter(0);
    e.run();
    assert_eq!(e.crash_message.unwrap(), "Unknown opcode at address 0, trail = 100, 0, ");
}

impl<'a> DebuggerOps for EmulatorLR35902<'a> {
    fn read_address(&self, address: u16) -> u8
    {
        self.read_memory(address)
    }

    fn format<'b>(&self, s: &'b mut io::Write) -> Result<()>
    {
        write!(s, "{:?}", self)
    }

    fn next(&mut self)
    {
        self.run_one_instruction();
    }

    fn current_address(&self) -> u16
    {
        self.read_program_counter()
    }

    fn crashed(&self) -> Option<&String>
    {
        self.crash_message.as_ref()
    }

    fn memory_changed(&self) -> &HashSet<u16>
    {
        &self.memory_changed
    }

    fn set_current_address(&mut self, address: u16)
    {
        self.set_program_counter(address)
    }
}

pub fn run_emulator(rom: &[u8])
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&rom);
    e.run();
}

pub fn run_debugger(rom: &[u8])
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&rom);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let mut stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run();
}

/*  _     _                         _            _
 * | |__ | | __ _ _ __ __ _  __ _  | |_ ___  ___| |_   _ __ ___  _ __ ___  ___
 * | '_ \| |/ _` | '__/ _` |/ _` | | __/ _ \/ __| __| | '__/ _ \| '_ ` _ \/ __|
 * | |_) | | (_| | | | (_| | (_| | | ||  __/\__ \ |_  | | | (_) | | | | | \__ \
 * |_.__/|_|\__,_|_|  \__, |\__, |  \__\___||___/\__| |_|  \___/|_| |_| |_|___/
 *                    |___/ |___/
 */

#[cfg(test)]
fn run_blargg_test_rom_cpu_instrs(name: &str, stop_address: u16)
{
    let mut e = EmulatorLR35902::new();
    let mut rom : Vec<u8> = vec![];
    {
        let mut file = File::open(format!("blargg_test_roms/{}", name)).ok().expect(
            "Did you forget to download the test roms?");
        file.read_to_end(&mut rom).unwrap();
    }
    e.load_rom(&rom);

    let mut pc = e.read_program_counter();
    // This address is where the rom ends.  At this address is an infinite loop where normally the
    // rom will sit at forever.
    while pc != stop_address {
        e.run_one_instruction();
        pc = e.read_program_counter();
    }

    // Scrape from tile memory what is displayed on the screen
    let mut message = String::new();
    let mut c = 0x9800;
    while c < 0x9BFF {
        for i in 0..20 {
            let tile = e.read_memory(c + i);
            // The rom happens to use ASCII as the way it maps characters to the correct tile.
            message.push(tile as char);
        }
        c += 0x20;
        message = String::from(message.trim_right());
        message.push('\n');
    }

    // The message ends with 'Passed' when the test was successful
    assert!(message.ends_with("Passed\n"), "{}", message);
}

// XXX: The following disabled tests are basically a to-do list for fixing / finishing the LR35902
// emulation.

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_1_special()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/01-special.gb", 0xc7d2);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_2_interrupts()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/02-interrupts.gb", 0xc7f4);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_3_op_sp_hl()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/03-op sp,hl.gb", 0xcb44);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_4_op_r_imm()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/04-op r,imm.gb", 0xcb35);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_5_op_rp()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/05-op rp.gb", 0xcb31);
}

#[test]
fn blargg_test_rom_cpu_instrs_6_ld_r_r()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/06-ld r,r.gb", 0xcc5f);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_7_jr_jp_call_ret_rst()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/07-jr,jp,call,ret,rst.gb", 0xcbb0);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_8_misc_instrs()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/08-misc instrs.gb", 0xcb91);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_9_op_r_r()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/09-op r,r.gb", 0xce67);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_10_bit_ops()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/10-bit ops.gb", 0xcf58);
}

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_11_op_a_hl()
{
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/11-op a,(hl).gb", 0xcc62);
}
