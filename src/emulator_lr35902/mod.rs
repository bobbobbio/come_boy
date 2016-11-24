mod opcodes;

use std::cmp;
use emulator_common::Register8080;
use emulator_8080::{Emulator8080, InstructionSetOps, Flag8080};
pub use emulator_lr35902::opcodes::disassemble_lr35902_rom;
use emulator_lr35902::opcodes::{
    get_lr35902_instruction, dispatch_lr35902_instruction, InstructionSetLR35902};

const ROM_ADDRESS: usize = 0x0100;
const LCD_ADDRESS: usize = 0x8000;

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
        let end = cmp::min(ROM_ADDRESS + rom.len(), LCD_ADDRESS);
        self.e8080.main_memory[ROM_ADDRESS..end].clone_from_slice(rom);
    }
}

impl<'a> InstructionSetOps for EmulatorLR35902<'a> {
    fn read_memory(&self, address: u16) -> u8 {
        self.e8080.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.e8080.set_memory(address, value);
    }

    fn set_flag(&mut self, flag: Flag8080, value: bool)
    {
        self.e8080.set_flag(flag, value);
    }

    fn read_flag(&self, flag: Flag8080) -> bool
    {
        self.e8080.read_flag(flag)
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

    fn update_flags_for_new_value(&mut self, new_value: u8)
    {
        self.e8080.update_flags_for_new_value(new_value);
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8
    {
        self.e8080.perform_addition(value_a, value_b, update_carry)
    }

    fn perform_subtraction_using_twos_complement(&mut self, value_a: u8, value_b: u8) -> u8
    {
        self.e8080.perform_subtraction_using_twos_complement(value_a, value_b)
    }

    fn add_to_register(&mut self, register: Register8080, value: u8, update_carry: bool)
    {
        self.e8080.add_to_register(register, value, update_carry);
    }

    fn add_to_register_pair(&mut self, register: Register8080, value: u16, update_carry: bool)
    {
        self.e8080.add_to_register_pair(register, value, update_carry);
    }

    fn subtract_from_register_pair(&mut self, register: Register8080, value: u16)
    {
        self.e8080.subtract_from_register_pair(register, value);
    }

    fn subtract_from_register(&mut self, register: Register8080, value: u8)
    {
        self.e8080.subtract_from_register(register, value);
    }

    fn subtract_from_register_using_twos_complement(&mut self, register: Register8080, value: u8)
    {
        self.e8080.subtract_from_register_using_twos_complement(register, value);
    }

    fn read_program_counter(&self) -> u16
    {
        self.e8080.read_program_counter()
    }

    fn set_program_counter(&mut self, address: u16)
    {
        self.e8080.set_program_counter(address);
    }

    fn set_interrupt_state(&mut self, value: bool)
    {
        self.e8080.set_interrupt_state(value);
    }
}

impl<'a> InstructionSetLR35902 for EmulatorLR35902<'a> {
    fn move_and_increment_m(&mut self, _dest: Register8080, _src: Register8080)
    {
        unimplemented!();
    }
    fn move_and_decrement_m(&mut self, _dest: Register8080, _src: Register8080)
    {
        unimplemented!();
    }
    fn store_accumulator_direct(&mut self, _address: u16)
    {
        unimplemented!();
    }
    fn store_sp_plus_immediate(&mut self, _data: u8)
    {
        unimplemented!();
    }
    fn add_immediate_to_sp(&mut self, _data: u8)
    {
        unimplemented!();
    }
    fn store_accumulator_direct_one_byte(&mut self, _address: u8)
    {
        unimplemented!();
    }
    fn load_accumulator_direct_one_byte(&mut self, _address: u8)
    {
        unimplemented!();
    }
    fn return_and_enable_interrupts(&mut self)
    {
        unimplemented!();
    }
    fn halt_until_button_press(&mut self)
    {
        unimplemented!();
    }
    fn jump_after_adding(&mut self, _n: u8)
    {
        unimplemented!();
    }
    fn jump_after_adding_if_zero(&mut self, _n: u8)
    {
        unimplemented!();
    }
    fn jump_after_adding_if_not_zero(&mut self, _n: u8)
    {
        unimplemented!();
    }
    fn jump_after_adding_if_carry(&mut self, _n: u8)
    {
        unimplemented!();
    }
    fn jump_after_adding_if_not_carry(&mut self, _n: u8)
    {
        unimplemented!();
    }
    fn store_sp_direct(&mut self, _address: u16)
    {
        unimplemented!();
    }
    fn load_accumulator_direct(&mut self, _address: u16)
    {
        unimplemented!();
    }
    fn reset_bit(&mut self, _bit: u8, _register: Register8080)
    {
        unimplemented!();
    }
    fn shift_register_right_with_zero(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn swap_register(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn rotate_register_left(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn shift_register_right(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn rotate_register_left_through_carry(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn rotate_register_right(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn set_bit(&mut self, _bit: u8, _register: Register8080)
    {
        unimplemented!();
    }
    fn shift_register_left(&mut self, _register: Register8080)
    {
        unimplemented!();
    }
    fn test_bit(&mut self, _bit: u8, _register: Register8080)
    {
        unimplemented!();
    }
    fn rotate_register_right_through_carry(&mut self, _register: Register8080)
    {
        unimplemented!();
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
        self.set_program_counter(ROM_ADDRESS as u16);
        while self.read_program_counter() != 0 {
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
