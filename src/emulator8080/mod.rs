pub mod opcodes;

use std::mem;

use emulator8080::opcodes::opcode_gen::{InstructionSet8080, Register8080, dispatch_opcode};

const MAX_ADDRESS: usize = 0xffff;
const ROM_ADDRESS: usize = 0x0100;

#[derive(Debug,Clone,Copy)]
enum Flag8080 {
    Sign = 0x80,           // Bit 7
    Zero = 0x40,           // Bit 6
    AuxiliaryCarry = 0x10, // Bit 4
    Parity = 0x4,          // Bit 2
    Carry = 0x1,           // Bit 0
}

struct Emulator8080 {
    main_memory: [u8; MAX_ADDRESS + 1],
    registers: [u8; Register8080::Count as usize]
}

impl Emulator8080 {
    fn new(rom: &[u8]) -> Emulator8080
    {
        let mut emu = Emulator8080 {
            main_memory: [0; MAX_ADDRESS + 1],
            registers: [0; Register8080::Count as usize]
        };

        emu.main_memory[ROM_ADDRESS..(ROM_ADDRESS + rom.len())].clone_from_slice(rom);

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

    fn get_register_pair(&mut self, register: Register8080) -> &mut u16
    {
        /*
         * 8008 Registers are laid out in a specified order in memory. (See enum
         * Register8080 for the order).  This function allows you to read two
         * adjacent registers together as one u16.  These two registers together
         * are known as a 'register pair.'  The register pairs are referenced by
         * the first in the pair.  The only valid register pairs are:
         *     B:   B and C
         *     D:   D and E
         *     H:   H and L
         *     PSW: A and FLAGS
         */
        let register_pairs: &mut [u16; Register8080::Count as usize / 2];
        unsafe {
             register_pairs = mem::transmute(&mut self.registers);
        }
        match register {
            Register8080::B | Register8080::D | Register8080::H =>
                &mut register_pairs[register as usize / 2],
            Register8080::PSW => &mut register_pairs[Register8080::A as usize / 2],
            _ => panic!("Invalid register")
        }
    }

    #[cfg(test)]
    fn set_register_pair(&mut self, register: Register8080, value: u16)
    {
        *self.get_register_pair(register) = u16::to_be(value);
    }

    fn read_register_pair(&mut self, register: Register8080) -> u16
    {
        u16::from_be(*self.get_register_pair(register))
    }

    fn get_register(&mut self, register: Register8080) -> &mut u8
    {
        match register {
            Register8080::PSW => panic!("PSW too big"),
            Register8080::M => {
                /*
                 * The M register is special and represents the byte stored at
                 * the memory address stored in the register pair H.
                 */
                let address = self.read_register_pair(Register8080::H) as usize;
                &mut self.main_memory[address]
            },
            _ => &mut self.registers[register as usize]
        }
    }

    fn set_register(&mut self, register: Register8080, value: u8)
    {
        *self.get_register(register) = value;
    }

    fn read_register(&mut self, register: Register8080) -> u8
    {
        *self.get_register(register)
    }
}

#[test]
fn read_register_pair_b()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x3F);
    e.set_register(Register8080::C, 0x29);

    assert_eq!(e.read_register_pair(Register8080::B), 0x3Fu16 << 8 | 0x29u16);
}

#[test]
fn read_and_set_register_pair_d()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register_pair(Register8080::D, 0xBEEF);

    assert_eq!(e.read_register_pair(Register8080::D), 0xBEEF);
}

#[test]
fn set_register_pair_h_and_read_registers()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register_pair(Register8080::H, 0xABCD);

    assert_eq!(e.read_register(Register8080::H), 0xAB);
    assert_eq!(e.read_register(Register8080::L), 0xCD);
}

#[test]
fn set_register_m()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register_pair(Register8080::H, 0xf812);
    e.set_register(Register8080::M, 0xAB);

    assert_eq!(e.main_memory[0xf812], 0xAB);
}

#[test]
fn set_register_m_max_address()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register_pair(Register8080::H, MAX_ADDRESS as u16);
    e.set_register(Register8080::M, 0xD9);

    assert_eq!(e.main_memory[MAX_ADDRESS], 0xD9);
}

fn calculate_parity(value: u8) -> bool
{
    /*
     * Parity for a give byte can be odd or even.  Odd parity means the byte
     * when represented as binary has an odd number of ones.  Even means the
     * number of ones is even.  The 8080 represents odd parity as 0 (or false)
     * and even parity as 1 (or true).
     */
    value.count_ones() % 2 == 0
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

/*
 *  ___           _                   _   _
 * |_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __  ___
 *  | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
 *  | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
 * |___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
 *
 */

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
        let old_value = self.read_register(register);
        let new_value = match old_value {
            0xFF => 0,
            _ => old_value + 1
        };
        self.set_register(register, new_value);

        self.set_flag(Flag8080::Zero, new_value == 0);
        self.set_flag(Flag8080::Sign, new_value & 0x80 != 0);
        self.set_flag(Flag8080::Parity, calculate_parity(new_value));

        // XXX This needs to be generalized and explained.
        self.set_flag(Flag8080::AuxiliaryCarry, new_value & 0x0F < old_value & 0x0F);
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
fn increment_register_or_memory_test(
    e: &mut Emulator8080,
    register: Register8080,
    starting_value: u8)
{
    e.clear_all_flags();
    e.set_register(register, starting_value);
    e.increment_register_or_memory(register);
}

#[test]
fn increment_register_or_memory_increments_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0x99);
    assert_eq!(e.read_register(Register8080::B), 0x9A);
}

#[test]
fn increment_register_or_memory_increments_memory()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register_pair(Register8080::H, 0x1234);
    increment_register_or_memory_test(&mut e, Register8080::M, 0x19);
    assert_eq!(e.read_register(Register8080::M), 0x1A);
}

#[test]
fn increment_register_or_memory_doesnt_set_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0x99);
    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn increment_register_or_memory_update_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0xFF);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn increment_register_or_memory_doesnt_set_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0x00);
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn increment_register_or_memory_sets_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0x7f);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn increment_register_or_memory_clears_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Parity, true);
    increment_register_or_memory_test(&mut e, Register8080::B, 0x00);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Flag8080::Parity));
}

#[test]
fn increment_register_or_memory_sets_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    increment_register_or_memory_test(&mut e, Register8080::B, 0x02);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn increment_register_or_memory_sets_auxiliary_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    //           76543210
    //    (0x0f) 00001111
    //  + (0x01) 00000001
    //    (0x10) 00010000
    increment_register_or_memory_test(&mut e, Register8080::B, 0x0f);

    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn increment_register_or_memory_clears_auxiliary_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::AuxiliaryCarry, true);
    increment_register_or_memory_test(&mut e, Register8080::B, 0x00);

    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
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
