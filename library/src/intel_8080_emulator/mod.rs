// Copyright 2017 Remi Bernotavicius

pub mod opcodes;

use crate::collections::HashMap;

use crate::emulator_common::Intel8080Register;
pub use crate::intel_8080_emulator::opcodes::{
    disassemble_8080_rom, Intel8080Instruction, Intel8080InstructionPrinterFactory,
    Intel8080InstructionSet,
};
use crate::util::TwosComplement;

const MAX_ADDRESS: usize = 0xffff;
const ROM_ADDRESS: usize = 0x0100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intel8080Flag {
    // 76543210
    Sign = 0b10000000,
    Zero = 0b01000000,
    AuxiliaryCarry = 0b00010000,
    Parity = 0b00000100,
    Carry = 0b00000001,

    ValidityMask = 0b11010101,
}

/*  _          _
 * | |__   ___| |_ __   ___ _ __ ___
 * | '_ \ / _ \ | '_ \ / _ \ '__/ __|
 * | | | |  __/ | |_) |  __/ |  \__ \
 * |_| |_|\___|_| .__/ \___|_|  |___/
 *              |_|
 */

fn calculate_parity(value: u8) -> bool {
    /*
     * Parity for a give byte can be odd or even.  Odd parity means the byte
     * when represented as binary has an odd number of ones.  Even means the
     * number of ones is even.  The 8080 represents odd parity as 0 (or false)
     * and even parity as 1 (or true).
     */
    value.count_ones() % 2 == 0
}

#[test]
fn calculate_parity_odd_parity() {
    assert_eq!(calculate_parity(0b00000001), false);
}

#[test]
fn calculate_parity_even_parity() {
    assert_eq!(calculate_parity(0b00000011), true);
}

#[test]
fn calculate_parity_zero_is_even_parity() {
    assert_eq!(calculate_parity(0b00000000), true);
}

/*   ___
 *  / _ \ _ __  ___
 * | | | | '_ \/ __|
 * | |_| | |_) \__ \
 *  \___/| .__/|___/
 *       |_|
 */

pub trait Intel8080InstructionSetOps {
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);
    fn read_memory_u16(&self, address: u16) -> u16;
    fn set_memory_u16(&mut self, address: u16, value: u16);
    fn set_flag(&mut self, flag: Intel8080Flag, value: bool);
    fn read_flag(&self, flag: Intel8080Flag) -> bool;
    fn read_program_counter(&self) -> u16;
    fn set_program_counter(&mut self, address: u16);
    fn set_interrupts_enabled(&mut self, state: bool);
    fn get_interrupts_enabled(&self) -> bool;
    fn read_raw_register(&self, index: usize) -> u8;
    fn set_raw_register(&mut self, index: usize, value: u8);
    fn read_raw_register_pair(&self, index: usize) -> u16;
    fn set_raw_register_pair(&mut self, index: usize, value: u16);
    fn add_cycles(&mut self, cycles: u8);
    fn push_frame(&mut self, address: u16);
    fn pop_frame(&mut self);

    /*
     * 8008 Registers are laid out in a specified order in memory. (See enum Intel8080Register for
     * the order). These functions allows you to access two adjacent registers together as one u16.
     * These two registers together are known as a 'register pair.'  The register pairs are
     * referenced by the first in the pair.  The only valid register pairs are:
     *     B:   B and C
     *     D:   D and E
     *     H:   H and L
     *     SP:  Stack Pointer
     *     PSW: A and FLAGS
     */
    fn set_register_pair(&mut self, register: Intel8080Register, value: u16) {
        match register {
            Intel8080Register::B
            | Intel8080Register::D
            | Intel8080Register::H
            | Intel8080Register::SP => {
                self.set_raw_register_pair(register as usize / 2, value);
            }
            Intel8080Register::PSW => {
                self.set_raw_register_pair(Intel8080Register::A as usize / 2, value);
            }
            _ => panic!("Invalid register {:?}", register),
        }
    }

    fn read_register_pair(&self, register: Intel8080Register) -> u16 {
        match register {
            Intel8080Register::B
            | Intel8080Register::D
            | Intel8080Register::H
            | Intel8080Register::SP => self.read_raw_register_pair(register as usize / 2),
            Intel8080Register::PSW => {
                self.read_raw_register_pair(Intel8080Register::A as usize / 2)
            }
            _ => panic!("Invalid register {:?}", register),
        }
    }

    /*
     * The M register is special and represents the byte stored at the memory address stored in the
     * register pair H.
     */
    fn set_register(&mut self, register: Intel8080Register, value: u8) {
        match register {
            Intel8080Register::PSW => panic!("PSW too big"),
            Intel8080Register::SP => panic!("SP too big"),
            Intel8080Register::FLAGS => panic!("Setting FLAGS is illegal"),
            Intel8080Register::M => {
                let address = self.read_register_pair(Intel8080Register::H);
                Intel8080InstructionSetOps::set_memory(self, address, value);
            }
            _ => self.set_raw_register(register as usize, value),
        }
    }

    fn read_register(&self, register: Intel8080Register) -> u8 {
        match register {
            Intel8080Register::PSW => panic!("PSW too big"),
            Intel8080Register::SP => panic!("SP too big"),
            Intel8080Register::M => {
                let address = self.read_register_pair(Intel8080Register::H);
                Intel8080InstructionSetOps::read_memory(self, address)
            }
            _ => self.read_raw_register(register as usize),
        }
    }

    fn update_flags_for_new_value(&mut self, new_value: u8) {
        self.set_flag(Intel8080Flag::Zero, new_value == 0);
        self.set_flag(Intel8080Flag::Sign, new_value & 0b10000000 != 0);
        self.set_flag(Intel8080Flag::Parity, calculate_parity(new_value));
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8 {
        let new_value = value_a.wrapping_add(value_b);
        self.update_flags_for_new_value(new_value);

        if update_carry {
            self.set_flag(Intel8080Flag::Carry, value_b > 0xFF - value_a);
        }

        self.set_flag(
            Intel8080Flag::AuxiliaryCarry,
            value_b & 0x0F > 0x0F - (value_a & 0x0F),
        );

        return new_value;
    }

    fn add_to_register(&mut self, register: Intel8080Register, value: u8, update_carry: bool) {
        let old_value = self.read_register(register);
        let new_value = self.perform_addition(old_value, value, update_carry);
        self.set_register(register, new_value);
    }

    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, ovalue_b: u8) -> u8 {
        let value_b = ovalue_b.twos_complement();
        let new_value = value_a.wrapping_add(value_b);
        self.update_flags_for_new_value(new_value);

        self.set_flag(
            Intel8080Flag::AuxiliaryCarry,
            value_b & 0x0F > 0x0F - (value_a & 0x0F),
        );
        self.set_flag(Intel8080Flag::Carry, value_a < ovalue_b);

        return new_value;
    }

    fn subtract_from_register_using_twos_complement(
        &mut self,
        register: Intel8080Register,
        value: u8,
    ) {
        let old_value = self.read_register(register);
        let new_value = self.perform_subtraction_using_twos_complement(old_value, value);
        self.set_register(register, new_value);
    }

    fn perform_subtraction(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a.wrapping_sub(value_b);
        self.update_flags_for_new_value(new_value);
        self.set_flag(
            Intel8080Flag::AuxiliaryCarry,
            value_b & 0x0F > (value_a & 0x0F),
        );
        return new_value;
    }

    fn subtract_from_register(&mut self, register: Intel8080Register, value: u8) {
        let old_value = self.read_register(register);
        let new_value = self.perform_subtraction(old_value, value);
        self.set_register(register, new_value);
    }

    fn perform_and(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a & value_b;
        self.update_flags_for_new_value(new_value);
        self.set_flag(Intel8080Flag::Carry, false);
        self.set_flag(Intel8080Flag::AuxiliaryCarry, false);
        return new_value;
    }

    fn perform_exclusive_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a ^ value_b;
        self.update_flags_for_new_value(new_value);
        self.set_flag(Intel8080Flag::Carry, false);
        self.set_flag(Intel8080Flag::AuxiliaryCarry, false);
        return new_value;
    }

    fn perform_or(&mut self, value_a: u8, value_b: u8) -> u8 {
        let new_value = value_a | value_b;
        self.update_flags_for_new_value(new_value);
        self.set_flag(Intel8080Flag::Carry, false);
        self.set_flag(Intel8080Flag::AuxiliaryCarry, false);
        return new_value;
    }

    fn perform_rotate_left(&mut self, value: u8) -> u8 {
        self.set_flag(Intel8080Flag::Carry, value & (1u8 << 7) != 0);
        return value.rotate_left(1);
    }

    fn perform_rotate_right(&mut self, value: u8) -> u8 {
        self.set_flag(Intel8080Flag::Carry, value & 1 != 0);
        return value.rotate_right(1);
    }

    fn perform_rotate_left_through_carry(&mut self, value: u8) -> u8 {
        let carry = if self.read_flag(Intel8080Flag::Carry) {
            1
        } else {
            0
        };
        self.set_flag(Intel8080Flag::Carry, (value & (1u8 << 7)) != 0);
        return (value << 1) | carry;
    }

    fn perform_rotate_right_through_carry(&mut self, value: u8) -> u8 {
        let carry = if self.read_flag(Intel8080Flag::Carry) {
            1
        } else {
            0
        };
        self.set_flag(Intel8080Flag::Carry, value & 1 != 0);
        return (value >> 1) | (carry << 7);
    }

    fn push_u16_onto_stack(&mut self, data: u16) {
        let sp = self.read_register_pair(Intel8080Register::SP);
        self.set_memory_u16(sp.wrapping_sub(2), data);
        self.set_register_pair(Intel8080Register::SP, sp.wrapping_sub(2));
    }

    fn pop_u16_off_stack(&mut self) -> u16 {
        let sp = self.read_register_pair(Intel8080Register::SP);
        self.set_register_pair(Intel8080Register::SP, sp.wrapping_add(2));
        self.read_memory_u16(sp)
    }

    fn wait_until_interrupt(&mut self);
}

