// Copyright 2017 Remi Bernotavicius

extern crate sdl2;

pub mod debugger;
mod opcodes;

use std::mem;

pub use emulator_common::disassembler::{
    MemoryAccessor, MemoryIterator, MemoryStream, SimpleMemoryAccessor,
};
pub use emulator_common::Intel8080Register;
use intel_8080_emulator::{Intel8080Flag, Intel8080InstructionSet, Intel8080InstructionSetOps};
pub use lr35902_emulator::debugger::run_debugger;
pub use lr35902_emulator::opcodes::disassemble_lr35902_rom;
pub use lr35902_emulator::opcodes::{
    dispatch_lr35902_instruction, get_lr35902_instruction, LR35902InstructionSet,
};
use util::TwosComplement;

#[cfg(test)]
use std::fs::File;

#[cfg(test)]
use std::io::Read;

/*  _     ____  _________  ___   ___ ____  _____                 _       _
 * | |   |  _ \|___ / ___|/ _ \ / _ \___ \| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * | |   | |_) | |_ \___ \ (_) | | | |__) |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |___|  _ < ___) |__) \__, | |_| / __/| |___| | | | | | |_| | | (_| | || (_) | |
 * |_____|_| \_\____/____/  /_/ \___/_____|_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 */

const ROM_ADDRESS: usize = 0x0100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LR35902Flag {
    // 76543210
    Zero = 0b10000000,
    Subtract = 0b01000000,
    HalfCarry = 0b00100000,
    Carry = 0b00010000,

    ValidityMask = 0b11110000,
}

pub struct LR35902Emulator<M: MemoryAccessor> {
    pub memory_accessor: M,
    registers: [u8; Intel8080Register::Count as usize],
    program_counter: u16,
    interrupts_enabled: bool,
    pub elapsed_cycles: u64,
    pub crash_message: Option<String>,
}

impl<M: MemoryAccessor> LR35902Emulator<M> {
    pub fn new(memory_accessor: M) -> LR35902Emulator<M> {
        let mut e = LR35902Emulator {
            memory_accessor: memory_accessor,
            registers: [0; Intel8080Register::Count as usize],
            program_counter: 0,
            interrupts_enabled: true,
            elapsed_cycles: 102348,
            crash_message: None,
        };

        e.set_register_pair(Intel8080Register::SP, 0xFFFE);
        e.set_program_counter(ROM_ADDRESS as u16);

        return e;
    }

    pub fn set_flag(&mut self, flag: LR35902Flag, value: bool) {
        if value {
            self.registers[Intel8080Register::FLAGS as usize] |= flag as u8;
        } else {
            self.registers[Intel8080Register::FLAGS as usize] &= !(flag as u8);
        }
    }

    pub fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.registers[Intel8080Register::FLAGS as usize] & flag as u8 == flag as u8
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory_accessor.read_memory(address)
    }

    pub fn read_register(&self, register: Intel8080Register) -> u8 {
        Intel8080InstructionSetOps::read_register(self, register)
    }

    pub fn set_register(&mut self, register: Intel8080Register, value: u8) {
        Intel8080InstructionSetOps::set_register(self, register, value);
    }

    pub fn read_register_pair(&self, register: Intel8080Register) -> u16 {
        Intel8080InstructionSetOps::read_register_pair(self, register)
    }

    pub fn set_register_pair(&mut self, register: Intel8080Register, value: u16) {
        Intel8080InstructionSetOps::set_register_pair(self, register, value);
    }

    pub fn set_memory(&mut self, address: u16, value: u8) {
        self.memory_accessor.set_memory(address, value);
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        self.memory_accessor.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.memory_accessor.set_memory_u16(address, value);
    }

    fn read_raw_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    fn set_raw_register(&mut self, index: usize, value: u8) {
        assert!(index != Intel8080Register::FLAGS as usize);
        self.registers[index] = value;
    }

    fn read_raw_register_pair(&self, index: usize) -> u16 {
        let register_pairs: &[u16; Intel8080Register::Count as usize / 2];
        unsafe {
            register_pairs = mem::transmute(&self.registers);
        }

        register_pairs[index]
    }

    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        let register_pairs: &mut [u16; Intel8080Register::Count as usize / 2];
        unsafe {
            register_pairs = mem::transmute(&mut self.registers);
        }
        register_pairs[index] = value;
        if index == Intel8080Register::A as usize / 2 {
            // If we are setting the FLAGS register, we need to force the zero flags to be zero.
            self.registers[Intel8080Register::FLAGS as usize] &= LR35902Flag::ValidityMask as u8;
        }
    }

    pub fn read_program_counter(&self) -> u16 {
        self.program_counter
    }

    fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn set_interrupts_enabled(&mut self, value: bool) {
        self.interrupts_enabled = value;
    }

    pub fn get_interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    pub fn interrupt(&mut self, address: u16) {
        assert!(self.interrupts_enabled);
        self.interrupts_enabled = false;
        Intel8080InstructionSet::call(self, address);
    }

    fn add_cycles(&mut self, cycles: u8) {
        self.elapsed_cycles += cycles as u64;
    }
}

#[cfg(test)]
fn new_lr35902_emulator_for_test() -> LR35902Emulator<SimpleMemoryAccessor> {
    return LR35902Emulator::<SimpleMemoryAccessor>::new(SimpleMemoryAccessor::new());
}

/*   ___
 *  / _ \ _ __  ___
 * | | | | '_ \/ __|
 * | |_| | |_) \__ \
 *  \___/| .__/|___/
 *       |_|
 */

