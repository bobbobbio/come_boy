pub mod opcodes;

use emulator8080::opcodes::opcode_gen::{InstructionSet8080, Register8080, dispatch_opcode};

const MAX_ADDRESS: u16 = 0xffff;
const ROM_ADDRESS: u16 = 0x0100;

#[derive(Debug,Clone,Copy)]
enum Flag8080 {
    Sign = 0x80,           // Bit 7
    Zero = 0x40,           // Bit 6
    AuxiliaryCarry = 0x10, // Bit 4
    Parity = 0x4,          // Bit 2
    Carry = 0x1,           // Bit 0
}

struct Emulator8080 {
    main_memory: [u8; MAX_ADDRESS as usize + 1],
    registers: [u8; Register8080::Count as usize]
}

impl Emulator8080 {
    fn new(rom: &[u8]) -> Emulator8080
    {
        let mut emu = Emulator8080 {
            main_memory: [0; MAX_ADDRESS as usize + 1],
            registers: [0; Register8080::Count as usize]
        };

        emu.main_memory[
            ROM_ADDRESS as usize..(ROM_ADDRESS as usize + rom.len())].clone_from_slice(rom);

        return emu;
    }

    fn set_flag(&mut self, flag: Flag8080, value: bool)
    {
        if value {
            self.registers[Register8080::FLAGS as usize] |= flag as u8;
        } else {
            self.registers[Register8080::FLAGS as usize] &= !(flag as u8);
        }
    }

    fn flip_flag(&mut self, flag: Flag8080)
    {
        self.registers[Register8080::FLAGS as usize] ^= flag as u8;
    }

    #[cfg(test)]
    fn read_flag(&mut self, flag: Flag8080) -> bool
    {
        self.registers[Register8080::FLAGS as usize] & (flag as u8) == flag as u8
    }

    #[cfg(test)]
    fn clear_all_flags(&mut self)
    {
        self.registers[Register8080::FLAGS as usize] = 0;
    }

    fn set_register(&mut self, register: Register8080, value: u8)
    {
        self.registers[register as usize] = value;
    }

    fn read_register(&self, register: Register8080) -> u8
    {
        self.registers[register as usize]
    }
}

fn calculate_parity(value: u8) -> bool
{
    let mut mask = 0x1;
    let mut parity = true;
    while mask != 0x80 {
        if (value & mask) != 0 {
            parity = !parity;
        }
        mask = mask << 1;
    }
    parity
}

#[test]
fn calculate_parity_odd_parity()
{
    assert_eq!(calculate_parity(0x01), false);
}

#[test]
fn calculate_parity_even_parity()
{
    assert_eq!(calculate_parity(0x03), true);
}

#[test]
fn calculate_parity_zero_is_even_parity()
{
    assert_eq!(calculate_parity(0x00), true);
}

impl InstructionSet8080 for Emulator8080 {
    fn complement_carry(&mut self)
    {
        self.flip_flag(Flag8080::Carry);
    }
    fn set_carry(&mut self)
    {
        self.set_flag(Flag8080::Carry, true);
    }
    fn increment_register_or_memory(&mut self, register: Register8080)
    {
        let new_value;
        if self.read_register(register) == 0xFF {
            new_value = 0;
        } else {
            new_value = self.read_register(register) + 1;
        }
        self.set_register(register, new_value);

        self.set_flag(Flag8080::Zero, new_value == 0);
        self.set_flag(Flag8080::Sign, new_value & 0x80 != 0);
        self.set_flag(Flag8080::Parity, calculate_parity(new_value));
        self.set_flag(Flag8080::AuxiliaryCarry, false);
    }