/*  ___       _       _  ___   ___   ___   ___  _____                 _       _
 * |_ _|_ __ | |_ ___| |( _ ) / _ \ ( _ ) / _ \| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 *  | || '_ \| __/ _ \ |/ _ \| | | |/ _ \| | | |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 *  | || | | | ||  __/ | (_) | |_| | (_) | |_| | |___| | | | | | |_| | | (_| | || (_) | |
 * |___|_| |_|\__\___|_|\___/ \___/ \___/ \___/|_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 */

pub struct Intel8080Emulator<'a> {
    main_memory: [u8; MAX_ADDRESS + 1],
    registers: [u8; Intel8080Register::Count as usize],
    program_counter: u16,
    interrupts_enabled: bool,
    call_table: HashMap<u16, &'a mut dyn FnMut(&mut Intel8080Emulator)>,
    call_stack: Vec<u16>,
}

impl<'a> Intel8080Emulator<'a> {
    pub fn new() -> Intel8080Emulator<'a> {
        let emu = Intel8080Emulator {
            main_memory: [0; MAX_ADDRESS + 1],
            registers: [0; Intel8080Register::Count as usize],
            program_counter: 0,
            interrupts_enabled: true,
            call_table: HashMap::new(),
            call_stack: Vec::new(),
        };

        return emu;
    }

    #[cfg(test)]
    fn new_for_test() -> Intel8080Emulator<'a> {
        let mut emu = Intel8080Emulator::new();
        emu.set_register_pair(Intel8080Register::SP, 0x0400);

        return emu;
    }

    #[cfg(test)]
    fn add_routine<F: FnMut(&mut Intel8080Emulator)>(&mut self, address: u16, func: &'a mut F) {
        self.call_table.insert(address, func);
    }

    #[cfg(test)]
    fn load_rom(&mut self, rom: &[u8]) {
        self.main_memory[ROM_ADDRESS..(ROM_ADDRESS + rom.len())].clone_from_slice(rom);
    }
}

impl<'a> Intel8080InstructionSetOps for Intel8080Emulator<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        self.main_memory[address as usize]
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.main_memory[address as usize] = value;
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        if address == 0xFFFF {
            return self.read_memory(address) as u16;
        }
        let address = address as usize;
        u16::from_le_bytes([self.main_memory[address], self.main_memory[address + 1]])
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        if address == 0xFFFF {
            return self.set_memory(address, (value >> 8) as u8);
        }

        let address = address as usize;
        let bytes = value.to_le_bytes();
        self.main_memory[address] = bytes[0];
        self.main_memory[address + 1] = bytes[1];
    }

    fn set_flag(&mut self, flag: Intel8080Flag, value: bool) {
        if value {
            self.registers[Intel8080Register::FLAGS as usize] |= flag as u8;
        } else {
            self.registers[Intel8080Register::FLAGS as usize] &= !(flag as u8);
        }
    }

    fn read_flag(&self, flag: Intel8080Flag) -> bool {
        self.registers[Intel8080Register::FLAGS as usize] & flag as u8 == flag as u8
    }

    fn read_raw_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    fn set_raw_register(&mut self, index: usize, value: u8) {
        assert!(index != Intel8080Register::FLAGS as usize);
        self.registers[index] = value;
    }

    fn read_raw_register_pair(&self, index: usize) -> u16 {
        let first_byte = index * 2;
        let second_byte = first_byte + 1;
        u16::from_be_bytes([self.registers[first_byte], self.registers[second_byte]])
    }

    fn set_raw_register_pair(&mut self, index: usize, value: u16) {
        let first_byte = index * 2;
        let second_byte = first_byte + 1;

        let bytes = value.to_be_bytes();
        self.registers[first_byte] = bytes[0];
        self.registers[second_byte] = bytes[1];

        if second_byte == Intel8080Register::FLAGS as usize {
            // If we are setting the FLAGS register, we need to force the zero flags to be zero.
            self.registers[Intel8080Register::FLAGS as usize] &= Intel8080Flag::ValidityMask as u8;
        }
    }

    fn read_program_counter(&self) -> u16 {
        self.program_counter
    }

    fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;

        let lookup = self.call_table.remove(&address);
        match lookup {
            Some(func) => {
                func(self);
                self.call_table.insert(address, func);
                self.return_unconditionally();
            }
            None => (),
        }
    }

    fn set_interrupts_enabled(&mut self, state: bool) {
        self.interrupts_enabled = state;
    }

    fn get_interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    fn add_cycles(&mut self, _cycles: u8) {}

    fn push_frame(&mut self, address: u16) {
        self.call_stack.push(address);
    }

    fn pop_frame(&mut self) {
        self.call_stack.pop();
    }

    fn wait_until_interrupt(&mut self) {}
}

/*  _ __ ___  __ _(_)___| |_ ___ _ __   ___  ___| |_     / /
 * | '__/ _ \/ _` | / __| __/ _ \ '__| / __|/ _ \ __|   / /
 * | | |  __/ (_| | \__ \ ||  __/ |    \__ \  __/ |_   / /
 * |_|  \___|\__, |_|___/\__\___|_|    |___/\___|\__| /_/
 *           |___/
 *                     _   _            _
 *  _ __ ___  __ _  __| | | |_ ___  ___| |_ ___
 * | '__/ _ \/ _` |/ _` | | __/ _ \/ __| __/ __|
 * | | |  __/ (_| | (_| | | ||  __/\__ \ |_\__ \
 * |_|  \___|\__,_|\__,_|  \__\___||___/\__|___/
 *
 */

#[test]
fn read_register_pair_b() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x3F);
    e.set_register(Intel8080Register::C, 0x29);

    assert_eq!(
        e.read_register_pair(Intel8080Register::B),
        0x3Fu16 << 8 | 0x29u16
    );
}

#[test]
fn read_and_set_register_pair_d() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::D, 0xBEEF);

    assert_eq!(e.read_register_pair(Intel8080Register::D), 0xBEEF);
}

#[test]
fn set_register_pair_h_and_read_registers() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xABCD);

    assert_eq!(e.read_register(Intel8080Register::H), 0xAB);
    assert_eq!(e.read_register(Intel8080Register::L), 0xCD);
}

#[test]
fn set_register_m() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xf812);
    e.set_register(Intel8080Register::M, 0xAB);

    assert_eq!(e.main_memory[0xf812], 0xAB);
}

#[test]
fn set_register_m_max_address() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, MAX_ADDRESS as u16);
    e.set_register(Intel8080Register::M, 0xD9);

    assert_eq!(e.main_memory[MAX_ADDRESS], 0xD9);
}

