use std::cmp;
use emulator_8080::{Register8080, Emulator8080, InstructionSetOps, Flag8080};

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

    fn read_flag(&mut self, flag: Flag8080) -> bool
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

    fn read_program_counter(&mut self) -> u16
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

impl<'a> EmulatorLR35902<'a> {
    fn run(&mut self)
    {
        self.e8080.run();
    }
}

pub fn run_emulator(rom: &[u8])
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&rom);
    e.run();
}