pub trait LR35902InstructionSetOps {
    fn set_flag(&mut self, flag: LR35902Flag, value: bool);
    fn read_flag(&self, flag: LR35902Flag) -> bool;
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);
    fn read_memory_u16(&self, address: u16) -> u16;
    fn set_memory_u16(&mut self, address: u16, value: u16);
    fn read_raw_register(&self, index: usize) -> u8;
    fn set_raw_register(&mut self, index: usize, value: u8);
    fn read_raw_register_pair(&self, index: usize) -> u16;
    fn set_raw_register_pair(&mut self, index: usize, value: u16);
    fn read_program_counter(&self) -> u16;
    fn set_program_counter(&mut self, address: u16);
    fn set_interrupts_enabled(&mut self, value: bool);
    fn get_interrupts_enabled(&self) -> bool;
    fn add_cycles(&mut self, cycles: u8);

    fn get_relative_address(&self, n: u8) -> u16 {
        self.read_program_counter()
            .wrapping_add(((n as i8) as i16) as u16)
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8 {
        let new_value = value_a.wrapping_add(value_b);

        self.set_flag(LR35902Flag::Zero, new_value == 0);
        if update_carry {
            self.set_flag(LR35902Flag::Carry, value_b > 0xFF - value_a);
        }
        self.set_flag(
            LR35902Flag::HalfCarry,
            value_b & 0x0F > 0x0F - (value_a & 0x0F),
        );
        self.set_flag(LR35902Flag::Subtract, false);

        return new_value;
    }

    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, ovalue_b: u8) -> u8 {
        let value_b = ovalue_b.twos_complement();
        let new_value = value_a.wrapping_add(value_b);

        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, true);
        self.set_flag(LR35902Flag::HalfCarry, new_value & 0x0F > (value_a & 0x0F));
        self.set_flag(LR35902Flag::Carry, value_a < ovalue_b);

        return new_value;
    }

    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a.wrapping_sub(value_b);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::HalfCarry, value_b & 0x0F > (value_a & 0x0F));
        self.set_flag(LR35902Flag::Subtract, true);
        return new_value;
    }

    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a & value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, true);
        return new_value;
    }

    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a ^ value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        return new_value;
    }

    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a | value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        return new_value;
    }

    fn perform_signed_double_add(&mut self, value_a: u16, value_b: u8) -> u16 {
        let value = ((value_b as i8) as i16) as u16;
        let new_value = value_a.wrapping_add(value);

        self.set_flag(
            LR35902Flag::Carry,
            value & 0x00FF > (0x00FF - (value_a & 0x00FF)),
        );
        self.set_flag(
            LR35902Flag::HalfCarry,
            value & 0x000F > 0x000F - (value_a & 0x000F),
        );
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Zero, false);

        return new_value;
    }
}

impl<M: MemoryAccessor> LR35902InstructionSetOps for LR35902Emulator<M> {
    fn set_flag(&mut self, flag: LR35902Flag, value: bool) {
        self.set_flag(flag, value);
    }

    fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.read_flag(flag)
    }

    fn read_memory(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        self.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory_u16(address, value);
    }

    fn read_raw_register(&self, index: usize) -> u8 {
        self.read_raw_register(index)
    }

    fn set_raw_register(&mut self, index: usize, value: u8) {
        self.set_raw_register(index, value);
    }

    fn read_raw_register_pair(&self, index: usize) -> u16 {
        self.read_raw_register_pair(index)
    }

    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        self.set_raw_register_pair(index, value);
    }

    fn read_program_counter(&self) -> u16 {
        self.read_program_counter()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.set_program_counter(address);
    }

    fn set_interrupts_enabled(&mut self, value: bool) {
        self.set_interrupts_enabled(value);
    }

    fn get_interrupts_enabled(&self) -> bool {
        self.get_interrupts_enabled()
    }

    fn add_cycles(&mut self, cycles: u8) {
        self.add_cycles(cycles);
    }
}

/*   ___   ___   ___   ___    _          _     ____  _________  ___   ___ ____
 *  ( _ ) / _ \ ( _ ) / _ \  | |_ ___   | |   |  _ \|___ / ___|/ _ \ / _ \___ \
 *  / _ \| | | |/ _ \| | | | | __/ _ \  | |   | |_) | |_ \___ \ (_) | | | |__) |
 * | (_) | |_| | (_) | |_| | | || (_) | | |___|  _ < ___) |__) \__, | |_| / __/
 *  \___/ \___/ \___/ \___/   \__\___/  |_____|_| \_\____/____/  /_/ \___/_____|
 *
 */

impl<I: LR35902InstructionSetOps> Intel8080InstructionSetOps for I {
    /*
     * Implementing this trait is the translation layer that allows 8080 instructions to be run on
     * the LR35902.
     */
    fn read_memory(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        self.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory_u16(address, value);
    }

    fn set_flag(&mut self, flag: Intel8080Flag, value: bool) {
        match flag {
            Intel8080Flag::Zero => self.set_flag(LR35902Flag::Zero, value),
            Intel8080Flag::AuxiliaryCarry => self.set_flag(LR35902Flag::HalfCarry, value),
            Intel8080Flag::Carry => self.set_flag(LR35902Flag::Carry, value),
            _ => {}
        };
    }

    fn read_flag(&self, flag: Intel8080Flag) -> bool {
        match flag {
            Intel8080Flag::Zero => self.read_flag(LR35902Flag::Zero),
            Intel8080Flag::AuxiliaryCarry => self.read_flag(LR35902Flag::HalfCarry),
            Intel8080Flag::Carry => self.read_flag(LR35902Flag::Carry),
            flag => panic!("LR35902 doesn't know about {:?}", flag),
        }
    }

    fn read_raw_register(&self, index: usize) -> u8 {
        self.read_raw_register(index)
    }

    fn set_raw_register(&mut self, index: usize, value: u8) {
        self.set_raw_register(index, value);
    }