#[cfg(test)]
fn add_to_register_test(
    e: &mut Intel8080Emulator,
    register: Intel8080Register,
    starting_value: u8,
    delta: u8,
    update_carry: bool,
) {
    e.set_register(register, starting_value);
    e.add_to_register(register, delta, update_carry);
}

#[test]
fn add_to_register_increments_register() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x99,
        1,
        true, /* update carry */
    );
    assert_eq!(e.read_register(Intel8080Register::B), 0x9A);
}

#[test]
fn add_to_register_increments_memory() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register_pair(Intel8080Register::H, 0x1234);
    add_to_register_test(
        &mut e,
        Intel8080Register::M,
        0x19,
        1,
        true, /* update carry */
    );
    assert_eq!(e.read_register(Intel8080Register::M), 0x1A);
}

#[test]
fn add_to_register_overflows() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xFF,
        1,
        true, /* update carry */
    );
    assert_eq!(e.read_register(Intel8080Register::B), 0x00);
}

#[test]
fn add_to_register_doesnt_set_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x99,
        1,
        true, /* update carry */
    );
    assert!(!e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn add_to_register_update_sets_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xFF,
        1,
        true, /* update carry */
    );
    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn add_to_register_doesnt_set_sign_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x00,
        1,
        true, /* update carry */
    );
    assert!(!e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn add_to_register_sets_sign_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x7f,
        1,
        true, /* update carry */
    );
    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn add_to_register_clears_parity_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Parity, true);
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x00,
        1,
        true, /* update carry */
    );

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn add_to_register_sets_parity_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x02,
        1,
        true, /* update carry */
    );

    // 00000011 -> even parity = true
    assert!(e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x0F,
        1,
        true, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag_when_adding_u8_max() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x0F,
        0xFF,
        true, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_incrementing_high_bits_only() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xA0,
        0x10,
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_overflowing_basic() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xF0,
        0x10,
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_overflowing() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xFF,
        0x10,
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag_when_overflowing() {
    let mut e = Intel8080Emulator::new_for_test();
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xFF,
        0x11,
        true, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_clears_auxiliary_carry_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::AuxiliaryCarry, true);
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x00,
        1,
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_carry_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xFF,
        1,
        true, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn add_to_register_sets_carry_flag_when_adding_u8_max() {
    let mut e = Intel8080Emulator::new_for_test();

    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0xAB,
        0xFF,
        true, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn add_to_register_clears_carry_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x01,
        1,
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn add_to_register_doesnt_update_carry_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x01,
        1,
        false, /* update carry */
    );

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn add_to_register_clears_carry_with_negative_numbers() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    add_to_register_test(
        &mut e,
        Intel8080Register::B,
        0x0C,
        0x0F.twos_complement(),
        true, /* update carry */
    );

    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[cfg(test)]
fn subtract_from_register_test(
    e: &mut Intel8080Emulator,
    register: Intel8080Register,
    starting_value: u8,
    delta: u8,
) {
    e.set_register(register, starting_value);
    e.subtract_from_register(register, delta);
}

#[test]
fn subtract_from_register_underflows() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::C, 0x00, 2);
    assert_eq!(e.read_register(Intel8080Register::C), 0xFE);
}

#[test]
fn subtract_from_register_doesnt_set_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x99, 1);
    assert!(!e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn subtract_from_register_update_sets_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x01, 1);
    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn subtract_from_register_doesnt_set_sign_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x0B, 1);
    assert!(!e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn subtract_from_register_sets_sign_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x00, 1);
    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn subtract_from_register_clears_parity_flag() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Parity, true);
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x02, 1);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn subtract_from_register_sets_parity_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_test(&mut e, Intel8080Register::B, 0x04, 1);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn subtract_from_register_clears_auxiliary_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::AuxiliaryCarry, true);
    subtract_from_register_test(&mut e, Intel8080Register::C, 0x88, 0x03);
    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn subtract_from_register_doesnt_set_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    subtract_from_register_test(&mut e, Intel8080Register::C, 0x88, 4);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[cfg(test)]
fn subtract_from_register_using_twos_complement_test(
    e: &mut Intel8080Emulator,
    register: Intel8080Register,
    starting_value: u8,
    delta: u8,
) {
    e.set_register(register, starting_value);
    e.subtract_from_register_using_twos_complement(register, delta);
}

#[test]
fn subtract_from_register_using_twos_complement_positive_from_positive() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(&mut e, Intel8080Register::C, 0x88, 4);
    assert_eq!(e.read_register(Intel8080Register::C), 0x84);
}

#[test]
fn subtract_from_register_using_twos_complement_positive_from_negative() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(
        &mut e,
        Intel8080Register::C,
        0x09.twos_complement(),
        0x02,
    );
    assert_eq!(
        e.read_register(Intel8080Register::C),
        0x0B.twos_complement()
    );
}

#[test]
fn subtract_from_register_using_twos_complement_negative_from_positive() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(
        &mut e,
        Intel8080Register::C,
        0x09,
        0x02.twos_complement(),
    );
    assert_eq!(e.read_register(Intel8080Register::C), 0x0B);
}

#[test]
fn subtract_from_register_using_twos_complement_negative_from_negative() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(
        &mut e,
        Intel8080Register::C,
        0x09.twos_complement(),
        0x02.twos_complement(),
    );
    assert_eq!(
        e.read_register(Intel8080Register::C),
        0x07.twos_complement()
    );
}

#[test]
fn subtract_from_register_using_twos_complement_negative_from_negative_clears_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    subtract_from_register_using_twos_complement_test(
        &mut e,
        Intel8080Register::C,
        0x08.twos_complement(),
        0x04,
    );
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn subtract_from_register_using_twos_complement_clears_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    subtract_from_register_using_twos_complement_test(&mut e, Intel8080Register::C, 0x08, 0x04);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn subtract_from_register_using_twos_complement_sets_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(&mut e, Intel8080Register::C, 0x05, 0x06);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn subtract_from_register_using_twos_complement_clear_auxiliary_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    subtract_from_register_using_twos_complement_test(&mut e, Intel8080Register::C, 0xA5, 0x04);
    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn subtract_from_register_using_twos_complement_sets_auxiliary_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::AuxiliaryCarry, true);
    subtract_from_register_using_twos_complement_test(&mut e, Intel8080Register::C, 0xA5, 0x06);
    assert!(!e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn push_and_pop_data() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x1234);
    e.push_u16_onto_stack(0xAABB);
    assert_eq!(e.pop_u16_off_stack(), 0xAABB);
    assert_eq!(e.pop_u16_off_stack(), 0x1234);
}

#[test]
fn push_byte_order() {
    let mut e = Intel8080Emulator::new_for_test();
    let sp = e.read_register_pair(Intel8080Register::SP);
    e.push_u16_onto_stack(0x1234);
    assert_eq!(e.read_memory(sp - 2), 0x34);
    assert_eq!(e.read_memory(sp - 1), 0x12);
}

/*  ___           _                   _   _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __  ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
 *
 */

const JUMP_EXTRA_CYCLES: u8 = 4;
const CALL_EXTRA_CYCLES: u8 = 12;
const RETURN_EXTRA_CYCLES: u8 = 12;

impl<I: Intel8080InstructionSetOps> Intel8080InstructionSet for I {
    fn complement_carry(&mut self) {
        let value = self.read_flag(Intel8080Flag::Carry);
        self.set_flag(Intel8080Flag::Carry, !value);
    }

    fn set_carry(&mut self) {
        self.set_flag(Intel8080Flag::Carry, true);
    }

    fn increment_register_or_memory(&mut self, register: Intel8080Register) {
        self.add_to_register(register, 1, false /* update carry */);
    }

    fn decrement_register_or_memory(&mut self, register: Intel8080Register) {
        self.subtract_from_register(register, 1);
    }

    fn complement_accumulator(&mut self) {
        let old_value = self.read_register(Intel8080Register::A);
        self.set_register(Intel8080Register::A, !old_value);
    }