    fn subtract_from_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn return_if_not_zero(&mut self)
    {
        panic!("Not Implemented")
    }
    fn add_immediate_to_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn pop_data_off_stack(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn add_to_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn jump_if_parity_even(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn move_data(&mut self, _register1: Register8080, _register2: Register8080)
    {
        panic!("Not Implemented")
    }
    fn double_add(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn or_immediate_with_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn call_if_carry(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn jump(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn logical_or(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn rim(&mut self)
    {
        panic!("Not Implemented")
    }
    fn call_if_parity_even(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn jump_if_positive(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn jump_if_zero(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn no_instruction(&mut self)
    {
        panic!("Not Implemented")
    }
    fn disable_interrupts(&mut self)
    {
        panic!("Not Implemented")
    }
    fn compare_with_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn call_if_not_zero(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn call_if_parity_odd(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn subtract_immediate_from_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        panic!("Not Implemented")
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        panic!("Not Implemented")
    }
    fn logical_and_with_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn load_h_and_l_direct(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn add_immediate_with_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn call(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn enable_interrupts(&mut self)
    {
        panic!("Not Implemented")
    }
    fn load_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn input(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn jump_if_parity_odd(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn increment_register_pair(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn logical_exclusive_or(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn exchange_registers(&mut self)
    {
        panic!("Not Implemented")
    }
    fn rotate_accumulator_right(&mut self)
    {
        panic!("Not Implemented")
    }
    fn call_if_no_carry(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn return_if_parity_even(&mut self)
    {
        panic!("Not Implemented")
    }
    fn call_if_zero(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn return_unconditionally(&mut self)
    {
        panic!("Not Implemented")
    }
    fn halt(&mut self)
    {
        panic!("Not Implemented")
    }
    fn call_if_plus(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn compare_immediate_with_accumulator(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn load_program_counter(&mut self)
    {
        panic!("Not Implemented")
    }
    fn return_if_minus(&mut self)
    {
        panic!("Not Implemented")
    }
    fn jump_if_carry(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn call_if_minus(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        panic!("Not Implemented")
    }
    fn load_register_pair_immediate(&mut self, _register1: Register8080, _data2: u16)
    {
        panic!("Not Implemented")
    }
    fn move_immediate_data(&mut self, _register1: Register8080, _data2: u8)
    {
        panic!("Not Implemented")
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn return_if_plus(&mut self)
    {
        panic!("Not Implemented")
    }
    fn restart(&mut self, _implicit_data1: u8)
    {
        panic!("Not Implemented")
    }
    fn store_accumulator_direct(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn jump_if_not_zero(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn jump_if_minus(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn decrement_register_or_memory(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn output(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn store_accumulator(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn add_to_accumulator_with_carry(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn return_if_carry(&mut self)
    {
        panic!("Not Implemented")
    }
    fn complement_accumulator(&mut self)
    {
        panic!("Not Implemented")
    }
    fn return_if_no_carry(&mut self)
    {
        panic!("Not Implemented")
    }
    fn return_if_zero(&mut self)
    {
        panic!("Not Implemented")
    }
    fn return_if_parity_odd(&mut self)
    {
        panic!("Not Implemented")
    }
    fn store_h_and_l_direct(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn subtract_from_accumulator_with_borrow(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn not_implemented(&mut self)
    {
        panic!("Not Implemented")
    }
    fn push_data_onto_stack(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn jump_if_no_carry(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn sim(&mut self)
    {
        panic!("Not Implemented")
    }
    fn decrement_register_pair(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn rotate_accumulator_left(&mut self)
    {
        panic!("Not Implemented")
    }
    fn load_accumulator_direct(&mut self, _address1: u16)
    {
        panic!("Not Implemented")
    }
    fn exchange_stack(&mut self)
    {
        panic!("Not Implemented")
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        panic!("Not Implemented")
    }
}

#[test]
fn complement_carry_test_false_to_true()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, false);

    e.complement_carry();

    assert!(e.read_flag(Flag8080::Carry))
}

#[test]
fn complement_carry_test_true_to_false()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, true);

    e.complement_carry();

    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn set_carry_test_false_to_true()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, false);

    e.set_carry();

    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn set_carry_test_true_to_true()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, false);

    e.set_carry();

    assert!(e.read_flag(Flag8080::Carry));
}

#[cfg(test)]
fn increment_register_or_memory_test(e: &mut Emulator8080, starting_value: u8)
{
    e.clear_all_flags();
    e.set_register(Register8080::B, starting_value);
    e.increment_register_or_memory(Register8080::B);
}

#[test]
fn increment_register_or_memory_increments()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x99);
    assert_eq!(e.read_register(Register8080::B), 0x9A);
}

#[test]
fn increment_register_or_memory_doesnt_set_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x99);
    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn increment_register_or_memory_update_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0xFF);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn increment_register_or_memory_doesnt_set_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x00);
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn increment_register_or_memory_sets_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x7f);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn increment_register_or_memory_clears_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x00);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Flag8080::Parity));
}

#[test]
fn increment_register_or_memory_sets_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, 0x02);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Flag8080::Parity));
}

impl Emulator8080 {
    fn _run_opcode(&mut self, stream: &[u8]) -> u8
    {
        dispatch_opcode(stream, self)
    }
}

pub fn run_emulator<'a>(rom: &'a [u8]) {
    let _e = Emulator8080::new(rom);
}