    fn read_raw_register_pair(&self, index: usize) -> u16 {
        self.read_raw_register_pair(index)
    }

    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        self.set_raw_register_pair(index, value);
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8 {
        self.perform_addition(value_a, value_b, update_carry)
    }

    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_subtraction_using_twos_complement(value_a, value_b)
    }

    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_subtraction(value_a, value_b)
    }

    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_and(value_a, value_b)
    }

    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_exclusive_or(value_a, value_b)
    }

    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_or(value_a, value_b)
    }

    fn read_program_counter(&self) -> u16 {
        self.read_program_counter()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.set_program_counter(address);
    }

    fn set_interrupts_enabled(&mut self, value: bool) {
        self.set_interrupts_enabled(value);
    }

    fn get_interrupts_enabled(&self) -> bool {
        self.get_interrupts_enabled()
    }

    fn add_cycles(&mut self, cycles: u8) {
        self.add_cycles(cycles);
    }
}

/*   ___              _____         _
 *  / _ \ _ __  ___  |_   _|__  ___| |_ ___
 * | | | | '_ \/ __|   | |/ _ \/ __| __/ __|
 * | |_| | |_) \__ \   | |  __/\__ \ |_\__ \
 *  \___/| .__/|___/   |_|\___||___/\__|___/
 *       |_|
 *
 */

#[test]
fn can_set_and_read_memory() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_memory(0x1122, 0x88);
    assert_eq!(e.read_memory(0x1122), 0x88);
}

#[test]
fn can_set_and_read_memory_16_bit() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_memory_u16(0x1122, 0x2233);
    assert_eq!(e.read_memory_u16(0x1122), 0x2233);
}

#[test]
fn can_set_and_read_regiser() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x45);
    assert_eq!(e.read_register(Intel8080Register::A), 0x45);
}

#[test]
fn can_set_and_read_regiser_pair() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::B, 0x4523);
    assert_eq!(e.read_register_pair(Intel8080Register::B), 0x4523);
}

#[test]
fn perform_addition() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    assert_eq!(
        e.perform_addition(0x33, 0x11, false /* update carry */),
        0x44
    );
}

#[test]
fn perform_addition_with_overflow() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    assert_eq!(
        e.perform_addition(0xF3, 0x11, false /* update carry */),
        0x04
    );
}

#[test]
fn perform_addition_sets_zero_flag() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_addition(0xF3, 0x0D, false /* update carry */);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn perform_addition_sets_half_carry() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_addition(0x0F, 0x01, false /* update carry */);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn perform_addition_clears_subtract_flag() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.perform_addition(0x0D, 0x01, false /* update carry */);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn perform_addition_does_not_set_carry() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_addition(0xFF, 0x01, false /* update carry */);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn perform_addition_clears_carry() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Carry, true);
    e.perform_addition(0xF1, 0x01, true /* update carry */);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn perform_addition_sets_carry() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_addition(0xFF, 0x01, true /* update carry */);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn perform_subtraction() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    assert_eq!(e.perform_subtraction(0x12, 0x11), 0x01);
}

#[test]
fn perform_subtraction_with_underflow() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    assert_eq!(e.perform_subtraction(0x12, 0x13), 0xFF);
}