    fn decimal_adjust_accumulator(&mut self) {
        /* The eight-bit hexadecimal number in the accumulator is adjusted to form two four-bit
         * binary-coded decimal digits by the follow two step process:
         *
         * (1) If the least significant four bits of the accumulator represents a number greater
         * than 9, or if the Auxiliary Carry bit is equal to one, the accumulator is incremented
         * by six. Otherwise, no incrementing occurs.
         */

        let accumulator = self.read_register(Intel8080Register::A);
        if accumulator & 0x0F > 0x09 || self.read_flag(Intel8080Flag::AuxiliaryCarry) {
            self.set_register(Intel8080Register::A, accumulator.wrapping_add(0x6));
        }

        /*
         * (2) If the most significant four bits of the accumulator now represent a number greater
         * than 9, or if the normal carry bit is equal to one, the most significant four bits of
         * the accumulator are incremented by six. Otherwise, no incrementing occurs.
         */
        if accumulator > 0x99 || self.read_flag(Intel8080Flag::Carry) {
            let accumulator = self.read_register(Intel8080Register::A);
            self.set_register(Intel8080Register::A, accumulator.wrapping_add(0x60));
            self.set_flag(Intel8080Flag::Carry, true);
        }

        let accumulator = self.read_register(Intel8080Register::A);
        self.set_flag(Intel8080Flag::Zero, accumulator == 0);
    }

    fn no_operation(&mut self) {
        /*
         * Easiest instruction to implement :)
         *
         * ░░░░░░░░░░▄▄▄▄▄▄▄▄▄▄▄▄▄▄░░░░░░░░░░░░░
         * ░░░░░░▄▄█▀▀░░░░░░░░░░░░▀██▄▄▄▄▄░░░░░░
         * ░░░░▄█▀░▄▄█▀▀▀▀░░░░░░░░░░░░▄█████▄░░░
         * ░░▄█▀▄▄█▀░▄▀▀▀▀█▄░░░░░░░░▄▀▀░░▄██▄░░░
         * ░▄▀░▀▀▀░▄▀░░░░░░░█░░░░░░█░░░░░▀▀░█░░░
         * ░█░░░░░░█░▄▄▄░░░░█▄░░░░░█░░░░░░░▄█▄░░
         * █▀░░░░░▀█▄▀█▀░░░░█░░░░░░▀▄▄▄▄▄▄██▄█░░
         * █░░░░░░░░▀▀▄▄▄▄▄▀░░░░░░░░░▄▀░░▄▄░▀█▄░
         * █░░░░░░░░░░░░░░░░░░░░▄░▀▀█▀░█▀░▀▀▄░█░
         * █░░░░░░░░░░░░░░░░░░░░░░░█▀░█▀▀▀▀██░▀█
         * █░░░░░░░░░░░░░░░░░░░░░░░█░▄█▀▀▀▀▀███░
         * █░░░░░░░░░░░░░░░░░░░░░░░█░██░░░░░█░█░
         * ▀█░░░░░░░░░░░░░░░░░░░░░░████░░░░█▀█▀░
         * ░█░░░░░░░░░░░░░░░░░░░░░░███▀░░░░█░█░░
         * ░░█░░░░░░░░░░░░░░░░░░░░░█░█░░░░░█░░█░
         * ░░▀▄░░░░░░░░░░░░░░░░░░░▄▀░█▀▀▀▄░▀▄░█░
         * ░░░▀▄░░░░░░░░░░░░▄▄░░░░█░░█▄▄▄▄█▄█░█░
         * ░░░░▀▄░░░░░░░░░░░░░▀▀▄░█░░█▄██▀▄█░░█░
         * ░░░░░▀█░░░░░░░░░░░░░░░▀█░░█▄░░▀█▀░▄▀░
         *
         */
    }

    fn move_data(&mut self, dest_register: Intel8080Register, src_register: Intel8080Register) {
        let value = self.read_register(src_register);
        self.set_register(dest_register, value);
    }

    fn store_accumulator(&mut self, register_pair: Intel8080Register) {
        let address = self.read_register_pair(register_pair);
        let value = self.read_register(Intel8080Register::A);
        self.set_memory(address, value);
    }

    fn load_accumulator(&mut self, register_pair: Intel8080Register) {
        let address = self.read_register_pair(register_pair);
        let value = self.read_memory(address);
        self.set_register(Intel8080Register::A, value);
    }

