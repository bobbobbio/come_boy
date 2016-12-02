mod opcodes;

use std::{fmt, str};
use emulator_common::Register8080;
use emulator_8080::{Emulator8080, InstructionSetOps8080, Flag8080, InstructionSet8080};
pub use emulator_lr35902::opcodes::disassemble_lr35902_rom;
use emulator_lr35902::opcodes::create_disassembler;
use emulator_lr35902::opcodes::{
    get_lr35902_instruction, dispatch_lr35902_instruction, InstructionSetLR35902};

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

#[derive(Debug,Clone,Copy)]
pub enum FlagLR35902 {
                    // 76543210
    Zero =           0b10000000,
    Subtract =       0b01000000,
    HalfCarry =      0b00010000,
    Carry =          0b00000100,
}

struct EmulatorLR35902<'a> {
    e8080: Emulator8080<'a>,
}

impl<'a> EmulatorLR35902<'a> {
    fn new() -> EmulatorLR35902<'a> {
        return EmulatorLR35902 {
            e8080: Emulator8080::new()
        };
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.e8080.main_memory[0..rom.len()].clone_from_slice(rom);
    }

    fn set_flag(&mut self, flag: FlagLR35902, value: bool)
    {
        self.e8080.set_flag_u8(flag as u8, value);
    }

    fn read_flag(&self, flag: FlagLR35902) -> bool
    {
        self.e8080.read_flag_u8(flag as u8)
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
    }

    fn read_memory_u16(&self, address: u16) -> u16
    {
        self.e8080.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16)
    {
        self.e8080.set_memory_u16(address, value)
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

    fn add_to_register_pair(&mut self, register: Register8080, value: u16, update_carry: bool)
    {
        let old_value = self.read_register_pair(register);
        let new_value = old_value.wrapping_add(value);
        self.set_register_pair(register, new_value);
        if update_carry {
            self.set_flag(FlagLR35902::Carry, value > (0xFFFF - old_value));
            self.set_flag(FlagLR35902::HalfCarry, value & 0x00FF > 0x00FF - (old_value & 0x00FF));
        }
    }

    fn read_program_counter(&self) -> u16
    {
        self.e8080.read_program_counter()
    }

    fn set_program_counter(&mut self, address: u16)
    {
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
    fn move_and_increment_m(&mut self, dest_register: Register8080, src_register: Register8080)
    {
        self.move_data(dest_register, src_register);
        self.add_to_register(Register8080::M, 1, false /* update carry */);
    }

    fn move_and_decrement_m(&mut self, dest_register: Register8080, src_register: Register8080)
    {
        self.move_data(dest_register, src_register);
        self.subtract_from_register(Register8080::M, 1);
    }

    fn store_accumulator_direct_two_bytes(&mut self, address: u16)
    {
        self.store_accumulator_direct(address);
    }

    fn store_sp_plus_immediate(&mut self, data: u8)
    {
        let address = self.read_register_pair(Register8080::SP) + data as u16;
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
        let pc = self.read_program_counter();
        self.jump(pc.wrapping_add(n as u16));
    }

    fn jump_relative_if_zero(&mut self, n: u8)
    {
        let pc = self.read_program_counter();
        self.jump_if_zero(pc.wrapping_add(n as u16));
    }

    fn jump_relative_if_not_zero(&mut self, n: u8)
    {
        let pc = self.read_program_counter();
        self.jump_if_not_zero(pc.wrapping_add(n as u16));
    }

    fn jump_relative_if_carry(&mut self, n: u8)
    {
        let pc = self.read_program_counter();
        self.jump_if_carry(pc.wrapping_add(n as u16));
    }

    fn jump_relative_if_no_carry(&mut self, n: u8)
    {
        let pc = self.read_program_counter();
        self.jump_if_no_carry(pc.wrapping_add(n as u16));
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
        self.set_register(register, value.rotate_right(1));
        self.set_flag(FlagLR35902::Carry, (value & 1) != 0);
    }

    fn rotate_register_left(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.set_register(register, value.rotate_left(1));
        self.set_flag(FlagLR35902::Carry, (value & (1u8 << 7)) != 0);
    }

    fn rotate_register_right_through_carry(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let carry = if self.read_flag(FlagLR35902::Carry) { 1 } else { 0 };
        let res = (value >> 1) | (carry << 7);
        self.set_register(register, res);
        self.set_flag(FlagLR35902::Carry, (value & 1) != 0);
        self.set_flag(FlagLR35902::Zero, res == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }

    fn rotate_register_left_through_carry(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        let carry = if self.read_flag(FlagLR35902::Carry) { 1 } else { 0 };
        let res = (value << 1) | carry;
        self.set_register(register, res);
        self.set_flag(FlagLR35902::Carry, (value & (1u8 << 7)) != 0);
        self.set_flag(FlagLR35902::Zero, res == 0);
        self.set_flag(FlagLR35902::Subtract, false);
        self.set_flag(FlagLR35902::HalfCarry, false);
    }
}

#[test]
fn move_and_increment_m()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::M, 0x99);
    e.move_and_increment_m(Register8080::A, Register8080::M);
    assert_eq!(e.read_register(Register8080::A), 0x99);
    assert_eq!(e.read_register(Register8080::M), 0x9a);
}

#[test]
fn move_and_decrement_m()
{
    let mut e = EmulatorLR35902::new();
    e.set_register(Register8080::M, 0x99);
    e.move_and_decrement_m(Register8080::A, Register8080::M);
    assert_eq!(e.read_register(Register8080::A), 0x99);
    assert_eq!(e.read_register(Register8080::M), 0x98);
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
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0b10111011);
    e.set_flag(FlagLR35902::Carry, false);
    e.rotate_register_right_through_carry(Register8080::A);

    assert_eq!(e.read_register(Register8080::A), 0b01011101);
    assert!(e.read_flag(FlagLR35902::Carry));
    assert!(!e.read_flag(FlagLR35902::Subtract));
    assert!(!e.read_flag(FlagLR35902::HalfCarry));
}

#[test]
fn rotate_register_left_through_carry()
{
    let mut e = EmulatorLR35902::new();
    e.set_flag(FlagLR35902::Subtract, true);
    e.set_flag(FlagLR35902::HalfCarry, true);
    e.set_register(Register8080::A, 0b10111011);
    e.set_flag(FlagLR35902::Carry, false);
    e.rotate_register_left_through_carry(Register8080::A);

    assert_eq!(e.read_register(Register8080::A), 0b01110110);
    assert!(e.read_flag(FlagLR35902::Carry));
    assert!(!e.read_flag(FlagLR35902::Subtract));
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
    fn run_one_instruction(&mut self) -> bool
    {
        let pc = self.read_program_counter() as usize;
        let instruction = match get_lr35902_instruction(&self.e8080.main_memory[pc..]) {
            Some(res) => res,
            None => { return false; }
        };

        self.set_program_counter((pc + instruction.len()) as u16);

        dispatch_lr35902_instruction(&instruction, self);

        return true;
    }

    fn run(&mut self)
    {
        self.set_register_pair(Register8080::SP, 0xFFFE);
        self.set_program_counter(ROM_ADDRESS as u16);
        while self.read_program_counter() != 0 {
            println!("{:?}", self);

            if !self.run_one_instruction() {
                self.e8080.run_one_instruction();
            }
        }
    }
}

pub fn run_emulator(rom: &[u8])
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&rom);
    e.run();
}