#[test]
fn perform_subtraction_sets_zero_flag() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_subtraction(0x12, 0x12);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn perform_subtraction_sets_subtract_flag() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_subtraction(0x12, 0x04);
    assert!(e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn perform_subtraction_sets_half_carry_flag() {
    let e: &mut LR35902InstructionSetOps = &mut new_lr35902_emulator_for_test();
    e.perform_subtraction(0x03, 0x04);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
}

/*  ___           _                   _   _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __  ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
 *
 */

impl<I: LR35902InstructionSetOps> LR35902InstructionSet for I {
    fn move_and_increment_hl(
        &mut self,
        dest_register: Intel8080Register,
        src_register: Intel8080Register,
    ) {
        LR35902InstructionSet::move_data(self, dest_register, src_register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        self.set_register_pair(Intel8080Register::H, old_value.wrapping_add(1));
    }

    fn move_and_decrement_hl(
        &mut self,
        dest_register: Intel8080Register,
        src_register: Intel8080Register,
    ) {
        LR35902InstructionSet::move_data(self, dest_register, src_register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        self.set_register_pair(Intel8080Register::H, old_value.wrapping_sub(1));
    }

    fn store_accumulator_direct(&mut self, address: u16) {
        Intel8080InstructionSet::store_accumulator_direct(self, address);
    }

    fn store_sp_plus_immediate(&mut self, data: u8) {
        let sp = self.read_register_pair(Intel8080Register::SP);
        let address = self.perform_signed_double_add(sp, data);
        self.set_register_pair(Intel8080Register::H, address);
    }

    fn add_immediate_to_sp(&mut self, data: u8) {
        let sp = self.read_register_pair(Intel8080Register::SP);
        let new_value = self.perform_signed_double_add(sp, data);
        self.set_register_pair(Intel8080Register::SP, new_value);
    }

    fn double_add(&mut self, register: Intel8080Register) {
        let value = self.read_register_pair(register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        let new_value = old_value.wrapping_add(value);

        self.set_flag(LR35902Flag::Carry, value > (0xFFFF - old_value));
        self.set_flag(
            LR35902Flag::HalfCarry,
            value & 0x0FFF > 0x0FFF - (old_value & 0x0FFF),
        );
        self.set_flag(LR35902Flag::Subtract, false);

        self.set_register_pair(Intel8080Register::H, new_value);
    }

    fn store_accumulator_direct_one_byte(&mut self, relative_address: u8) {
        let value = self.read_register(Intel8080Register::A);
        self.set_memory(0xFF00 + relative_address as u16, value);
    }

    fn store_accumulator_one_byte(&mut self) {
        let relative_address = self.read_register(Intel8080Register::C);
        self.store_accumulator_direct_one_byte(relative_address);
    }

    fn load_accumulator_direct_one_byte(&mut self, relative_address: u8) {
        let value = self.read_memory(0xFF00 + relative_address as u16);
        self.set_register(Intel8080Register::A, value);
    }

    fn load_accumulator_one_byte(&mut self) {
        let relative_address = self.read_register(Intel8080Register::C);
        self.load_accumulator_direct_one_byte(relative_address);
    }

    fn return_and_enable_interrupts(&mut self) {
        LR35902InstructionSet::return_unconditionally(self);
        LR35902InstructionSet::enable_interrupts(self);
    }

    fn halt_until_button_press(&mut self) {
        unimplemented!();
    }

    fn jump_relative(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump(self, address);
    }

    fn jump_relative_if_zero(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_zero(self, address);
    }

    fn jump_relative_if_not_zero(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_not_zero(self, address);
    }

    fn jump_relative_if_carry(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_carry(self, address);
    }

    fn jump_relative_if_no_carry(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_no_carry(self, address);
    }

    fn store_sp_direct(&mut self, address: u16) {
        let value = self.read_register_pair(Intel8080Register::SP);
        self.set_memory_u16(address, value);
    }

    fn load_accumulator_direct(&mut self, address: u16) {
        Intel8080InstructionSet::load_accumulator_direct(self, address);
    }

    fn reset_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value & !(1u8 << bit));
    }

    fn set_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value | (1u8 << bit));
    }

    fn test_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_flag(LR35902Flag::Zero, (value & (1u8 << bit)) == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, true);
    }

    fn shift_register_right_signed(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = ((value as i8) >> 1) as u8;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & 1) != 0);
    }

    fn shift_register_right(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = value >> 1;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & 1) != 0);
    }

    fn shift_register_left(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = value << 1;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & (1u8 << 7)) != 0);
    }

    fn swap_register(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = (value << 4) | (value >> 4);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, false);
    }

    fn rotate_register_right(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn rotate_register_left(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn rotate_register_right_through_carry(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn rotate_register_left_through_carry(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn rotate_accumulator_right(&mut self) {
        self.rotate_register_right(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    fn rotate_accumulator_left(&mut self) {
        self.rotate_register_left(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    fn rotate_accumulator_right_through_carry(&mut self) {
        self.rotate_register_right_through_carry(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    fn rotate_accumulator_left_through_carry(&mut self) {
        self.rotate_register_left_through_carry(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    fn decimal_adjust_accumulator(&mut self) {
        if !self.read_flag(LR35902Flag::Subtract) {
            Intel8080InstructionSet::decimal_adjust_accumulator(self);
        } else {
            let value = if self.read_flag(LR35902Flag::Carry) {
                0x60
            } else {
                0x0
            } | if self.read_flag(LR35902Flag::HalfCarry) {
                0x06
            } else {
                0x0
            };
            let accumulator = self.read_register(Intel8080Register::A).wrapping_sub(value);
            self.set_register(Intel8080Register::A, accumulator);

            self.set_flag(LR35902Flag::Carry, value & 0x60 != 0);
            self.set_flag(LR35902Flag::Zero, accumulator == 0);
        }
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn complement_accumulator(&mut self) {
        Intel8080InstructionSet::complement_accumulator(self);
        self.set_flag(LR35902Flag::Subtract, true);
        self.set_flag(LR35902Flag::HalfCarry, true);
    }

    fn set_carry(&mut self) {
        Intel8080InstructionSet::set_carry(self);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn complement_carry(&mut self) {
        Intel8080InstructionSet::complement_carry(self);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn load_sp_from_h_and_l(&mut self) {
        Intel8080InstructionSet::load_sp_from_h_and_l(self)
    }

    fn or_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::or_immediate_with_accumulator(self, data)
    }

    fn no_operation(&mut self) {
        Intel8080InstructionSet::no_operation(self)
    }

    fn load_register_pair_immediate(&mut self, register: Intel8080Register, data: u16) {
        Intel8080InstructionSet::load_register_pair_immediate(self, register, data)
    }

    fn move_data(&mut self, dest_register: Intel8080Register, src_register: Intel8080Register) {
        Intel8080InstructionSet::move_data(self, dest_register, src_register)
    }

    fn enable_interrupts(&mut self) {
        Intel8080InstructionSet::enable_interrupts(self)
    }

    fn return_if_zero(&mut self) {
        Intel8080InstructionSet::return_if_zero(self)
    }

    fn exclusive_or_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::exclusive_or_immediate_with_accumulator(self, data)
    }

    fn and_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::and_immediate_with_accumulator(self, data)
    }

    fn decrement_register_or_memory(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::decrement_register_or_memory(self, register)
    }

    fn halt(&mut self) {
        Intel8080InstructionSet::halt(self)
    }

    fn compare_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::compare_with_accumulator(self, register)
    }

    fn restart(&mut self, implicit_data: u8) {
        Intel8080InstructionSet::restart(self, implicit_data)
    }

    fn decrement_register_pair(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::decrement_register_pair(self, register)
    }

    fn return_if_not_zero(&mut self) {
        Intel8080InstructionSet::return_if_not_zero(self)
    }

    fn logical_or_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_or_with_accumulator(self, register)
    }

    fn jump(&mut self, address: u16) {
        Intel8080InstructionSet::jump(self, address)
    }

    fn call_if_not_zero(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_not_zero(self, address)
    }

    fn subtract_immediate_from_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::subtract_immediate_from_accumulator(self, data)
    }

    fn subtract_from_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::subtract_from_accumulator(self, register)
    }

    fn load_accumulator(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::load_accumulator(self, register_pair)
    }

    fn return_unconditionally(&mut self) {
        Intel8080InstructionSet::return_unconditionally(self)
    }

    fn jump_if_not_zero(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_not_zero(self, address)
    }

    fn call_if_carry(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_carry(self, address)
    }

    fn logical_and_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_and_with_accumulator(self, register)
    }

    fn jump_if_no_carry(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_no_carry(self, address)
    }

    fn call(&mut self, address: u16) {
        Intel8080InstructionSet::call(self, address)
    }

    fn return_if_no_carry(&mut self) {
        Intel8080InstructionSet::return_if_no_carry(self)
    }

    fn call_if_zero(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_zero(self, address)
    }

    fn jump_if_carry(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_carry(self, address)
    }

    fn add_immediate_to_accumulator_with_carry(&mut self, data: u8) {
        Intel8080InstructionSet::add_immediate_to_accumulator_with_carry(self, data);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    fn increment_register_pair(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::increment_register_pair(self, register)
    }

    fn store_accumulator(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::store_accumulator(self, register_pair)
    }

    fn add_to_accumulator_with_carry(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::add_to_accumulator_with_carry(self, register)
    }

    fn subtract_from_accumulator_with_borrow(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::subtract_from_accumulator_with_borrow(self, register)
    }

    fn push_data_onto_stack(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::push_data_onto_stack(self, register_pair)
    }

    fn increment_register_or_memory(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::increment_register_or_memory(self, register)
    }

    fn load_program_counter(&mut self) {
        Intel8080InstructionSet::load_program_counter(self)
    }

    fn pop_data_off_stack(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::pop_data_off_stack(self, register_pair)
    }

    fn add_immediate_to_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::add_immediate_to_accumulator(self, data)
    }

    fn logical_exclusive_or_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_exclusive_or_with_accumulator(self, register)
    }

    fn add_to_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::add_to_accumulator(self, register)
    }

    fn disable_interrupts(&mut self) {
        Intel8080InstructionSet::disable_interrupts(self)
    }

    fn compare_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::compare_immediate_with_accumulator(self, data)
    }

    fn move_immediate_data(&mut self, dest_register: Intel8080Register, data: u8) {
        Intel8080InstructionSet::move_immediate_data(self, dest_register, data)
    }

    fn call_if_no_carry(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_no_carry(self, address)
    }

    fn return_if_carry(&mut self) {
        Intel8080InstructionSet::return_if_carry(self)
    }

    fn jump_if_zero(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_zero(self, address)
    }

    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data: u8) {
        Intel8080InstructionSet::subtract_immediate_from_accumulator_with_borrow(self, data);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }
}

/*  ___           _                   _   _               _____         _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __   |_   _|__  ___| |_ ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \    | |/ _ \/ __| __/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | |   | |  __/\__ \ |_\__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|   |_|\___||___/\__|___/
 *
 */

#[test]
fn move_and_increment_hl() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x1122);
    e.set_register(Intel8080Register::M, 0x99);
    e.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M);
    assert_eq!(e.read_register(Intel8080Register::A), 0x99);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1123);
}

