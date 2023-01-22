// Copyright 2017 Remi Bernotavicius

use crate::intel_8080_emulator::{
    Intel8080Flag, Intel8080InstructionSet, Intel8080InstructionSetOps,
};
use crate::util::TwosComplement;
use alloc::{format, string::String, vec::Vec};
use serde_derive::{Deserialize, Serialize};

pub use crate::emulator_common::Intel8080Register;
pub use crate::emulator_common::{MemoryAccessor, SimpleMemoryAccessor};
pub use crate::lr35902_emulator::debugger::run_debugger;
pub use crate::lr35902_emulator::opcodes::{
    disassemble_lr35902_rom, LR35902Instruction, LR35902InstructionSet, LR35902InstructionType,
    NUM_INSTRUCTIONS,
};

pub mod debugger;
mod opcodes;

/*  _     ____  _________  ___   ___ ____  _____                 _       _
 * | |   |  _ \|___ / ___|/ _ \ / _ \___ \| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * | |   | |_) | |_ \___ \ (_) | | | |__) |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |___|  _ < ___) |__) \__, | |_| / __/| |___| | | | | | |_| | | (_| | || (_) | |
 * |_____|_| \_\____/____/  /_/ \___/_____|_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 */

const ROM_ADDRESS: usize = 0x0100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LR35902Flag {
    // 76543210
    Zero = 0b10000000,
    Subtract = 0b01000000,
    HalfCarry = 0b00100000,
    Carry = 0b00010000,

    ValidityMask = 0b11110000,
}

#[derive(Serialize, Deserialize)]
pub struct LR35902Emulator {
    registers: [u8; Intel8080Register::Count as usize],
    program_counter: u16,
    pub elapsed_cycles: u64,
    pub crash_message: Option<String>,
    pub call_stack: Vec<u16>,
    halted: bool,
    instr: Option<LR35902Instruction>,
}

impl Default for LR35902Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl LR35902Emulator {
    pub fn new() -> LR35902Emulator {
        let mut e = LR35902Emulator {
            registers: [0; Intel8080Register::Count as usize],
            program_counter: 0,
            elapsed_cycles: 102348,
            crash_message: None,
            call_stack: Vec::new(),
            halted: false,
            instr: None,
        };

        e.set_register_pair(Intel8080Register::SP, 0xFFFE);
        e.set_program_counter(ROM_ADDRESS as u16);

        e
    }