    fn add_to_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.add_immediate_to_accumulator(value);
    }

    fn add_to_accumulator_with_carry(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.add_immediate_to_accumulator_with_carry(value);
    }

    fn subtract_from_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.subtract_immediate_from_accumulator(value);
    }

    fn subtract_from_accumulator_with_borrow(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.subtract_immediate_from_accumulator_with_borrow(value);
    }

    fn logical_and_with_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.and_immediate_with_accumulator(value);
    }

    fn logical_exclusive_or_with_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.exclusive_or_immediate_with_accumulator(value);
    }

    fn logical_or_with_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.or_immediate_with_accumulator(value);
    }

    fn compare_with_accumulator(&mut self, register: Intel8080Register) {
        let value = self.read_register(register);
        self.compare_immediate_with_accumulator(value);
    }

    fn rotate_accumulator_left(&mut self) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_rotate_left(value);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn rotate_accumulator_right(&mut self) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_rotate_right(value);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn rotate_accumulator_left_through_carry(&mut self) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_rotate_left_through_carry(value);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn rotate_accumulator_right_through_carry(&mut self) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_rotate_right_through_carry(value);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn push_data_onto_stack(&mut self, register_pair: Intel8080Register) {
        let pair_data = self.read_register_pair(register_pair);
        self.push_u16_onto_stack(pair_data);
    }

    fn pop_data_off_stack(&mut self, register_pair: Intel8080Register) {
        let pair_data = self.pop_u16_off_stack();
        self.set_register_pair(register_pair, pair_data);
    }

    fn double_add(&mut self, register: Intel8080Register) {
        let value = self.read_register_pair(register);
        let old_value = self.read_register_pair(Intel8080Register::H);
        let new_value = old_value.wrapping_add(value);
        self.set_flag(Intel8080Flag::Carry, value > (0xFFFF - old_value));
        self.set_register_pair(Intel8080Register::H, new_value);
    }

    fn increment_register_pair(&mut self, register: Intel8080Register) {
        let old_value = self.read_register_pair(register);
        let new_value = old_value.wrapping_add(1);
        self.set_register_pair(register, new_value);
    }

    fn decrement_register_pair(&mut self, register: Intel8080Register) {
        let old_value = self.read_register_pair(register);
        self.set_register_pair(register, old_value.wrapping_sub(1));
    }

    fn exchange_registers(&mut self) {
        let pair_h = self.read_register_pair(Intel8080Register::H);
        let pair_d = self.read_register_pair(Intel8080Register::D);
        self.set_register_pair(Intel8080Register::H, pair_d);
        self.set_register_pair(Intel8080Register::D, pair_h);
    }

    fn exchange_stack(&mut self) {
        let h_data = self.read_register(Intel8080Register::H);
        let l_data = self.read_register(Intel8080Register::L);
        let sp = self.read_register_pair(Intel8080Register::SP);
        let mem_1 = self.read_memory(sp + 1);
        let mem_2 = self.read_memory(sp);

        self.set_memory(sp + 1, h_data);
        self.set_memory(sp, l_data);
        self.set_register(Intel8080Register::H, mem_1);
        self.set_register(Intel8080Register::L, mem_2);
    }

    fn load_sp_from_h_and_l(&mut self) {
        let h_data = self.read_register_pair(Intel8080Register::H);
        self.set_register_pair(Intel8080Register::SP, h_data);
    }

    fn load_register_pair_immediate(&mut self, register: Intel8080Register, data: u16) {
        self.set_register_pair(register, data);
    }

    fn move_immediate_data(&mut self, dest_register: Intel8080Register, data: u8) {
        self.set_register(dest_register, data);
    }

    fn add_immediate_to_accumulator(&mut self, data: u8) {
        self.add_to_register(Intel8080Register::A, data, true /* update_carry */);
    }

    fn add_immediate_to_accumulator_with_carry(&mut self, data: u8) {
        let carry = self.read_flag(Intel8080Flag::Carry);
        self.add_to_register(Intel8080Register::A, data, true /* update_carry */);

        if carry {
            let carry = self.read_flag(Intel8080Flag::Carry);
            let aux_carry = self.read_flag(Intel8080Flag::AuxiliaryCarry);

            self.add_to_register(Intel8080Register::A, 1, true /* update_carry */);

            if carry {
                self.set_flag(Intel8080Flag::Carry, true)
            };
            if aux_carry {
                self.set_flag(Intel8080Flag::AuxiliaryCarry, true)
            };
        }
    }

    fn subtract_immediate_from_accumulator(&mut self, data: u8) {
        self.subtract_from_register_using_twos_complement(Intel8080Register::A, data);
    }

    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data: u8) {
        let carry = self.read_flag(Intel8080Flag::Carry);
        self.subtract_from_register_using_twos_complement(Intel8080Register::A, data);

        if carry {
            let carry = self.read_flag(Intel8080Flag::Carry);
            let aux_carry = self.read_flag(Intel8080Flag::AuxiliaryCarry);

            self.subtract_from_register_using_twos_complement(Intel8080Register::A, 1);

            if carry {
                self.set_flag(Intel8080Flag::Carry, true)
            };
            if aux_carry {
                self.set_flag(Intel8080Flag::AuxiliaryCarry, true)
            };
        }
    }

    fn and_immediate_with_accumulator(&mut self, data: u8) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_and(value, data);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn exclusive_or_immediate_with_accumulator(&mut self, data: u8) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_exclusive_or(value, data);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn or_immediate_with_accumulator(&mut self, data: u8) {
        let value = self.read_register(Intel8080Register::A);
        let new_value = self.perform_or(value, data);
        self.set_register(Intel8080Register::A, new_value);
    }

    fn compare_immediate_with_accumulator(&mut self, data: u8) {
        let accumulator = self.read_register(Intel8080Register::A);
        self.perform_subtraction_using_twos_complement(accumulator, data);
    }

    fn store_accumulator_direct(&mut self, address: u16) {
        let value = self.read_register(Intel8080Register::A);
        self.set_memory(address, value);
    }

    fn load_accumulator_direct(&mut self, address: u16) {
        let value = self.read_memory(address);
        self.set_register(Intel8080Register::A, value);
    }

    fn store_h_and_l_direct(&mut self, address: u16) {
        let value_l = self.read_register(Intel8080Register::L);
        let value_h = self.read_register(Intel8080Register::H);
        self.set_memory(address, value_l);
        self.set_memory(address + 1, value_h);
    }

    fn load_h_and_l_direct(&mut self, address: u16) {
        let value1 = self.read_memory(address);
        let value2 = self.read_memory(address + 1);
        self.set_register(Intel8080Register::L, value1);
        self.set_register(Intel8080Register::H, value2);
    }

    fn load_program_counter(&mut self) {
        let value = self.read_register_pair(Intel8080Register::H);
        self.set_program_counter(value);
    }

    fn jump(&mut self, address: u16) {
        self.set_program_counter(address);
    }

    fn jump_if_carry(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Carry) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_no_carry(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Carry) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_zero(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Zero) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_not_zero(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Zero) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_minus(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Sign) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_positive(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Sign) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_parity_even(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Parity) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn jump_if_parity_odd(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Parity) {
            self.jump(address);
            self.add_cycles(JUMP_EXTRA_CYCLES);
        }
    }

    fn call(&mut self, address: u16) {
        let pc = self.read_program_counter();
        self.push_u16_onto_stack(pc);
        self.jump(address);
        self.push_frame(pc);
    }

    fn call_if_carry(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Carry) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_no_carry(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Carry) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_zero(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Zero) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_not_zero(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Zero) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_minus(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Sign) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_plus(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Sign) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_parity_even(&mut self, address: u16) {
        if self.read_flag(Intel8080Flag::Parity) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn call_if_parity_odd(&mut self, address: u16) {
        if !self.read_flag(Intel8080Flag::Parity) {
            self.call(address);
            self.add_cycles(CALL_EXTRA_CYCLES);
        }
    }

    fn return_unconditionally(&mut self) {
        let address = self.pop_u16_off_stack();
        self.set_program_counter(address);
        self.pop_frame();
    }

    fn return_if_carry(&mut self) {
        if self.read_flag(Intel8080Flag::Carry) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_no_carry(&mut self) {
        if !self.read_flag(Intel8080Flag::Carry) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_zero(&mut self) {
        if self.read_flag(Intel8080Flag::Zero) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_not_zero(&mut self) {
        if !self.read_flag(Intel8080Flag::Zero) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_minus(&mut self) {
        if self.read_flag(Intel8080Flag::Sign) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_plus(&mut self) {
        if !self.read_flag(Intel8080Flag::Sign) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_parity_even(&mut self) {
        if self.read_flag(Intel8080Flag::Parity) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn return_if_parity_odd(&mut self) {
        if !self.read_flag(Intel8080Flag::Parity) {
            self.return_unconditionally();
            self.add_cycles(RETURN_EXTRA_CYCLES);
        }
    }

    fn rim(&mut self) {
        panic!("rim: Not Implemented")
    }

    fn disable_interrupts(&mut self) {
        self.set_interrupts_enabled(false);
    }

    fn enable_interrupts(&mut self) {
        self.set_interrupts_enabled(true);
    }

    fn input(&mut self, _data1: u8) {
        panic!("input: Not Implemented")
    }

    fn halt(&mut self) {
        self.wait_until_interrupt();
    }

    fn restart(&mut self, implicit_data: u8) {
        assert!(implicit_data <= 7);
        self.call((implicit_data as u16) << 3);
    }

    fn output(&mut self, _data1: u8) {
        panic!("output: Not Implemented")
    }

    fn sim(&mut self) {
        panic!("sim: Not Implemented")
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
fn complement_carry_test_false_to_true() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, false);

    e.complement_carry();

    assert!(e.read_flag(Intel8080Flag::Carry))
}

#[test]
fn complement_carry_test_true_to_false() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);

    e.complement_carry();

    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn set_carry_test_false_to_true() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, false);

    e.set_carry();

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn set_carry_test_true_to_true() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, false);

    e.set_carry();

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decrement_register_or_memory_decrements_register() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x40);
    e.decrement_register_or_memory(Intel8080Register::B);
    assert_eq!(e.read_register(Intel8080Register::B), 0x3F);
}

#[test]
fn decrement_register_or_memory_doesnt_update_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::B, 0x40);
    e.decrement_register_or_memory(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decrement_register_or_memory_updates_auxiliary_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x00);
    e.decrement_register_or_memory(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
}

#[test]
fn complement_accumulator() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10010101);
    e.complement_accumulator();
    assert_eq!(e.read_register(Intel8080Register::A), 0b01101010);
}

#[test]
fn decimal_adjust_accumulator_low_and_high_bits() {
    let mut e = Intel8080Emulator::new_for_test();
    /*
     * Suppose the accumulator contains 9BH, and both carry bits aren't set.
     */
    e.set_register(Intel8080Register::A, 0x9B);
    e.set_flag(Intel8080Flag::Carry, false);

    e.decimal_adjust_accumulator();

    /*
     * The DAA instruction will operate as follows:
     *
     *     (1) Since bits 0-3 are greater than 9, add 6 to the accumulator.  This addition will
     *         generate a carry out of the lower four bits, setting the Auxiliary Carry bit.
     *
     *     Accumulator = 1001 1011 = 0x9B
     *         +6      =      0110
     *                 ___________
     *                   1010 0001 = 0xA1
     *
     *     (2) Since bits 4-7 now are greater than 9, add 6 to these bits. This addition will
     *         generate a carry out of the upper four bits, setting the Carry bit.
     *
     *     Accumulator = 1010 0001 = 0xA1
     *         +6      = 0110
     *                 ___________
     *                   0000 0001
     *
     * The Accumulator will now contain 1, and both carry bits will be set.
     *
     */
    assert_eq!(e.read_register(Intel8080Register::A), 0x01);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decimal_adjust_accumulator_low_bits_increment_only() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0x0F);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Intel8080Register::A), 0x0F + 6);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decimal_adjust_accumulator_high_bits_increment_only() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0xA0);
    e.decimal_adjust_accumulator();

    assert_eq!(
        e.read_register(Intel8080Register::A),
        0xA0u8.wrapping_add(6 << 4)
    );
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decimal_adjust_accumulator_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0x01);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Intel8080Register::A), 0x61);
}

#[test]
fn decimal_adjust_accumulator_auxilliary_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::AuxiliaryCarry, true);
    e.set_register(Intel8080Register::A, 0x01);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Intel8080Register::A), 0x07);
}

#[test]
fn decimal_adjust_accumulator_carry_and_auxilliary_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    e.set_flag(Intel8080Flag::AuxiliaryCarry, true);
    e.set_register(Intel8080Register::A, 0x10);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Intel8080Register::A), 0x76);
}