#[test]
fn move_and_increment_hl_overflows() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFFFF);
    e.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x0);
}

#[test]
fn move_and_decrement_hl() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x1122);
    e.set_register(Intel8080Register::M, 0x99);
    e.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M);
    assert_eq!(e.read_register(Intel8080Register::A), 0x99);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1121);
}

#[test]
fn move_and_decrement_hl_underflows() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x0);
    e.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0xFFFF);
}

#[test]
fn store_accumulator_direct() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x44);
    LR35902InstructionSet::store_accumulator_direct(&mut e, 0x5588);
    assert_eq!(e.read_memory(0x5588), 0x44);
}

#[test]
fn store_sp_plus_immediate() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x4488);
    e.store_sp_plus_immediate(0x77);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x4488 + 0x77);
}

#[test]
fn store_sp_plus_immediate_with_overflow() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0xFF88);
    e.store_sp_plus_immediate(0x77);
    assert_eq!(
        e.read_register_pair(Intel8080Register::H),
        0xFF88u16.wrapping_add(0x77)
    );
}

#[test]
fn add_immediate_to_sp() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x4488);
    e.add_immediate_to_sp(0x22);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x44aa);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example1() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0xFFFF);
    e.add_immediate_to_sp(0x01);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example2() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x00FF);
    e.add_immediate_to_sp(0x01);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example3() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x00F0);
    e.add_immediate_to_sp(0x10);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example4() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0xFFFF);
    e.add_immediate_to_sp(0x84);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xFF83);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example5() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x000F);
    e.add_immediate_to_sp(0x01);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0010);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn add_immediate_to_sp_example6() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x0000);
    e.add_immediate_to_sp(0x90);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xFF90);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn subtract_immediate_from_accumulator_example1() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0xFF);
    LR35902InstructionSet::subtract_immediate_from_accumulator(&mut e, 0x01);
    assert_eq!(e.read_register(Intel8080Register::A), 0xFE);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn subtract_immediate_from_accumulator_example2() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x04);
    LR35902InstructionSet::subtract_immediate_from_accumulator(&mut e, 0x05);
    assert_eq!(e.read_register(Intel8080Register::A), 0xFF);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn subtract_immediate_from_accumulator_example3() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x14);
    LR35902InstructionSet::subtract_immediate_from_accumulator(&mut e, 0x05);
    assert_eq!(e.read_register(Intel8080Register::A), 0x0F);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn subtract_immediate_from_accumulator_example4() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x14);
    LR35902InstructionSet::subtract_immediate_from_accumulator(&mut e, 0x86);
    assert_eq!(e.read_register(Intel8080Register::A), 0x8E);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn double_add_updates_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x0FFF);
    e.set_register_pair(Intel8080Register::B, 0x0001);
    LR35902InstructionSet::double_add(&mut e, Intel8080Register::B);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn double_add_does_not_update_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x00FF);
    e.set_register_pair(Intel8080Register::B, 0x0001);
    LR35902InstructionSet::double_add(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn double_add_adds() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::H, 0x000F);
    e.set_register_pair(Intel8080Register::B, 0x0001);
    LR35902InstructionSet::double_add(&mut e, Intel8080Register::B);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x0010);
}