    pub fn get_last_instruction(&self) -> Option<LR35902Instruction> {
        self.instr.clone()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn set_flag(&mut self, flag: LR35902Flag, value: bool) {
        if value {
            self.registers[Intel8080Register::FLAGS as usize] |= flag as u8;
        } else {
            self.registers[Intel8080Register::FLAGS as usize] &= !(flag as u8);
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.registers[Intel8080Register::FLAGS as usize] & flag as u8 == flag as u8
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn read_register(&self, register: Intel8080Register) -> u8 {
        self.read_raw_register(register as usize)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn set_register(&mut self, register: Intel8080Register, value: u8) {
        self.set_raw_register(register as usize, value)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn read_register_pair(&self, register: Intel8080Register) -> u16 {
        self.read_raw_register_pair(register as usize / 2)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn set_register_pair(&mut self, register: Intel8080Register, value: u16) {
        self.set_raw_register_pair(register as usize / 2, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register(&mut self, index: usize, value: u8) {
        assert!(index != Intel8080Register::FLAGS as usize);
        self.registers[index] = value;
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register_pair(&self, index: usize) -> u16 {
        let first_byte = index * 2;
        let second_byte = first_byte + 1;
        u16::from_be_bytes([self.registers[first_byte], self.registers[second_byte]])
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        let first_byte = index * 2;
        let second_byte = first_byte + 1;

        let bytes = value.to_be_bytes();
        self.registers[first_byte] = bytes[0];
        self.registers[second_byte] = bytes[1];

        if second_byte == Intel8080Register::FLAGS as usize {
            // If we are setting the FLAGS register, we need to force the zero flags to be zero.
            self.registers[Intel8080Register::FLAGS as usize] &= LR35902Flag::ValidityMask as u8;
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn read_program_counter(&self) -> u16 {
        self.program_counter
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn push_u16_onto_stack<M: MemoryAccessor>(
        &mut self,
        memory_accessor: &mut M,
        address: u16,
    ) {
        let mut ops = InstructionDispatchOps::new(self, memory_accessor);
        Intel8080InstructionSetOps::push_u16_onto_stack(&mut ops, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn add_cycles(&mut self, cycles: u8) {
        self.elapsed_cycles += cycles as u64;
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn resume(&mut self) {
        self.halted = false;
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn is_halted(&self) -> bool {
        self.halted
    }
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
    fn add_cycles(&mut self, cycles: u8);
    fn push_frame(&mut self, address: u16);
    fn pop_frame(&mut self);

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn get_relative_address(&self, n: u8) -> u16 {
        self.read_program_counter()
            .wrapping_add(((n as i8) as i16) as u16)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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

        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, ovalue_b: u8) -> u8 {
        let value_b = ovalue_b.twos_complement();
        let new_value = value_a.wrapping_add(value_b);

        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, true);
        self.set_flag(LR35902Flag::HalfCarry, new_value & 0x0F > (value_a & 0x0F));
        self.set_flag(LR35902Flag::Carry, value_a < ovalue_b);

        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a.wrapping_sub(value_b);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::HalfCarry, value_b & 0x0F > (value_a & 0x0F));
        self.set_flag(LR35902Flag::Subtract, true);
        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a & value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, true);
        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a ^ value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a | value_b;
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::Carry, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        new_value
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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

        new_value
    }

    fn wait_until_interrupt(&mut self);
}

struct InstructionDispatchOps<'a, M: MemoryAccessor> {
    emulator: &'a mut LR35902Emulator,
    memory_accessor: &'a mut M,
}

impl<'a, M: MemoryAccessor> InstructionDispatchOps<'a, M> {
    fn new(emulator: &'a mut LR35902Emulator, memory_accessor: &'a mut M) -> Self {
        Self {
            emulator,
            memory_accessor,
        }
    }
}

impl<'a, M: MemoryAccessor> InstructionDispatchOps<'a, M> {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_flag(&mut self, flag: LR35902Flag, value: bool) {
        self.emulator.set_flag(flag, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.emulator.read_flag(flag)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        self.memory_accessor.read_memory(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory_accessor.set_memory(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory_u16(&self, address: u16) -> u16 {
        self.memory_accessor.read_memory_u16(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.memory_accessor.set_memory_u16(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_program_counter(&self) -> u16 {
        self.emulator.read_program_counter()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_program_counter(&mut self, address: u16) {
        self.emulator.set_program_counter(address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_interrupts_enabled(&mut self, value: bool) {
        self.memory_accessor.set_interrupts_enabled(value);
    }
}

impl<'a, M: MemoryAccessor> LR35902InstructionSetOps for InstructionDispatchOps<'a, M> {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_flag(&mut self, flag: LR35902Flag, value: bool) {
        self.set_flag(flag, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.read_flag(flag)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory_u16(&self, address: u16) -> u16 {
        self.read_memory_u16(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory_u16(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register(&self, index: usize) -> u8 {
        self.emulator.read_raw_register(index)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register(&mut self, index: usize, value: u8) {
        self.emulator.set_raw_register(index, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register_pair(&self, index: usize) -> u16 {
        self.emulator.read_raw_register_pair(index)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        self.emulator.set_raw_register_pair(index, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_program_counter(&self) -> u16 {
        self.read_program_counter()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_program_counter(&mut self, address: u16) {
        self.set_program_counter(address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_interrupts_enabled(&mut self, value: bool) {
        self.set_interrupts_enabled(value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_cycles(&mut self, cycles: u8) {
        self.emulator.add_cycles(cycles);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn push_frame(&mut self, address: u16) {
        self.emulator.call_stack.push(address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn pop_frame(&mut self) {
        self.emulator.call_stack.pop();
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn wait_until_interrupt(&mut self) {
        self.emulator.halted = true;
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
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_memory_u16(&self, address: u16) -> u16 {
        self.read_memory_u16(address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory_u16(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_flag(&mut self, flag: Intel8080Flag, value: bool) {
        match flag {
            Intel8080Flag::Zero => self.set_flag(LR35902Flag::Zero, value),
            Intel8080Flag::AuxiliaryCarry => self.set_flag(LR35902Flag::HalfCarry, value),
            Intel8080Flag::Carry => self.set_flag(LR35902Flag::Carry, value),
            _ => {}
        };
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_flag(&self, flag: Intel8080Flag) -> bool {
        match flag {
            Intel8080Flag::Zero => self.read_flag(LR35902Flag::Zero),
            Intel8080Flag::AuxiliaryCarry => self.read_flag(LR35902Flag::HalfCarry),
            Intel8080Flag::Carry => self.read_flag(LR35902Flag::Carry),
            flag => panic!("LR35902 doesn't know about {:?}", flag),
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register(&self, index: usize) -> u8 {
        self.read_raw_register(index)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register(&mut self, index: usize, value: u8) {
        self.set_raw_register(index, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_raw_register_pair(&self, index: usize) -> u16 {
        self.read_raw_register_pair(index)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        self.set_raw_register_pair(index, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8 {
        self.perform_addition(value_a, value_b, update_carry)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_subtraction_using_twos_complement(value_a, value_b)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_subtraction(value_a, value_b)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_and(value_a, value_b)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_exclusive_or(value_a, value_b)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        self.perform_or(value_a, value_b)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn read_program_counter(&self) -> u16 {
        self.read_program_counter()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_program_counter(&mut self, address: u16) {
        self.set_program_counter(address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_interrupts_enabled(&mut self, value: bool) {
        self.set_interrupts_enabled(value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_cycles(&mut self, cycles: u8) {
        self.add_cycles(cycles);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn push_frame(&mut self, address: u16) {
        self.push_frame(address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn pop_frame(&mut self) {
        self.pop_frame();
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn wait_until_interrupt(&mut self) {
        self.wait_until_interrupt();
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
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.set_memory(0x1122, 0x88);
        assert_eq!(e.read_memory(0x1122), 0x88);
    });
}

#[test]
fn can_set_and_read_memory_16_bit() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.set_memory_u16(0x1122, 0x2233);
        assert_eq!(e.read_memory_u16(0x1122), 0x2233);
    });
}

#[test]
fn can_set_and_read_regiser() {
    instruction_test(|e| {
        let e: &mut dyn Intel8080InstructionSetOps = e;
        e.set_register(Intel8080Register::A, 0x45);
        assert_eq!(e.read_register(Intel8080Register::A), 0x45);
    });
}

#[test]
fn can_set_and_read_regiser_pair() {
    instruction_test(|e| {
        let e: &mut dyn Intel8080InstructionSetOps = e;
        e.set_register_pair(Intel8080Register::B, 0x4523);
        assert_eq!(e.read_register_pair(Intel8080Register::B), 0x4523);
    });
}

#[test]
fn perform_addition() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        assert_eq!(
            e.perform_addition(0x33, 0x11, false /* update carry */),
            0x44
        );
    });
}

#[test]
fn perform_addition_with_overflow() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        assert_eq!(
            e.perform_addition(0xF3, 0x11, false /* update carry */),
            0x04
        );
    });
}

#[test]
fn perform_addition_sets_zero_flag() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_addition(0xF3, 0x0D, false /* update carry */);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn perform_addition_sets_half_carry() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_addition(0x0F, 0x01, false /* update carry */);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn perform_addition_clears_subtract_flag() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.set_flag(LR35902Flag::Subtract, true);
        e.perform_addition(0x0D, 0x01, false /* update carry */);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn perform_addition_does_not_set_carry() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_addition(0xFF, 0x01, false /* update carry */);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn perform_addition_clears_carry() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.set_flag(LR35902Flag::Carry, true);
        e.perform_addition(0xF1, 0x01, true /* update carry */);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn perform_addition_sets_carry() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_addition(0xFF, 0x01, true /* update carry */);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn perform_subtraction() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        assert_eq!(e.perform_subtraction(0x12, 0x11), 0x01);
    });
}

#[test]
fn perform_subtraction_with_underflow() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        assert_eq!(e.perform_subtraction(0x12, 0x13), 0xFF);
    });
}

#[test]
fn perform_subtraction_sets_zero_flag() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_subtraction(0x12, 0x12);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn perform_subtraction_sets_subtract_flag() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_subtraction(0x12, 0x04);
        assert!(e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn perform_subtraction_sets_half_carry_flag() {
    instruction_test(|e| {
        let e: &mut dyn LR35902InstructionSetOps = e;
        e.perform_subtraction(0x03, 0x04);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
    });
}

/*  ___           _                   _   _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __  ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
 *
 */

impl<I: LR35902InstructionSetOps> LR35902InstructionSet for I {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn move_and_increment_hl(
        &mut self,
        dest_register: Intel8080Register,
        src_register: Intel8080Register,
    ) {
        LR35902InstructionSet::move_data(self, dest_register, src_register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        self.set_register_pair(Intel8080Register::H, old_value.wrapping_add(1));
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn move_and_decrement_hl(
        &mut self,
        dest_register: Intel8080Register,
        src_register: Intel8080Register,
    ) {
        LR35902InstructionSet::move_data(self, dest_register, src_register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        self.set_register_pair(Intel8080Register::H, old_value.wrapping_sub(1));
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_accumulator_direct(&mut self, address: u16) {
        Intel8080InstructionSet::store_accumulator_direct(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_sp_plus_immediate(&mut self, data: u8) {
        let sp = self.read_register_pair(Intel8080Register::SP);
        let address = self.perform_signed_double_add(sp, data);
        self.set_register_pair(Intel8080Register::H, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_immediate_to_sp(&mut self, data: u8) {
        let sp = self.read_register_pair(Intel8080Register::SP);
        let new_value = self.perform_signed_double_add(sp, data);
        self.set_register_pair(Intel8080Register::SP, new_value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_accumulator_direct_one_byte(&mut self, relative_address: u8) {
        let value = self.read_register(Intel8080Register::A);
        self.set_memory(0xFF00 + relative_address as u16, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_accumulator_one_byte(&mut self) {
        let relative_address = self.read_register(Intel8080Register::C);
        self.store_accumulator_direct_one_byte(relative_address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_accumulator_direct_one_byte(&mut self, relative_address: u8) {
        let value = self.read_memory(0xFF00 + relative_address as u16);
        self.set_register(Intel8080Register::A, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_accumulator_one_byte(&mut self) {
        let relative_address = self.read_register(Intel8080Register::C);
        self.load_accumulator_direct_one_byte(relative_address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_and_enable_interrupts(&mut self) {
        LR35902InstructionSet::return_unconditionally(self);
        LR35902InstructionSet::enable_interrupts(self);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn halt_until_button_press(&mut self) {
        unimplemented!();
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_relative(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_relative_if_zero(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_zero(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_relative_if_not_zero(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_not_zero(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_relative_if_carry(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_carry(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_relative_if_no_carry(&mut self, n: u8) {
        let address = self.get_relative_address(n);
        Intel8080InstructionSet::jump_if_no_carry(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_sp_direct(&mut self, address: u16) {
        let value = self.read_register_pair(Intel8080Register::SP);
        self.set_memory_u16(address, value);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_accumulator_direct(&mut self, address: u16) {
        Intel8080InstructionSet::load_accumulator_direct(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn reset_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value & !(1u8 << bit));
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_register(register, value | (1u8 << bit));
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn test_bit(&mut self, bit: u8, register: Intel8080Register) {
        assert!(bit < 8);
        let value = self.read_register(register);
        self.set_flag(LR35902Flag::Zero, (value & (1u8 << bit)) == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, true);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn shift_register_right_signed(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = ((value as i8) >> 1) as u8;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & 1) != 0);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn shift_register_right(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = value >> 1;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & 1) != 0);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn shift_register_left(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = value << 1;
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, (value & (1u8 << 7)) != 0);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn swap_register(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = (value << 4) | (value >> 4);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
        self.set_flag(LR35902Flag::Carry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_register_right(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_register_left(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_register_right_through_carry(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_right_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_register_left_through_carry(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        let new_value = self.perform_rotate_left_through_carry(value);
        self.set_register(register, new_value);
        self.set_flag(LR35902Flag::Zero, new_value == 0);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_accumulator_right(&mut self) {
        self.rotate_register_right(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_accumulator_left(&mut self) {
        self.rotate_register_left(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_accumulator_right_through_carry(&mut self) {
        self.rotate_register_right_through_carry(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn rotate_accumulator_left_through_carry(&mut self) {
        self.rotate_register_left_through_carry(Intel8080Register::A);
        self.set_flag(LR35902Flag::Zero, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn complement_accumulator(&mut self) {
        Intel8080InstructionSet::complement_accumulator(self);
        self.set_flag(LR35902Flag::Subtract, true);
        self.set_flag(LR35902Flag::HalfCarry, true);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn set_carry(&mut self) {
        Intel8080InstructionSet::set_carry(self);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn complement_carry(&mut self) {
        Intel8080InstructionSet::complement_carry(self);
        self.set_flag(LR35902Flag::Subtract, false);
        self.set_flag(LR35902Flag::HalfCarry, false);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_sp_from_h_and_l(&mut self) {
        Intel8080InstructionSet::load_sp_from_h_and_l(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn or_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::or_immediate_with_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn no_operation(&mut self) {
        Intel8080InstructionSet::no_operation(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_register_pair_immediate(&mut self, register: Intel8080Register, data: u16) {
        Intel8080InstructionSet::load_register_pair_immediate(self, register, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn move_data(&mut self, dest_register: Intel8080Register, src_register: Intel8080Register) {
        Intel8080InstructionSet::move_data(self, dest_register, src_register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn enable_interrupts(&mut self) {
        Intel8080InstructionSet::enable_interrupts(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_if_zero(&mut self) {
        Intel8080InstructionSet::return_if_zero(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn exclusive_or_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::exclusive_or_immediate_with_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn and_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::and_immediate_with_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn decrement_register_or_memory(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::decrement_register_or_memory(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn halt(&mut self) {
        Intel8080InstructionSet::halt(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn compare_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::compare_with_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn restart(&mut self, implicit_data: u8) {
        Intel8080InstructionSet::restart(self, implicit_data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn decrement_register_pair(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::decrement_register_pair(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_if_not_zero(&mut self) {
        Intel8080InstructionSet::return_if_not_zero(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn logical_or_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_or_with_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump(&mut self, address: u16) {
        Intel8080InstructionSet::jump(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn call_if_not_zero(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_not_zero(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn subtract_immediate_from_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::subtract_immediate_from_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn subtract_from_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::subtract_from_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_accumulator(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::load_accumulator(self, register_pair)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_unconditionally(&mut self) {
        Intel8080InstructionSet::return_unconditionally(self);
        self.pop_frame();
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_if_not_zero(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_not_zero(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn call_if_carry(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_carry(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn logical_and_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_and_with_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_if_no_carry(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_no_carry(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn call(&mut self, address: u16) {
        Intel8080InstructionSet::call(self, address);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_if_no_carry(&mut self) {
        Intel8080InstructionSet::return_if_no_carry(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn call_if_zero(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_zero(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_if_carry(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_carry(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_immediate_to_accumulator_with_carry(&mut self, data: u8) {
        Intel8080InstructionSet::add_immediate_to_accumulator_with_carry(self, data);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn increment_register_pair(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::increment_register_pair(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn store_accumulator(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::store_accumulator(self, register_pair)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_to_accumulator_with_carry(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::add_to_accumulator_with_carry(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn subtract_from_accumulator_with_borrow(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::subtract_from_accumulator_with_borrow(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn push_data_onto_stack(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::push_data_onto_stack(self, register_pair)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn increment_register_or_memory(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::increment_register_or_memory(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn load_program_counter(&mut self) {
        Intel8080InstructionSet::load_program_counter(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn pop_data_off_stack(&mut self, register_pair: Intel8080Register) {
        Intel8080InstructionSet::pop_data_off_stack(self, register_pair)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_immediate_to_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::add_immediate_to_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn logical_exclusive_or_with_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::logical_exclusive_or_with_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn add_to_accumulator(&mut self, register: Intel8080Register) {
        Intel8080InstructionSet::add_to_accumulator(self, register)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn disable_interrupts(&mut self) {
        Intel8080InstructionSet::disable_interrupts(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn compare_immediate_with_accumulator(&mut self, data: u8) {
        Intel8080InstructionSet::compare_immediate_with_accumulator(self, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn move_immediate_data(&mut self, dest_register: Intel8080Register, data: u8) {
        Intel8080InstructionSet::move_immediate_data(self, dest_register, data)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn call_if_no_carry(&mut self, address: u16) {
        Intel8080InstructionSet::call_if_no_carry(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn return_if_carry(&mut self) {
        Intel8080InstructionSet::return_if_carry(self)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn jump_if_zero(&mut self, address: u16) {
        Intel8080InstructionSet::jump_if_zero(self, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data: u8) {
        Intel8080InstructionSet::subtract_immediate_from_accumulator_with_borrow(self, data);
    }
}

/*  ___           _                   _   _               _____         _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __   |_   _|__  ___| |_ ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \    | |/ _ \/ __| __/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | |   | |  __/\__ \ |_\__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|   |_|\___||___/\__|___/
 *
 */

#[cfg(test)]
fn instruction_test<F: Fn(&mut InstructionDispatchOps<SimpleMemoryAccessor>)>(func: F) {
    let mut emulator = LR35902Emulator::new();
    let mut memory_accessor = SimpleMemoryAccessor::new();
    let mut ops = InstructionDispatchOps::new(&mut emulator, &mut memory_accessor);
    func(&mut ops);
}

#[test]
fn move_and_increment_hl() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x1122);
        e.set_register(Intel8080Register::M, 0x99);
        e.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M);
        assert_eq!(e.read_register(Intel8080Register::A), 0x99);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1123);
    })
}

#[test]
fn move_and_increment_hl_overflows() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0xFFFF);
        e.move_and_increment_hl(Intel8080Register::A, Intel8080Register::M);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0x0);
    });
}

#[test]
fn move_and_decrement_hl() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x1122);
        e.set_register(Intel8080Register::M, 0x99);
        e.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M);
        assert_eq!(e.read_register(Intel8080Register::A), 0x99);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1121);
    });
}

#[test]
fn move_and_decrement_hl_underflows() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x0);
        e.move_and_decrement_hl(Intel8080Register::A, Intel8080Register::M);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0xFFFF);
    });
}

#[test]
fn store_accumulator_direct() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x44);
        LR35902InstructionSet::store_accumulator_direct(e, 0x5588);
        assert_eq!(e.read_memory(0x5588), 0x44);
    });
}

#[test]
fn store_sp_plus_immediate() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x4488);
        e.store_sp_plus_immediate(0x77);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0x4488 + 0x77);
    });
}

#[test]
fn store_sp_plus_immediate_with_overflow() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0xFF88);
        e.store_sp_plus_immediate(0x77);
        assert_eq!(
            e.read_register_pair(Intel8080Register::H),
            0xFF88u16.wrapping_add(0x77)
        );
    });
}

#[test]
fn add_immediate_to_sp() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x4488);
        e.add_immediate_to_sp(0x22);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x44aa);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example1() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0xFFFF);
        e.add_immediate_to_sp(0x01);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example2() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x00FF);
        e.add_immediate_to_sp(0x01);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example3() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x00F0);
        e.add_immediate_to_sp(0x10);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example4() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0xFFFF);
        e.add_immediate_to_sp(0x84);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xFF83);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example5() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x000F);
        e.add_immediate_to_sp(0x01);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0010);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn add_immediate_to_sp_example6() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x0000);
        e.add_immediate_to_sp(0x90);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xFF90);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn subtract_immediate_from_accumulator_example1() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0xFF);
        LR35902InstructionSet::subtract_immediate_from_accumulator(e, 0x01);
        assert_eq!(e.read_register(Intel8080Register::A), 0xFE);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn subtract_immediate_from_accumulator_example2() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x04);
        LR35902InstructionSet::subtract_immediate_from_accumulator(e, 0x05);
        assert_eq!(e.read_register(Intel8080Register::A), 0xFF);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn subtract_immediate_from_accumulator_example3() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x14);
        LR35902InstructionSet::subtract_immediate_from_accumulator(e, 0x05);
        assert_eq!(e.read_register(Intel8080Register::A), 0x0F);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn subtract_immediate_from_accumulator_example4() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x14);
        LR35902InstructionSet::subtract_immediate_from_accumulator(e, 0x86);
        assert_eq!(e.read_register(Intel8080Register::A), 0x8E);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn double_add_updates_half_carry() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x0FFF);
        e.set_register_pair(Intel8080Register::B, 0x0001);
        LR35902InstructionSet::double_add(e, Intel8080Register::B);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn double_add_does_not_update_half_carry() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x00FF);
        e.set_register_pair(Intel8080Register::B, 0x0001);
        LR35902InstructionSet::double_add(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn double_add_adds() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::H, 0x000F);
        e.set_register_pair(Intel8080Register::B, 0x0001);
        LR35902InstructionSet::double_add(e, Intel8080Register::B);
        assert_eq!(e.read_register_pair(Intel8080Register::H), 0x0010);
    });
}

#[test]
fn store_accumulator_direct_one_byte() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x34);
        e.store_accumulator_direct_one_byte(0x22);
        assert_eq!(e.read_memory(0xFF22), 0x34);
    });
}

#[test]
fn load_accumulator_direct_one_byte() {
    instruction_test(|e| {
        e.set_memory(0xFF22, 0x34);
        e.load_accumulator_direct_one_byte(0x22);
        assert_eq!(e.read_register(Intel8080Register::A), 0x34);
    });
}

#[test]
fn store_accumulator_one_byte() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x34);
        e.set_register(Intel8080Register::C, 0x22);
        e.store_accumulator_one_byte();
        assert_eq!(e.read_memory(0xFF22), 0x34);
    });
}

#[test]
fn load_accumulator_one_byte() {
    instruction_test(|e| {
        e.set_memory(0xFF22, 0x34);
        e.set_register(Intel8080Register::C, 0x22);
        e.load_accumulator_one_byte();
        assert_eq!(e.read_register(Intel8080Register::A), 0x34);
    });
}

#[test]
fn return_and_enable_interrupts() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x0400);
        e.return_and_enable_interrupts();
        assert_eq!(e.read_program_counter(), 0x0000);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0402);
    });
}

#[test]
fn jump_relative_negative() {
    instruction_test(|e| {
        e.set_program_counter(0x1234);
        e.jump_relative(-4i8 as u8);
        assert_eq!(e.read_program_counter(), 0x1230);
    });
}

#[test]
fn jump_relative_example() {
    instruction_test(|e| {
        e.set_program_counter(0x297);
        e.jump_relative(0xFC);
        assert_eq!(e.read_program_counter(), 0x293);
    });
}

#[test]
fn jump_relative() {
    instruction_test(|e| {
        e.set_program_counter(0x1234);
        e.jump_relative(0x11);
        assert_eq!(e.read_program_counter(), 0x1245);
    });
}

#[test]
fn jump_relative_if_zero() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Zero, true);
        e.set_program_counter(0x1234);
        e.jump_relative_if_zero(0x11);
        assert_eq!(e.read_program_counter(), 0x1245);
    });
}

#[test]
fn jump_relative_if_not_zero() {
    instruction_test(|e| {
        e.set_program_counter(0x1234);
        e.jump_relative_if_not_zero(0x11);
        assert_eq!(e.read_program_counter(), 0x1245);
    });
}

#[test]
fn jump_relative_if_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Carry, true);
        e.set_program_counter(0x1234);
        e.jump_relative_if_carry(0x11);
        assert_eq!(e.read_program_counter(), 0x1245);
    });
}

#[test]
fn jump_relative_if_no_carry() {
    instruction_test(|e| {
        e.set_program_counter(0x1234);
        e.jump_relative_if_no_carry(0x11);
        assert_eq!(e.read_program_counter(), 0x1245);
    });
}

#[test]
fn store_sp_direct() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x9923);
        e.store_sp_direct(0x8833);
        assert_eq!(e.read_memory_u16(0x8833), 0x9923);
    });
}

#[test]
fn store_sp_at_ffff() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x9923);
        e.store_sp_direct(0xFFFF);

        // This address is the Interrupt Enable Flag, so this test isn't quite legit.
        assert_eq!(e.read_memory(0xFFFF), 0x23);
    });
}

#[test]
fn reset_bit() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0xFF);
        e.reset_bit(4, Intel8080Register::A);
        e.reset_bit(0, Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b11101110);
    });
}

#[test]
fn set_bit() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0);
        e.set_bit(4, Intel8080Register::A);
        e.set_bit(0, Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b00010001);
    });
}

#[test]
fn test_bit_false() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b00010000);
        e.test_bit(4, Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn test_bit_true() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0);
        e.test_bit(4, Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn shift_register_right_signed() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.shift_register_right_signed(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b11011101);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn shift_register_right_signed_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.shift_register_right_signed(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn shift_register_right_signed_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.shift_register_right_signed(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn shift_register_right_signed_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.shift_register_right_signed(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn shift_register_right() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.shift_register_right(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b01011101);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn shift_register_right_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.shift_register_right(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn shift_register_right_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.shift_register_right(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn shift_register_right_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.shift_register_right(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn shift_register_left() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.shift_register_left(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b01110110);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn shift_register_left_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.shift_register_left(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn shift_register_left_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.shift_register_left(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn shift_register_left_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.shift_register_left(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn swap_register() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0xF8);
        e.swap_register(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0x8F);
    });
}

#[test]
fn swap_register_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.swap_register(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn swap_register_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.swap_register(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn swap_register_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.swap_register(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn rotate_register_right() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_right(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b11011101);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn rotate_register_right_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.rotate_register_right(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn rotate_register_right_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.rotate_register_right(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn rotate_register_right_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.rotate_register_right(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn rotate_register_left() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_left(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b01110111);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn rotate_register_left_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.rotate_register_left(Intel8080Register::A);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn rotate_register_left_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.rotate_register_left(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn rotate_register_left_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.rotate_register_left(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn rotate_register_right_through_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.set_flag(LR35902Flag::Carry, false);
        e.rotate_register_right_through_carry(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b01011101);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn rotate_register_right_through_carry_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_right_through_carry(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn rotate_register_right_through_carry_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_right_through_carry(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn rotate_register_left_through_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b10111011);
        e.set_flag(LR35902Flag::Carry, false);
        e.rotate_register_left_through_carry(Intel8080Register::A);
        assert_eq!(e.read_register(Intel8080Register::A), 0b01110110);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn rotate_register_left_through_carry_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_left_through_carry(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn rotate_register_left_through_carry_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_register(Intel8080Register::A, 0b10111011);
        e.rotate_register_left_through_carry(Intel8080Register::A);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn logical_and_with_accumulator() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b00000001);
        e.set_register(Intel8080Register::B, 0b11000001);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert_eq!(e.read_register(Intel8080Register::A), 0b00000001);
    });
}

#[test]
fn logical_and_with_accumulator_sets_zero() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x0);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_and_with_accumulator_clears_zero() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Zero, true);
        e.set_register(Intel8080Register::A, 0b00110001);
        e.set_register(Intel8080Register::B, 0b00010000);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_and_with_accumulator_clears_subtract() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn logical_and_with_accumulator_clears_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Carry, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn logical_and_with_accumulator_sets_half_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_and_with_accumulator(e, Intel8080Register::B);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn logical_exclusive_or_with_accumulator() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b00000001);
        e.set_register(Intel8080Register::B, 0b11000001);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert_eq!(e.read_register(Intel8080Register::A), 0b11000000);
    });
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_zero() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x33);
        e.set_register(Intel8080Register::B, 0x33);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_zero() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Zero, true);
        e.set_register(Intel8080Register::A, 0b00110001);
        e.set_register(Intel8080Register::B, 0b00010000);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_subtract() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Carry, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_exclusive_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn logical_or_with_accumulator() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0b00000001);
        e.set_register(Intel8080Register::B, 0b11000001);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert_eq!(e.read_register(Intel8080Register::A), 0b11000001);
    });
}

#[test]
fn logical_or_with_accumulator_sets_zero() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.set_register(Intel8080Register::B, 0x0);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_or_with_accumulator_clears_zero() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Zero, true);
        e.set_register(Intel8080Register::A, 0b00110001);
        e.set_register(Intel8080Register::B, 0b00010000);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn logical_or_with_accumulator_clears_subtract() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn logical_or_with_accumulator_clears_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Carry, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn logical_or_with_accumulator_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_register(Intel8080Register::A, 0x11);
        e.set_register(Intel8080Register::B, 0x22);
        LR35902InstructionSet::logical_or_with_accumulator(e, Intel8080Register::B);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn decimal_adjust_accumulator_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_register(Intel8080Register::A, 0x88);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[cfg(test)]
fn daa_test(input: u8, expected: u8) {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, input);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert_eq!(e.read_register(Intel8080Register::A), expected);
    });
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
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x9a);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn decimal_adjust_accumulator_resets_zero() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x91);
        e.set_flag(LR35902Flag::Zero, true);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert!(!e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn decimal_adjust_accumulator_sets_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x9b);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn decimal_adjust_accumulator_reads_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x73);
        e.set_flag(LR35902Flag::Carry, true);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert_eq!(e.read_register(Intel8080Register::A), 0xd3);
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn decimal_adjust_accumulator_reads_half_carry() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        e.set_flag(LR35902Flag::HalfCarry, true);
        e.set_flag(LR35902Flag::Carry, true);
        LR35902InstructionSet::decimal_adjust_accumulator(e);
        assert_eq!(e.read_register(Intel8080Register::A), 0x66);
        assert!(e.read_flag(LR35902Flag::Carry));
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn and_immediate_with_accumulator_sets_zero_flag() {
    instruction_test(|e| {
        LR35902InstructionSet::and_immediate_with_accumulator(e, 0x0);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn and_immediate_with_accumulator_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        LR35902InstructionSet::and_immediate_with_accumulator(e, 0x12);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn exclusive_or_immediate_with_accumulator_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        LR35902InstructionSet::exclusive_or_immediate_with_accumulator(e, 0x0);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn exclusive_or_immediate_with_accumulator_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        LR35902InstructionSet::exclusive_or_immediate_with_accumulator(e, 0x12);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn or_immediate_with_accumulator_sets_zero_flag() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x0);
        LR35902InstructionSet::or_immediate_with_accumulator(e, 0x0);
        assert!(e.read_flag(LR35902Flag::Zero));
    });
}

#[test]
fn or_immediate_with_accumulator_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        LR35902InstructionSet::or_immediate_with_accumulator(e, 0x12);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn complement_accumulator_sets_subtract_flag() {
    instruction_test(|e| {
        LR35902InstructionSet::complement_accumulator(e);
        assert!(e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn complement_accumulator_sets_half_carry() {
    instruction_test(|e| {
        LR35902InstructionSet::complement_accumulator(e);
        assert!(e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn set_carry_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        LR35902InstructionSet::set_carry(e);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn set_carry_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        LR35902InstructionSet::set_carry(e);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn complement_carry_clears_subtract_flag() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::Subtract, true);
        LR35902InstructionSet::complement_carry(e);
        assert!(!e.read_flag(LR35902Flag::Subtract));
    });
}

#[test]
fn complement_carry_clears_half_carry() {
    instruction_test(|e| {
        e.set_flag(LR35902Flag::HalfCarry, true);
        LR35902InstructionSet::complement_carry(e);
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example1() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x45);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0xF3);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example2() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x02);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0x01);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example3() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x80);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0x01);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example4() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x40);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0x01);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(!e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example5() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0x40);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0xFF);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::HalfCarry));
        assert!(e.read_flag(LR35902Flag::Carry));
    });
}

#[test]
fn compare_immediate_with_accumulator_example6() {
    instruction_test(|e| {
        e.set_register(Intel8080Register::A, 0);
        LR35902InstructionSet::compare_immediate_with_accumulator(e, 0x90);
        assert!(e.read_flag(LR35902Flag::Subtract));
        assert!(e.read_flag(LR35902Flag::Carry));
        assert!(!e.read_flag(LR35902Flag::HalfCarry));
    });
}

#[test]
fn increment_register_pair_example1() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::SP, 0x000F);
        LR35902InstructionSet::increment_register_pair(e, Intel8080Register::SP);
        assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0010);
    });
}

#[test]
fn flags_register_keeps_zero_flags_zero() {
    instruction_test(|e| {
        e.set_register_pair(Intel8080Register::PSW, 0xFFFF);
        assert_eq!(e.read_register_pair(Intel8080Register::PSW), 0xFFF0);
    });
}

/*  _____                     _   _
 * | ____|_  _____  ___ _   _| |_(_) ___  _ __
 * |  _| \ \/ / _ \/ __| | | | __| |/ _ \| '_ \
 * | |___ >  <  __/ (__| |_| | |_| | (_) | | | |
 * |_____/_/\_\___|\___|\__,_|\__|_|\___/|_| |_|
 *
 */

impl LR35902Emulator {
    fn crash(&mut self, message: String) {
        self.crash_message = Some(message);
    }

    pub fn crashed(&self) -> bool {
        self.crash_message.is_some()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn run_lr35902_instruction<M: MemoryAccessor>(
        &mut self,
        instruction: LR35902Instruction,
        memory_accessor: &mut M,
    ) {
        let instruction_size = instruction.size();
        let total_duration = instruction.duration();
        let mut ops = InstructionDispatchOps::new(self, memory_accessor);
        instruction.dispatch(&mut ops);
        self.add_cycles(total_duration - (instruction_size * 4));
    }

    fn crash_from_unkown_opcode(&mut self) {
        let pc = self.read_program_counter();
        self.crash(format!("Unknown opcode at address {pc:x}"));
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn load_instruction<M: MemoryAccessor>(&mut self, memory_accessor: &M) {
        debug_assert!(!self.halted);

        let pc = self.read_program_counter();
        let instr = LR35902Instruction::from_memory(memory_accessor, pc);

        if let Some(instr) = &instr {
            self.set_program_counter(pc + instr.size() as u16);
            self.add_cycles(instr.size() * 4);
        }
        self.instr = instr;
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn execute_instruction<M: MemoryAccessor>(&mut self, memory_accessor: &mut M) {
        debug_assert!(!self.halted);

        match self.instr.clone() {
            Some(res) => {
                self.run_lr35902_instruction(res, memory_accessor);
            }
            None => self.crash_from_unkown_opcode(),
        };
    }

    pub fn run_one_instruction<M: MemoryAccessor>(&mut self, memory_accessor: &mut M) {
        self.load_instruction(memory_accessor);
        self.execute_instruction(memory_accessor);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn jump<M: MemoryAccessor>(&mut self, memory_accessor: &mut M, address: u16) {
        let mut ops = InstructionDispatchOps::new(self, memory_accessor);
        LR35902InstructionSet::jump(&mut ops, address)
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub fn push_frame(&mut self, address: u16) {
        self.call_stack.push(address);
    }
}

#[test]
fn emulator_crashes_on_unkown_opcode() {
    let mut e = LR35902Emulator::new();
    let mut memory_accessor = SimpleMemoryAccessor::new();
    memory_accessor.memory[0..1].clone_from_slice(&[0xfc]);
    e.set_program_counter(0);
    e.run_one_instruction(&mut memory_accessor);
    assert_eq!(e.crash_message.unwrap(), "Unknown opcode at address 0");
}

#[cfg(test)]
pub(crate) mod tests;