#[test]
fn no_operation() {
    let mut e = Intel8080Emulator::new_for_test();
    e.no_operation();
}

#[test]
fn move_data_moves_to_register() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0xA0);
    e.move_data(Intel8080Register::E, Intel8080Register::A);

    assert_eq!(e.read_register(Intel8080Register::E), 0xA0);
}

#[test]
fn move_data_same_destination_and_source() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0xA0);
    e.move_data(Intel8080Register::A, Intel8080Register::A);

    assert_eq!(e.read_register(Intel8080Register::A), 0xA0);
}

#[test]
fn move_data_moves_to_memory() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0xBE);
    e.set_register_pair(Intel8080Register::H, 0x2BE9);
    e.move_data(Intel8080Register::M, Intel8080Register::A);

    assert_eq!(e.read_memory(0x2BE9), 0xBE);
}

#[test]
fn store_accumulator_at_address_in_b() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0xAF);
    e.set_register_pair(Intel8080Register::B, 0x3F16);
    e.store_accumulator(Intel8080Register::B);

    assert_eq!(e.read_memory(0x3F16), 0xAF);
}

#[test]
fn store_accumulator_at_address_in_d() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0xAF);
    e.set_register_pair(Intel8080Register::D, 0x3F16);
    e.store_accumulator(Intel8080Register::D);

    assert_eq!(e.read_memory(0x3F16), 0xAF);
}

#[test]
fn load_accumulator_from_address_in_b() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x9388, 0xAF);
    e.set_register_pair(Intel8080Register::B, 0x9388);
    e.load_accumulator(Intel8080Register::B);

    assert_eq!(e.read_register(Intel8080Register::A), 0xAF);
}

#[test]
fn load_accumulator_from_address_in_d() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x9388, 0xAF);
    e.set_register_pair(Intel8080Register::D, 0x9388);
    e.load_accumulator(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0xAF);
}

#[test]
fn add_to_accumulator_from_register() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0xBD);
    e.set_register(Intel8080Register::A, 0x09);
    e.add_to_accumulator(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0xBD + 0x09);
}

#[test]
fn add_to_accumulator_with_carry_from_register_and_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0xBD);
    e.set_register(Intel8080Register::A, 0x09);
    e.set_flag(Intel8080Flag::Carry, true);
    e.add_to_accumulator_with_carry(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0xBD + 0x09 + 1);
}

#[test]
fn add_to_accumulator_with_carry_from_register_and_carry_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0xBD);
    e.set_register(Intel8080Register::A, 0x09);
    e.add_to_accumulator_with_carry(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0xBD + 0x09);
}

#[test]
fn adding_to_accumulator_sets_carry_when_overflowing() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0x01);
    e.set_register(Intel8080Register::A, 0xFF);
    e.add_to_accumulator_with_carry(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0x0);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn adding_to_accumulator_sets_carry_when_overflowing_due_to_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0x00);
    e.set_register(Intel8080Register::A, 0xFF);
    e.set_flag(Intel8080Flag::Carry, true);
    e.add_to_accumulator_with_carry(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0x0);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn adding_to_accumulator_sets_carry_when_overflowing_due_to_carry_and_augend() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::D, 0xFF);
    e.set_register(Intel8080Register::A, 0x0);
    e.set_flag(Intel8080Flag::Carry, true);
    e.add_to_accumulator_with_carry(Intel8080Register::D);

    assert_eq!(e.read_register(Intel8080Register::A), 0x0);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn subtract_from_accumulator_from_register() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0xF2);
    e.set_register(Intel8080Register::A, 0x1A);
    e.subtract_from_accumulator(Intel8080Register::B);

    assert_eq!(
        e.read_register(Intel8080Register::A),
        0x1Au8.wrapping_sub(0xF2)
    );
}

#[test]
fn subtract_from_accumulator_with_borrow_condition_flags_get_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::L, 0x02);
    e.set_register(Intel8080Register::A, 0x04);
    e.set_flag(Intel8080Flag::Carry, true);
    e.subtract_from_accumulator_with_borrow(Intel8080Register::L);

    assert_eq!(e.read_register(Intel8080Register::A), 0x01);
    assert!(!e.read_flag(Intel8080Flag::Zero));
    assert!(!e.read_flag(Intel8080Flag::Carry));
    assert!(e.read_flag(Intel8080Flag::AuxiliaryCarry));
    assert!(!e.read_flag(Intel8080Flag::Parity));
    assert!(!e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn subtract_from_accumulator_with_borrow_and_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0xF2);
    e.set_register(Intel8080Register::A, 0x1A);
    e.set_flag(Intel8080Flag::Carry, true);
    e.subtract_from_accumulator_with_borrow(Intel8080Register::B);

    assert_eq!(
        e.read_register(Intel8080Register::A),
        0x1Au8.wrapping_sub(0xF2 + 1)
    );
}

#[test]
fn subtract_from_accumulator_with_borrow_and_carry_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0xF2);
    e.set_register(Intel8080Register::A, 0x1A);
    e.subtract_from_accumulator_with_borrow(Intel8080Register::B);

    assert_eq!(
        e.read_register(Intel8080Register::A),
        0x1Au8.wrapping_sub(0xF2)
    );
}

#[cfg(test)]
fn subtract_with_borrow_carry_test(a: u8, b: u8, borrow: bool) {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, a);
    e.set_register(Intel8080Register::B, b);
    e.set_flag(Intel8080Flag::Carry, borrow);
    e.subtract_from_accumulator_with_borrow(Intel8080Register::B);

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn subtract_from_accumulator_with_borrow_sets_carry() {
    subtract_with_borrow_carry_test(0x1, 0x1, true);
    subtract_with_borrow_carry_test(0x0, 0x0, true);
    subtract_with_borrow_carry_test(0x0, 0x1, false);
    subtract_with_borrow_carry_test(0xFF, 0x1.twos_complement(), true);
}

#[test]
fn logical_and_with_accumulator() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b00100110);
    e.set_register(Intel8080Register::A, 0b01000111);
    e.logical_and_with_accumulator(Intel8080Register::B);

    assert_eq!(e.read_register(Intel8080Register::A), 0b00000110);
}

#[test]
fn logical_and_with_accumulator_sets_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x0);
    e.set_register(Intel8080Register::A, 0x4F);
    e.logical_and_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn logical_and_with_accumulator_sets_parity() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b11011100);
    e.set_register(Intel8080Register::A, 0b11000000);
    e.logical_and_with_accumulator(Intel8080Register::B);

    assert!(e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn logical_and_with_accumulator_sets_sign() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b10000000);
    e.set_register(Intel8080Register::A, 0b10000000);
    e.logical_and_with_accumulator(Intel8080Register::B);

    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn logical_and_with_accumulator_clears_the_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.logical_and_with_accumulator(Intel8080Register::B);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn logical_exclusive_or_with_accumulator() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b00100110);
    e.set_register(Intel8080Register::A, 0b01000111);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::B);

    assert_eq!(e.read_register(Intel8080Register::A), 0b01100001);
}

#[test]
fn logical_exclusive_or_with_accumulator_zeros_accumulator() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b01000111);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::A);

    assert_eq!(e.read_register(Intel8080Register::A), 0x0);
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0xFF);
    e.set_register(Intel8080Register::A, 0xFF);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_parity() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b01001001);
    e.set_register(Intel8080Register::A, 0b00000001);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_sign() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b10000000);
    e.set_register(Intel8080Register::A, 0b00100100);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.logical_exclusive_or_with_accumulator(Intel8080Register::B);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn logical_or_with_accumulator() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b00100110);
    e.set_register(Intel8080Register::A, 0b01000111);
    e.logical_or_with_accumulator(Intel8080Register::B);

    assert_eq!(e.read_register(Intel8080Register::A), 0b01100111);
}

#[test]
fn logical_or_with_accumulator_sets_zero_flag() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x00);
    e.set_register(Intel8080Register::A, 0x00);
    e.logical_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn logical_or_with_accumulator_sets_parity() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b01001011);
    e.set_register(Intel8080Register::A, 0b00000001);
    e.logical_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Parity));
}