#[test]
fn store_accumulator_direct_one_byte() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x34);
    e.store_accumulator_direct_one_byte(0x22);
    assert_eq!(e.read_memory(0xFF22), 0x34);
}

#[test]
fn load_accumulator_direct_one_byte() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_memory(0xFF22, 0x34);
    e.load_accumulator_direct_one_byte(0x22);
    assert_eq!(e.read_register(Intel8080Register::A), 0x34);
}

#[test]
fn store_accumulator_one_byte() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x34);
    e.set_register(Intel8080Register::C, 0x22);
    e.store_accumulator_one_byte();
    assert_eq!(e.read_memory(0xFF22), 0x34);
}

#[test]
fn load_accumulator_one_byte() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_memory(0xFF22, 0x34);
    e.set_register(Intel8080Register::C, 0x22);
    e.load_accumulator_one_byte();
    assert_eq!(e.read_register(Intel8080Register::A), 0x34);
}

#[test]
fn return_and_enable_interrupts() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x0400);
    e.return_and_enable_interrupts();
    assert_eq!(e.read_program_counter(), 0x0000);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0402);
    assert!(e.get_interrupts_enabled());
}

#[test]
fn jump_relative_negative() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_program_counter(0x1234);
    e.jump_relative(-4i8 as u8);
    assert_eq!(e.read_program_counter(), 0x1230);
}

#[test]
fn jump_relative_example() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_program_counter(0x297);
    e.jump_relative(0xFC);
    assert_eq!(e.read_program_counter(), 0x293);
}

#[test]
fn jump_relative() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_program_counter(0x1234);
    e.jump_relative(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Zero, true);
    e.set_program_counter(0x1234);
    e.jump_relative_if_zero(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_not_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_program_counter(0x1234);
    e.jump_relative_if_not_zero(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Carry, true);
    e.set_program_counter(0x1234);
    e.jump_relative_if_carry(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn jump_relative_if_no_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_program_counter(0x1234);
    e.jump_relative_if_no_carry(0x11);
    assert_eq!(e.read_program_counter(), 0x1245);
}

#[test]
fn store_sp_direct() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x9923);
    e.store_sp_direct(0x8833);
    assert_eq!(e.read_memory_u16(0x8833), 0x9923);
}

#[test]
fn store_sp_at_ffff() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x9923);
    e.store_sp_direct(0xFFFF);

    // This address is the Interrupt Enable Flag, so this test isn't quite legit.
    assert_eq!(e.read_memory(0xFFFF), 0x23);
}

#[test]
fn reset_bit() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0xFF);
    e.reset_bit(4, Intel8080Register::A);
    e.reset_bit(0, Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b11101110);
}

#[test]
fn set_bit() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0);
    e.set_bit(4, Intel8080Register::A);
    e.set_bit(0, Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b00010001);
}

#[test]
fn test_bit_false() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b00010000);
    e.test_bit(4, Intel8080Register::A);
    assert_eq!(e.read_flag(LR35902Flag::Zero), false);
}

#[test]
fn test_bit_true() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0);
    e.test_bit(4, Intel8080Register::A);
    assert_eq!(e.read_flag(LR35902Flag::Zero), true);
}

#[test]
fn shift_register_right_signed() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.shift_register_right_signed(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b11011101);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn shift_register_right_signed_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.shift_register_right_signed(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn shift_register_right_signed_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.shift_register_right_signed(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn shift_register_right_signed_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.shift_register_right_signed(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn shift_register_right() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.shift_register_right(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b01011101);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn shift_register_right_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.shift_register_right(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn shift_register_right_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.shift_register_right(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn shift_register_right_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.shift_register_right(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn shift_register_left() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.shift_register_left(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b01110110);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn shift_register_left_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.shift_register_left(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn shift_register_left_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.shift_register_left(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn shift_register_left_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.shift_register_left(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn swap_register() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0xF8);
    e.swap_register(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0x8F);
}

#[test]
fn swap_register_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.swap_register(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn swap_register_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.swap_register(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn swap_register_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.swap_register(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn rotate_register_right() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_right(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b11011101);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn rotate_register_right_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.rotate_register_right(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn rotate_register_right_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.rotate_register_right(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn rotate_register_right_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.rotate_register_right(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn rotate_register_left() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_left(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b01110111);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn rotate_register_left_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.rotate_register_left(Intel8080Register::A);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn rotate_register_left_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.rotate_register_left(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn rotate_register_left_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.rotate_register_left(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn rotate_register_right_through_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.set_flag(LR35902Flag::Carry, false);
    e.rotate_register_right_through_carry(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b01011101);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn rotate_register_right_through_carry_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_right_through_carry(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn rotate_register_right_through_carry_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_right_through_carry(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn rotate_register_left_through_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b10111011);
    e.set_flag(LR35902Flag::Carry, false);
    e.rotate_register_left_through_carry(Intel8080Register::A);
    assert_eq!(e.read_register(Intel8080Register::A), 0b01110110);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn rotate_register_left_through_carry_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_left_through_carry(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn rotate_register_left_through_carry_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_register(Intel8080Register::A, 0b10111011);
    e.rotate_register_left_through_carry(Intel8080Register::A);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn logical_and_with_accumulator() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b00000001);
    e.set_register(Intel8080Register::B, 0b11000001);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert_eq!(e.read_register(Intel8080Register::A), 0b00000001);
}

#[test]
fn logical_and_with_accumulator_sets_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x0);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_and_with_accumulator_clears_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Zero, true);
    e.set_register(Intel8080Register::A, 0b00110001);
    e.set_register(Intel8080Register::B, 0b00010000);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_and_with_accumulator_clears_subtract() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn logical_and_with_accumulator_clears_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn logical_and_with_accumulator_sets_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_and_with_accumulator(&mut e, Intel8080Register::B);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn logical_exclusive_or_with_accumulator() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b00000001);
    e.set_register(Intel8080Register::B, 0b11000001);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert_eq!(e.read_register(Intel8080Register::A), 0b11000000);
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x33);
    e.set_register(Intel8080Register::B, 0x33);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Zero, true);
    e.set_register(Intel8080Register::A, 0b00110001);
    e.set_register(Intel8080Register::B, 0b00010000);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_subtract() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_exclusive_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn logical_or_with_accumulator() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0b00000001);
    e.set_register(Intel8080Register::B, 0b11000001);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert_eq!(e.read_register(Intel8080Register::A), 0b11000001);
}

#[test]
fn logical_or_with_accumulator_sets_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.set_register(Intel8080Register::B, 0x0);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_or_with_accumulator_clears_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Zero, true);
    e.set_register(Intel8080Register::A, 0b00110001);
    e.set_register(Intel8080Register::B, 0b00010000);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Zero));
}

#[test]
fn logical_or_with_accumulator_clears_subtract() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn logical_or_with_accumulator_clears_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn logical_or_with_accumulator_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_register(Intel8080Register::A, 0x11);
    e.set_register(Intel8080Register::B, 0x22);
    LR35902InstructionSet::logical_or_with_accumulator(&mut e, Intel8080Register::B);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn decimal_adjust_accumulator_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_register(Intel8080Register::A, 0x88);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[cfg(test)]
fn daa_test(input: u8, expected: u8) {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, input);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert_eq!(e.read_register(Intel8080Register::A), expected);
}

#[test]
fn decimal_adjust_accumulator_examples() {
    daa_test(0x1, 0x1);
    daa_test(0xa, 0x10);
    daa_test(0xa8, 0x8);
    daa_test(0x9a, 0x0);
}

#[test]
fn decimal_adjust_accumulator_sets_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x9a);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn decimal_adjust_accumulator_resets_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x91);
    e.set_flag(LR35902Flag::Zero, true);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert!(!e.read_flag(LR35902Flag::Zero));
}

#[test]
fn decimal_adjust_accumulator_sets_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x9b);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn decimal_adjust_accumulator_reads_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x73);
    e.set_flag(LR35902Flag::Carry, true);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert_eq!(e.read_register(Intel8080Register::A), 0xd3);
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn decimal_adjust_accumulator_reads_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    e.set_flag(LR35902Flag::HalfCarry, true);
    e.set_flag(LR35902Flag::Carry, true);
    LR35902InstructionSet::decimal_adjust_accumulator(&mut e);
    assert_eq!(e.read_register(Intel8080Register::A), 0x66);
    assert!(e.read_flag(LR35902Flag::Carry));
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn and_immediate_with_accumulator_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    LR35902InstructionSet::and_immediate_with_accumulator(&mut e, 0x0);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn and_immediate_with_accumulator_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    LR35902InstructionSet::and_immediate_with_accumulator(&mut e, 0x12);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn exclusive_or_immediate_with_accumulator_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    LR35902InstructionSet::exclusive_or_immediate_with_accumulator(&mut e, 0x0);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn exclusive_or_immediate_with_accumulator_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    LR35902InstructionSet::exclusive_or_immediate_with_accumulator(&mut e, 0x12);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn or_immediate_with_accumulator_sets_zero_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x0);
    LR35902InstructionSet::or_immediate_with_accumulator(&mut e, 0x0);
    assert!(e.read_flag(LR35902Flag::Zero));
}

#[test]
fn or_immediate_with_accumulator_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    LR35902InstructionSet::or_immediate_with_accumulator(&mut e, 0x12);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn complement_accumulator_sets_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    LR35902InstructionSet::complement_accumulator(&mut e);
    assert!(e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn complement_accumulator_sets_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    LR35902InstructionSet::complement_accumulator(&mut e);
    assert!(e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn set_carry_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    LR35902InstructionSet::set_carry(&mut e);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn set_carry_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    LR35902InstructionSet::set_carry(&mut e);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn complement_carry_clears_subtract_flag() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::Subtract, true);
    LR35902InstructionSet::complement_carry(&mut e);
    assert!(!e.read_flag(LR35902Flag::Subtract));
}

#[test]
fn complement_carry_clears_half_carry() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_flag(LR35902Flag::HalfCarry, true);
    LR35902InstructionSet::complement_carry(&mut e);
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn compare_immediate_with_accumulator_example1() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x45);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0xF3);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn compare_immediate_with_accumulator_example2() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x02);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0x01);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn compare_immediate_with_accumulator_example3() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x80);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0x01);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn compare_immediate_with_accumulator_example4() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x40);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0x01);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(!e.read_flag(LR35902Flag::Carry));
}

#[test]
fn compare_immediate_with_accumulator_example5() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0x40);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0xFF);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::HalfCarry));
    assert!(e.read_flag(LR35902Flag::Carry));
}

#[test]
fn compare_immediate_with_accumulator_example6() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register(Intel8080Register::A, 0);
    LR35902InstructionSet::compare_immediate_with_accumulator(&mut e, 0x90);
    assert!(e.read_flag(LR35902Flag::Subtract));
    assert!(e.read_flag(LR35902Flag::Carry));
    assert!(!e.read_flag(LR35902Flag::HalfCarry));
}

#[test]
fn increment_register_pair_example1() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x000F);
    LR35902InstructionSet::increment_register_pair(&mut e, Intel8080Register::SP);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0010);
}

#[test]
fn flags_register_keeps_zero_flags_zero() {
    let mut e = new_lr35902_emulator_for_test();
    e.set_register_pair(Intel8080Register::PSW, 0xFFFF);
    assert_eq!(e.read_register_pair(Intel8080Register::PSW), 0xFFF0);
}