#[test]
fn logical_or_with_accumulator_sets_sign() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0b10000000);
    e.set_register(Intel8080Register::A, 0b00100100);
    e.logical_or_with_accumulator(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn logical_or_with_accumulator_clears_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.logical_or_with_accumulator(Intel8080Register::B);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn compare_with_accumulator_doesnt_affect_register_values() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x05);
    e.compare_with_accumulator(Intel8080Register::E);

    assert_eq!(e.read_register(Intel8080Register::A), 0x0A);
    assert_eq!(e.read_register(Intel8080Register::E), 0x05);
}

#[test]
fn compare_with_accumulator_clears_carry() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x05);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn compare_with_accumulator_sets_carry() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x0B);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn compare_with_accumulator_clears_carry_when_signs_differ() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0xF5);
    e.set_register(Intel8080Register::E, 0x00);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn compare_with_accumulator_clears_sets_when_signs_differ() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0x00);
    e.set_register(Intel8080Register::E, 0xF5);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn compare_with_accumulator_clears_zero_when_compared_with_zero() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0xF5);
    e.set_register(Intel8080Register::E, 0x00);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(!e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn compare_with_accumulator_clears_zero() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Zero, true);

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x05);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(!e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn compare_with_accumulator_sets_zero() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x0A);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(e.read_flag(Intel8080Flag::Zero));
}

#[test]
fn compare_with_accumulator_clears_sign() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_flag(Intel8080Flag::Sign, true);
    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x01);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(!e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn compare_with_accumulator_sets_sign() {
    let mut e = Intel8080Emulator::new_for_test();

    e.set_register(Intel8080Register::A, 0x0A);
    e.set_register(Intel8080Register::E, 0x0B);
    e.compare_with_accumulator(Intel8080Register::E);

    assert!(e.read_flag(Intel8080Flag::Sign));
}

#[test]
fn rotate_accumulator_left_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_left();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01001111);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_left_carry_cleared() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b00100111);
    e.rotate_accumulator_left();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01001110);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_carry_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_right();

    assert_eq!(e.read_register(Intel8080Register::A), 0b11010011);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_carry_cleared() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b10100110);
    e.rotate_accumulator_right();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01010011);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_set_to_reset() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b01100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b11001111);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_stays_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01001111);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_reset_to_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01001110);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_stays_reset() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b00100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01001110);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_set_to_unset() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b10100110);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b11010011);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_stays_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b11010011);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_reset_to_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10100111);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01010011);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_stays_reset() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0b10100110);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Intel8080Register::A), 0b01010011);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn push_when_sp_zero() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::SP, 0);
    e.set_register_pair(Intel8080Register::B, 0xFBEE);
    e.push_data_onto_stack(Intel8080Register::B);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xFFFE);
}

#[test]
fn pop_when_sp_ffff() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::SP, 0xFFFF);
    e.pop_data_off_stack(Intel8080Register::B);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0001);
}

#[test]
fn push_register_pair_onto_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::B, 0x34);
    e.set_register(Intel8080Register::C, 0xA7);
    e.set_register_pair(Intel8080Register::SP, 0x20);
    e.push_data_onto_stack(Intel8080Register::B);
    assert_eq!(e.read_memory(0x20 - 1), 0x34);
    assert_eq!(e.read_memory(0x20 - 2), 0xA7);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x20 - 2);
}

#[test]
fn push_psw_onto_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::PSW, 0x34FF);
    e.set_register_pair(Intel8080Register::SP, 0x20);
    e.push_data_onto_stack(Intel8080Register::PSW);
    assert_eq!(e.read_memory(0x20 - 1), 0x34);
    assert_eq!(e.read_memory(0x20 - 2), 0xD5);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x20 - 2);
}

#[test]
fn pop_register_pair_from_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x20, 0xF2);
    e.set_memory(0x20 + 1, 0x78);
    e.set_register_pair(Intel8080Register::SP, 0x20);
    e.pop_data_off_stack(Intel8080Register::B);
    assert_eq!(e.read_register_pair(Intel8080Register::B), 0x78F2);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x20 + 2);
}

#[test]
fn pop_psw_from_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x20, 0xFF);
    e.set_memory(0x20 + 1, 0x78);
    e.set_register_pair(Intel8080Register::SP, 0x20);
    e.pop_data_off_stack(Intel8080Register::PSW);
    assert_eq!(e.read_register_pair(Intel8080Register::PSW), 0x78D5);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x20 + 2);
}

#[test]
fn double_add() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xABCD);
    e.set_register_pair(Intel8080Register::B, 0x1001);
    e.double_add(Intel8080Register::B);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0xBBCE);
}

#[test]
fn double_add_updates_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFBCD);
    e.set_register_pair(Intel8080Register::B, 0x1000);
    e.double_add(Intel8080Register::B);
    assert!(e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn increment_register_pair() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFBCD);
    e.increment_register_pair(Intel8080Register::H);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0xFBCE);
}

#[test]
fn increment_register_pair_doesnt_update_carry() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFFFF);
    e.increment_register_pair(Intel8080Register::H);
    assert!(!e.read_flag(Intel8080Flag::Carry));
}

#[test]
fn decrement_register_pair() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFBCD);
    e.decrement_register_pair(Intel8080Register::H);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0xFBCC);
}

#[test]
fn exchange_registers() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xFBCD);
    e.set_register_pair(Intel8080Register::D, 0x1122);
    e.exchange_registers();
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1122);
    assert_eq!(e.read_register_pair(Intel8080Register::D), 0xFBCD);
}

#[test]
fn exchange_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x1234);
    e.set_memory(0x1234, 0xBB);
    e.set_memory(0x1235, 0xCC);
    e.set_register_pair(Intel8080Register::H, 0xDDEE);

    e.exchange_stack();

    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x1234);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0xCCBB);
    assert_eq!(e.read_memory(0x1234), 0xEE);
    assert_eq!(e.read_memory(0x1235), 0xDD);
}

#[test]
fn load_sp_from_h_and_l() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0xDDEE);
    e.load_sp_from_h_and_l();
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0xDDEE);
}

#[test]
fn load_register_pair_immediate() {
    let mut e = Intel8080Emulator::new_for_test();
    e.load_register_pair_immediate(Intel8080Register::B, 0x1234);
    assert_eq!(e.read_register_pair(Intel8080Register::B), 0x1234);
}

#[test]
fn move_immediate_data() {
    let mut e = Intel8080Emulator::new_for_test();
    e.move_immediate_data(Intel8080Register::E, 0xF1);
    assert_eq!(e.read_register(Intel8080Register::E), 0xF1);
}

#[test]
fn store_accumulator_direct() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register(Intel8080Register::A, 0x8F);
    e.store_accumulator_direct(0x1234);
    assert_eq!(e.read_memory(0x1234), 0x8F);
}

#[test]
fn load_accumulator_direct() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x1234, 0x8F);
    e.load_accumulator_direct(0x1234);
    assert_eq!(e.read_register(Intel8080Register::A), 0x8F);
}

#[test]
fn store_h_and_l_direct() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0x1234);
    e.store_h_and_l_direct(0x8888);
    assert_eq!(e.read_memory(0x8889), 0x12);
    assert_eq!(e.read_memory(0x8888), 0x34);
}

#[test]
fn load_h_and_l_direct() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_memory(0x8889, 0x12);
    e.set_memory(0x8888, 0x34);
    e.load_h_and_l_direct(0x8888);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1234);
}

#[test]
fn store_and_load_h_and_l() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0x1234);
    e.store_h_and_l_direct(0x8888);
    e.set_register_pair(Intel8080Register::H, 0x0);
    e.load_h_and_l_direct(0x8888);
    assert_eq!(e.read_register_pair(Intel8080Register::H), 0x1234);
}