/*  _____                     _   _
 * | ____|_  _____  ___ _   _| |_(_) ___  _ __
 * |  _| \ \/ / _ \/ __| | | | __| |/ _ \| '_ \
 * | |___ >  <  __/ (__| |_| | |_| | (_) | | | |
 * |_____/_/\_\___|\___|\__,_|\__|_|\___/|_| |_|
 *
 */

impl<M: MemoryAccessor> LR35902Emulator<M> {
    fn crash(&mut self, message: String) {
        self.crash_message = Some(message);
    }

    pub fn crashed(&self) -> bool {
        self.crash_message.is_some()
    }

    fn run_lr35902_instruction(&mut self, instruction: &[u8]) {
        let pc = self.read_program_counter() as usize;
        self.set_program_counter((pc + instruction.len()) as u16);
        let duration = dispatch_lr35902_instruction(&instruction, self);
        self.add_cycles(duration);
    }

    fn crash_from_unkown_opcode(&mut self) {
        let pc = self.read_program_counter();
        self.crash(format!("Unknown opcode at address {:x}", pc));
    }

    pub fn run_one_instruction(&mut self) {
        let instr;
        {
            let pc = self.read_program_counter();
            let stream = MemoryStream::new(&self.memory_accessor, pc);
            instr = get_lr35902_instruction(stream);
        }
        match instr {
            Some(res) => {
                self.run_lr35902_instruction(&res);
                return;
            }
            None => self.crash_from_unkown_opcode(),
        };
    }
}

#[test]
fn emulator_crashes_on_unkown_opcode() {
    let mut e = new_lr35902_emulator_for_test();
    e.memory_accessor.memory[0..1].clone_from_slice(&[0xfc]);
    e.set_program_counter(0);
    e.run_one_instruction();
    assert_eq!(e.crash_message.unwrap(), "Unknown opcode at address 0");
}

/*  ____  _                         _____         _     ____   ___  __  __
 * | __ )| | __ _ _ __ __ _  __ _  |_   _|__  ___| |_  |  _ \ / _ \|  \/  |___
 * |  _ \| |/ _` | '__/ _` |/ _` |   | |/ _ \/ __| __| | |_) | | | | |\/| / __|
 * | |_) | | (_| | | | (_| | (_| |   | |  __/\__ \ |_  |  _ <| |_| | |  | \__ \
 * |____/|_|\__,_|_|  \__, |\__, |   |_|\___||___/\__| |_| \_\\___/|_|  |_|___/
 *                    |___/ |___/
 *
 */

#[cfg(test)]
fn load_rom(e: &mut LR35902Emulator<SimpleMemoryAccessor>, rom: &Vec<u8>) {
    e.memory_accessor.memory[0..rom.len()].clone_from_slice(rom);
}

#[cfg(test)]
pub fn read_blargg_test_rom(name: &str) -> Vec<u8> {
    let mut rom: Vec<u8> = vec![];
    let mut file = File::open(format!("blargg_test_roms/{}", name))
        .ok()
        .expect("Did you forget to download the test roms?");
    file.read_to_end(&mut rom).unwrap();
    return rom;
}

#[cfg(test)]
pub fn run_blargg_test_rom<M: MemoryAccessor>(e: &mut LR35902Emulator<M>, stop_address: u16) {
    let mut pc = e.read_program_counter();
    // This address is where the rom ends.  At this address is an infinite loop where normally the
    // rom will sit at forever.
    while pc != stop_address {
        e.run_one_instruction();
        pc = e.read_program_counter();
    }

    // Scrape from memory what is displayed on the screen
    let mut message = String::new();
    let iter = &mut MemoryIterator::new(&e.memory_accessor, 0x9800..0x9BFF).peekable();
    while iter.peek() != None {
        for c in iter.take(0x20) {
            // The rom happens to use ASCII as the way it maps characters to the correct tile.
            message.push(c as char);
        }
        message = String::from(message.trim_right());
        message.push('\n');
    }

    // The message ends with 'Passed' when the test was successful
    assert!(message.ends_with("Passed\n"), "{}", message);
}

#[cfg(test)]
fn run_blargg_test_rom_cpu_instrs(name: &str, address: u16) {
    let mut e = new_lr35902_emulator_for_test();
    load_rom(&mut e, &read_blargg_test_rom(name));
    run_blargg_test_rom(&mut e, address);
}

#[test]
fn blargg_test_rom_cpu_instrs_1_special() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/01-special.gb", 0xc7d2);
}

#[test]
fn blargg_test_rom_cpu_instrs_3_op_sp_hl() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/03-op sp,hl.gb", 0xcb44);
}

// XXX: Why does this test fail? I have no idea!
#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_4_op_r_imm() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/04-op r,imm.gb", 0xcb35);
}

#[test]
fn blargg_test_rom_cpu_instrs_5_op_rp() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/05-op rp.gb", 0xcb31);
}

#[test]
fn blargg_test_rom_cpu_instrs_6_ld_r_r() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/06-ld r,r.gb", 0xcc5f);
}

#[test]
fn blargg_test_rom_cpu_instrs_7_jr_jp_call_ret_rst() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/07-jr,jp,call,ret,rst.gb", 0xcbb0);
}

#[test]
fn blargg_test_rom_cpu_instrs_8_misc_instrs() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/08-misc instrs.gb", 0xcb91);
}

#[test]
fn blargg_test_rom_cpu_instrs_9_op_r_r() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/09-op r,r.gb", 0xce67);
}

#[test]
fn blargg_test_rom_cpu_instrs_10_bit_ops() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/10-bit ops.gb", 0xcf58);
}

#[test]
fn blargg_test_rom_cpu_instrs_11_op_a_hl() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/11-op a,(hl).gb", 0xcc62);
}