#[test]
fn load_program_counter() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::H, 0x1234);
    e.load_program_counter();
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_carry_when_carry_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.jump_if_carry(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_carry_when_carry_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_carry(0x1234);
    assert_eq!(e.program_counter, 0x0);
}
#[test]
fn jump_if_no_carry_when_carry_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.jump_if_no_carry(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn jump_if_no_carry_when_carry_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_no_carry(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_zero_when_zero_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Zero, true);
    e.jump_if_zero(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_zero_when_zero_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_zero(0x1234);
    assert_eq!(e.program_counter, 0x0);
}
#[test]
fn jump_if_not_zero_when_zero_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Zero, true);
    e.jump_if_not_zero(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn jump_if_not_zero_when_zero_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_not_zero(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_minus_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.jump_if_minus(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_minus_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_minus(0x1234);
    assert_eq!(e.program_counter, 0x0);
}
#[test]
fn jump_if_positive_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.jump_if_positive(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn jump_if_positive_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_positive(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_parity_even_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.jump_if_parity_even(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn jump_if_parity_even_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_parity_even(0x1234);
    assert_eq!(e.program_counter, 0x0);
}
#[test]
fn jump_if_parity_odd_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.jump_if_parity_odd(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn jump_if_parity_odd_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.jump_if_parity_odd(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call() {
    let mut e = Intel8080Emulator::new_for_test();
    e.program_counter = 0xFF88;
    e.call(0x1234);
    assert_eq!(e.program_counter, 0x1234);
    assert_eq!(e.pop_u16_off_stack(), 0xFF88);
}

#[test]
fn call_if_carry_when_carry_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.call_if_carry(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call_if_carry_when_carry_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_carry(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_zero_when_zero_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Zero, true);
    e.call_if_zero(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call_if_zero_when_zero_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_zero(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_minus_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.call_if_minus(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call_if_minus_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_minus(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_plus_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.call_if_plus(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_plus_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_plus(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call_if_parity_even_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.call_if_parity_even(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn call_if_parity_even_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_parity_even(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_parity_odd_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.call_if_parity_odd(0x1234);
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn call_if_parity_odd_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.call_if_parity_odd(0x1234);
    assert_eq!(e.program_counter, 0x1234);
}

#[test]
fn return_unconditionally() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_register_pair(Intel8080Register::SP, 0x0400);
    e.push_u16_onto_stack(0x22FF);
    e.return_unconditionally();
    assert_eq!(e.program_counter, 0x22FF);
    assert_eq!(e.read_register_pair(Intel8080Register::SP), 0x0400);
}

#[test]
fn return_if_carry_when_carry_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_carry();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_carry_when_carry_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_carry();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_no_carry_when_carry_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_no_carry();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_no_carry_when_carry_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_no_carry();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_zero_when_zero_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Zero, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_zero();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_zero_when_zero_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_zero();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_not_zero_when_zero_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Zero, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_not_zero();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_not_zero_when_zero_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_not_zero();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_minus_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_minus();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_minus_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_minus();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_plus_when_sign_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Sign, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_plus();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_plus_when_sign_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_plus();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_parity_even_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_parity_even();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn return_if_parity_even_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_parity_even();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_parity_odd_when_parity_is_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Parity, true);
    e.push_u16_onto_stack(0x22FF);
    e.return_if_parity_odd();
    assert_eq!(e.program_counter, 0x0);
}

#[test]
fn return_if_parity_odd_when_parity_is_not_set() {
    let mut e = Intel8080Emulator::new_for_test();
    e.push_u16_onto_stack(0x22FF);
    e.return_if_parity_odd();
    assert_eq!(e.program_counter, 0x22FF);
}

#[test]
fn restart_one() {
    let mut e = Intel8080Emulator::new_for_test();
    e.restart(1);
    assert_eq!(e.program_counter, 1 << 3);
}

#[test]
fn restart_seven() {
    let mut e = Intel8080Emulator::new_for_test();
    e.restart(7);
    assert_eq!(e.program_counter, 7 << 3);
}

#[test]
fn disable_interrupts() {
    let mut e = Intel8080Emulator::new_for_test();
    e.disable_interrupts();
    assert!(!e.interrupts_enabled);
}

#[test]
fn enable_interrupts() {
    let mut e = Intel8080Emulator::new_for_test();
    e.interrupts_enabled = false;
    e.enable_interrupts();
    assert!(e.interrupts_enabled);
}

#[test]
fn call_updates_call_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_program_counter(0x1111);
    e.call(0x2222);
    e.call(0x3333);
    e.call(0x4444);

    assert_eq!(e.call_stack, vec![0x1111, 0x2222, 0x3333]);
}

#[test]
fn return_updates_call_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_program_counter(0x1111);
    e.call(0x2222);
    e.call(0x3333);
    e.call(0x4444);
    e.return_unconditionally();

    assert_eq!(e.call_stack, vec![0x1111, 0x2222]);
}

#[test]
fn if_carry_updates_call_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, true);

    e.set_program_counter(0x1111);

    e.call_if_carry(0x3333);
    assert_eq!(e.call_stack, vec![0x1111]);

    e.return_if_carry();
    assert_eq!(e.call_stack, vec![]);
}

#[test]
fn if_no_carry_updates_call_stack() {
    let mut e = Intel8080Emulator::new_for_test();
    e.set_flag(Intel8080Flag::Carry, false);

    e.set_program_counter(0x1111);

    e.call_if_no_carry(0x3333);
    assert_eq!(e.call_stack, vec![0x1111]);

    e.return_if_no_carry();
    assert_eq!(e.call_stack, vec![]);
}

/*  _____                     _   _
 * | ____|_  _____  ___ _   _| |_(_) ___  _ __
 * |  _| \ \/ / _ \/ __| | | | __| |/ _ \| '_ \
 * | |___ >  <  __/ (__| |_| | |_| | (_) | | | |
 * |_____/_/\_\___|\___|\__,_|\__|_|\___/|_| |_|
 *
 */

impl<'a> Intel8080Emulator<'a> {
    pub fn run_one_instruction(&mut self) {
        let pc = self.program_counter as usize;
        let maybe_instr = Intel8080Instruction::from_reader(&self.main_memory[pc..]).unwrap();
        let instruction = match maybe_instr {
            Some(res) => res,
            None => panic!("Unknown Opcode {}", self.main_memory[pc]),
        };

        self.program_counter += instruction.size() as u16;
        instruction.dispatch(self);
    }

    pub fn run(&mut self) {
        self.program_counter = ROM_ADDRESS as u16;
        while self.program_counter != 0 {
            self.run_one_instruction();
        }
    }
}

/*  ____  _                             _   _        ____   ___  __  __
 * |  _ \(_) __ _  __ _ _ __   ___  ___| |_(_) ___  |  _ \ / _ \|  \/  |
 * | | | | |/ _` |/ _` | '_ \ / _ \/ __| __| |/ __| | |_) | | | | |\/| |
 * | |_| | | (_| | (_| | | | | (_) \__ \ |_| | (__  |  _ <| |_| | |  | |
 * |____/|_|\__,_|\__, |_| |_|\___/|___/\__|_|\___| |_| \_\\___/|_|  |_|
 *                |___/
 *
 */

#[cfg(test)]
use std::{fs::File, str};

#[cfg(test)]
use crate::io::{self, Read};

#[cfg(test)]
fn console_print(e: &mut Intel8080Emulator, stream: &mut dyn io::Write) {
    match e.read_register(Intel8080Register::C) {
        9 => {
            let mut msg_addr = e.read_register_pair(Intel8080Register::D) as usize;
            while e.main_memory[msg_addr] != '$' as u8 {
                write!(stream, "{}", e.main_memory[msg_addr] as char).unwrap();
                msg_addr += 1;
            }
        }
        2 => {
            write!(stream, "{}", e.read_register(Intel8080Register::E) as char).unwrap();
        }
        op => panic!("{} unknown print operation", op),
    }
}

/*
 * This test runs a ROM entitled "MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC" stored in
 * cpudiag.bin.  It tests most the instructions, and when an instruction doesn't behave the way it
 * is suppose to, it will print out the address where it failed.
 */
#[test]
fn cpu_diagnostic_8080() {
    // Load up the ROM
    let mut rom: Vec<u8> = vec![];
    {
        let mut file = File::open("cpudiag.bin").unwrap();
        file.read_to_end(&mut rom).unwrap();
    }

    let mut console_buffer: Vec<u8> = vec![];
    let mut console_print_closure =
        |e: &mut Intel8080Emulator| console_print(e, &mut console_buffer);

    let mut emulator = Intel8080Emulator::new();
    emulator.load_rom(&rom);
    // The program write to the console via a routine at address 0x0005
    emulator.add_routine(0x0005, &mut console_print_closure);
    emulator.run();
    let ascii_output = str::from_utf8(&console_buffer).unwrap();

    // When we see this string it means the program succeeded.  When it fails, we see
    // 'CPU HAS FAILED! EXIT=xxxx' where xxxx is the address it failed at.
    assert_eq!(ascii_output, "\u{c}\r\n CPU IS OPERATIONAL");
}
