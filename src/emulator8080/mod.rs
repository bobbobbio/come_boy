pub mod opcodes;

use std::mem;

use emulator8080::opcodes::opcode_gen::{
    InstructionSet8080, Register8080, dispatch_opcode, opcode_size};

const MAX_ADDRESS: usize = 0xffff;
const ROM_ADDRESS: usize = 0x0100;

#[derive(Debug,Clone,Copy)]
enum Flag8080 {
                    // 76543210
    Sign =           0b10000000,
    Zero =           0b01000000,
    AuxiliaryCarry = 0b00010000,
    Parity =         0b00000100,
    Carry =          0b00000001,
}

/*
 *  _          _
 * | |__   ___| |_ __   ___ _ __ ___
 * | '_ \ / _ \ | '_ \ / _ \ '__/ __|
 * | | | |  __/ | |_) |  __/ |  \__ \
 * |_| |_|\___|_| .__/ \___|_|  |___/
 *              |_|
 */

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
    assert_eq!(calculate_parity(0b00000001), false);
}

#[test]
fn calculate_parity_even_parity()
{
    assert_eq!(calculate_parity(0b00000011), true);
}

#[test]
fn calculate_parity_zero_is_even_parity()
{
    assert_eq!(calculate_parity(0b00000000), true);
}

fn twos_complement(value: u8) -> u8
{
    (!value).wrapping_add(1)
}

/*
 *  _____                 _       _             ___   ___   ___   ___
 * | ____|_ __ ___  _   _| | __ _| |_ ___  _ __( _ ) / _ \ ( _ ) / _ \
 * |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__/ _ \| | | |/ _ \| | | |
 * | |___| | | | | | |_| | | (_| | || (_) | | | (_) | |_| | (_) | |_| |
 * |_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|  \___/ \___/ \___/ \___/
 *
 */

struct Emulator8080 {
    main_memory: [u8; MAX_ADDRESS + 1],
    registers: [u8; Register8080::Count as usize],
    _program_counter: u16
}

impl Emulator8080 {
    fn new(rom: &[u8]) -> Emulator8080
    {
        let mut emu = Emulator8080 {
            main_memory: [0; MAX_ADDRESS + 1],
            registers: [0; Register8080::Count as usize],
            _program_counter: 0
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

    fn read_flag(&mut self, flag: Flag8080) -> bool
    {
        self.registers[Register8080::FLAGS as usize] & (flag as u8) == flag as u8
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

    // XXX: This really shouldn't take a mutable reference to self.
    fn read_register(&mut self, register: Register8080) -> u8
    {
        *self.get_register(register)
    }

    fn update_flags_for_new_value(&mut self, new_value: u8)
    {
        self.set_flag(Flag8080::Zero, new_value == 0);
        self.set_flag(Flag8080::Sign, new_value & 0b10000000 != 0);
        self.set_flag(Flag8080::Parity, calculate_parity(new_value));
    }

    fn perform_addition(&mut self, value_a: u8, value_b: u8, update_carry: bool) -> u8
    {
        let new_value = value_a.wrapping_add(value_b);
        self.update_flags_for_new_value(new_value);

        if update_carry {
            self.set_flag(Flag8080::Carry, value_b > 0xFF - value_a);
        }

        self.set_flag(Flag8080::AuxiliaryCarry, value_b & 0x0F > 0x0F - (value_a & 0x0F));

        return new_value;
    }

    fn perform_subtraction_using_twos_complement(
        &mut self,
        value_a: u8,
        value_b: u8) -> u8
    {
        let new_value = self.perform_addition(
            value_a, twos_complement(value_b), true /* update carry */);
        self.flip_flag(Flag8080::Carry);
        return new_value;
    }

    fn add_to_register(&mut self, register: Register8080, value: u8, update_carry: bool)
    {
        let old_value = self.read_register(register);
        let new_value = self.perform_addition(old_value, value, update_carry);
        self.set_register(register, new_value);
    }

    fn subtract_from_register(&mut self, register: Register8080, value: u8)
    {
        let old_value = self.read_register(register);
        let new_value = old_value.wrapping_sub(value);
        self.set_register(register, new_value);
        self.update_flags_for_new_value(new_value);
        self.set_flag(Flag8080::AuxiliaryCarry, false);
    }

    fn subtract_from_register_using_twos_complement(&mut self, register: Register8080, value: u8)
    {
        let old_value = self.read_register(register);
        let new_value = self.perform_subtraction_using_twos_complement(old_value, value);
        self.set_register(register, new_value);
    }
}

/*
 *                 _     _                       _        __
 *  _ __ ___  __ _(_)___| |_ ___ _ __   ___  ___| |_     / /
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

#[cfg(test)]
fn add_to_register_test(
    e: &mut Emulator8080,
    register: Register8080,
    starting_value: u8,
    delta: u8,
    update_carry: bool)
{
    e.set_register(register, starting_value);
    e.add_to_register(register, delta, update_carry);
}

#[test]
fn add_to_register_increments_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x99, 1, true /* update carry */);
    assert_eq!(e.read_register(Register8080::B), 0x9A);
}

#[test]
fn add_to_register_increments_memory()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register_pair(Register8080::H, 0x1234);
    add_to_register_test(&mut e, Register8080::M, 0x19, 1, true /* update carry */);
    assert_eq!(e.read_register(Register8080::M), 0x1A);
}

#[test]
fn add_to_register_overflows()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xFF, 1, true /* update carry */);
    assert_eq!(e.read_register(Register8080::B), 0x00);
}

#[test]
fn add_to_register_doesnt_set_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x99, 1, true /* update carry */);
    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn add_to_register_update_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xFF, 1, true /* update carry */);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn add_to_register_doesnt_set_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x00, 1, true /* update carry */);
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn add_to_register_sets_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x7f, 1, true /* update carry */);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn add_to_register_clears_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Parity, true);
    add_to_register_test(&mut e, Register8080::B, 0x00, 1, true /* update carry */);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Flag8080::Parity));
}

#[test]
fn add_to_register_sets_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x02, 1, true /* update carry */);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x0F, 1, true /* update carry */);

    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag_when_adding_u8_max()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0x0F, 0xFF, true /* update carry */);

    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_incrementing_high_bits_only()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xA0, 0x10, true /* update carry */);

    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_overflowing_basic()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xF0, 0x10, true /* update carry */);

    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_doesnt_set_auxiliary_carry_flag_when_overflowing()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xFF, 0x10, true /* update carry */);

    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_auxiliary_carry_flag_when_overflowing()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    add_to_register_test(&mut e, Register8080::B, 0xFF, 0x11, true /* update carry */);

    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_clears_auxiliary_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::AuxiliaryCarry, true);
    add_to_register_test(&mut e, Register8080::B, 0x00, 1, true /* update carry */);

    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn add_to_register_sets_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    add_to_register_test(&mut e, Register8080::B, 0xFF, 1, true /* update carry */);

    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn add_to_register_sets_carry_flag_when_adding_u8_max()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    add_to_register_test(&mut e, Register8080::B, 0xAB, 0xFF, true /* update carry */);

    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn add_to_register_clears_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, true);
    add_to_register_test(&mut e, Register8080::B, 0x01, 1, true /* update carry */);

    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn add_to_register_doesnt_update_carry_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, true);
    add_to_register_test(&mut e, Register8080::B, 0x01, 1, false /* update carry */);

    assert!(e.read_flag(Flag8080::Carry));
}

#[cfg(test)]
fn subtract_from_register_test(
    e: &mut Emulator8080,
    register: Register8080,
    starting_value: u8,
    delta: u8)
{
    e.set_register(register, starting_value);
    e.subtract_from_register(register, delta);
}

#[test]
fn subtract_from_register_underflows()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::C, 0x00, 2);
    assert_eq!(e.read_register(Register8080::C), 0xFE);
}

#[test]
fn subtract_from_register_doesnt_set_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x99, 1);
    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn subtract_from_register_update_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x01, 1);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn subtract_from_register_doesnt_set_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x0B, 1);
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn subtract_from_register_sets_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x00, 1);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn subtract_from_register_clears_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Parity, true);
    subtract_from_register_test(&mut e, Register8080::B, 0x02, 1);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Flag8080::Parity));
}

#[test]
fn subtract_from_register_sets_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x04, 1);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn subtract_from_register_clears_auxiliary_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::AuxiliaryCarry, true);
    subtract_from_register_test(&mut e, Register8080::C, 0x88, 0x03);
    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn subtract_from_register_doesnt_set_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    subtract_from_register_test(&mut e, Register8080::C, 0x88, 4);
    assert!(e.read_flag(Flag8080::Carry));
}

#[cfg(test)]
fn subtract_from_register_using_twos_complement_test(
    e: &mut Emulator8080,
    register: Register8080,
    starting_value: u8,
    delta: u8)
{
    e.set_register(register, starting_value);
    e.subtract_from_register_using_twos_complement(register, delta);
}

#[test]
fn subtract_from_register_using_twos_complement_positive_from_positive()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(&mut e, Register8080::C, 0x88, 4);
    assert_eq!(e.read_register(Register8080::C), 0x84);
}

#[test]
fn subtract_from_register_using_twos_complement_positive_from_negative()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(
        &mut e, Register8080::C, twos_complement(0x09), 0x02);
    assert_eq!(e.read_register(Register8080::C), twos_complement(0x0B));
}

#[test]
fn subtract_from_register_using_twos_complement_negative_from_positive()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(
        &mut e, Register8080::C, 0x09, twos_complement(0x02));
    assert_eq!(e.read_register(Register8080::C), 0x0B);
}

#[test]
fn subtract_from_register_using_twos_complement_negative_from_negative()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(
        &mut e, Register8080::C, twos_complement(0x09), twos_complement(0x02));
    assert_eq!(e.read_register(Register8080::C), twos_complement(0x07));
}

#[test]
fn subtract_from_register_using_twos_complement_clears_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    subtract_from_register_using_twos_complement_test(&mut e, Register8080::C, 0x88, 4);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn subtract_from_register_using_twos_complement_sets_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(&mut e, Register8080::C, 0x05, 0x06);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn subtract_from_register_using_twos_complement_clear_auxiliary_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_using_twos_complement_test(&mut e, Register8080::C, 0xA5, 0x04);
    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn subtract_from_register_using_twos_complement_sets_auxiliary_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::AuxiliaryCarry, true);
    subtract_from_register_using_twos_complement_test(&mut e, Register8080::C, 0xA5, 0x06);
    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
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
        self.add_to_register(register, 1, false /* update carry */);
    }
    fn decrement_register_or_memory(&mut self, register: Register8080)
    {
        self.subtract_from_register(register, 1);
    }
    fn complement_accumulator(&mut self)
    {
        let old_value = self.read_register(Register8080::A);
        self.set_register(Register8080::A, !old_value);
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        /* The eight-bit hexadecimal number in the accumulator is adjusted to form two four-bit
         * binary-coded decimal digits by the follow two step process:
         *
         * (1) If the least significant four bits of the accumulator represents a number greater
         * than 9, or if the Auxiliary Carry bit is equal to one, the accumulator is incremented
         * by six. Otherwise, no incrementing occurs.
         */

        let accumulator = self.read_register(Register8080::A);
        if accumulator & 0x0F > 9 || self.read_flag(Flag8080::AuxiliaryCarry) {
            self.add_to_register(Register8080::A, 6, true /* update carry */);
        }
        let auxiliary_carry = self.read_flag(Flag8080::AuxiliaryCarry);

        /*
         * (2) If the most significant four bits of the accumulator now represent a number greater
         * than 9, or if the normal carry bit is equal to one, the most significant four bits of
         * the accumulator are incremented by six. Otherwise, no incrementing occurs.
         */
        let accumulator = self.read_register(Register8080::A);
        if (accumulator >> 4) & 0x0F > 9 || self.read_flag(Flag8080::Carry) {
            self.add_to_register(Register8080::A, 6 << 4, true /* update carry */);
        }
        self.set_flag(Flag8080::AuxiliaryCarry, auxiliary_carry);
    }
    fn no_instruction(&mut self)
    {
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
    fn move_data(&mut self, dest_register: Register8080, src_register: Register8080)
    {
        let value = self.read_register(src_register);
        self.set_register(dest_register, value);
    }
    fn store_accumulator(&mut self, register_pair: Register8080)
    {
        let address = self.read_register_pair(register_pair) as usize;
        self.main_memory[address] = self.read_register(Register8080::A);
    }
    fn load_accumulator(&mut self, register_pair: Register8080)
    {
        let address = self.read_register_pair(register_pair) as usize;
        let value = self.main_memory[address];
        self.set_register(Register8080::A, value);
    }
    fn add_to_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.add_immediate_to_accumulator(value);
    }
    fn add_to_accumulator_with_carry(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.add_immediate_to_accumulator_with_carry(value);
    }
    fn subtract_from_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.subtract_immediate_from_accumulator(value);
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.subtract_immediate_from_accumulator_with_borrow(value);
    }
    fn logical_and_with_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.and_immediate_with_accumulator(value);
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.exclusive_or_immediate_with_accumulator(value);
    }
    fn logical_or_with_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.or_immediate_with_accumulator(value);
    }
    fn compare_with_accumulator(&mut self, register: Register8080)
    {
        let value = self.read_register(register);
        self.compare_immediate_with_accumulator(value);
    }
    fn rotate_accumulator_left(&mut self)
    {
        let accumulator = self.read_register(Register8080::A);
        let rotated_bit = (accumulator & 0b10000000) >> 7;
        self.set_register(Register8080::A, (accumulator << 1) | rotated_bit);
        self.set_flag(Flag8080::Carry, rotated_bit == 0x01);
    }
    fn rotate_accumulator_right(&mut self)
    {
        let accumulator = self.read_register(Register8080::A);
        let rotated_bit = accumulator & 0x1;
        self.set_register(Register8080::A, (accumulator >> 1) | (rotated_bit << 7));
        self.set_flag(Flag8080::Carry, rotated_bit == 0x01);
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        let accumulator = self.read_register(Register8080::A);
        let rotated_bit = (accumulator & 0b10000000) >> 7;
        let carry = self.read_flag(Flag8080::Carry);
        self.set_register(Register8080::A, (accumulator << 1) | if carry { 0x01 } else { 0x0 });
        self.set_flag(Flag8080::Carry, rotated_bit == 0x01);
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        let accumulator = self.read_register(Register8080::A);
        let rotated_bit = accumulator & 0x1;
        let carry = self.read_flag(Flag8080::Carry);
        self.set_flag(Flag8080::Carry, rotated_bit == 0x01);
        self.set_register(Register8080::A, (accumulator >> 1) | if carry { 0x80 } else { 0x0 });
    }
    fn push_data_onto_stack(&mut self, register_pair: Register8080)
    {
        let sp = self.read_register(Register8080::SP) as usize;
        let pair_data = self.read_register_pair(register_pair);
        self.main_memory[sp - 1] = ((pair_data & 0xFF00) >> 8) as u8;
        self.main_memory[sp - 2] = (pair_data & 0x00FF) as u8;
        self.set_register(Register8080::SP, (sp - 2) as u8);
    }
    fn pop_data_off_stack(&mut self, register_pair: Register8080)
    {
        let sp = self.read_register(Register8080::SP) as usize;
        let pair_data = self.main_memory[sp - 2] as u16 | (self.main_memory[sp - 1] as u16) << 8;
        self.set_register_pair(register_pair, pair_data);
        self.set_register(Register8080::SP, (sp + 2) as u8);
    }

    fn double_add(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn increment_register_pair(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn decrement_register_pair(&mut self, _register1: Register8080)
    {
        panic!("Not Implemented")
    }
    fn exchange_registers(&mut self)
    {
        panic!("Not Implemented")
    }
    fn exchange_stack(&mut self)
    {
        panic!("Not Implemented")
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        panic!("Not Implemented")
    }

    fn move_immediate_data(&mut self, _register1: Register8080, _data2: u8)
    {
        panic!("Not Implemented")
    }
    fn add_immediate_to_accumulator(&mut self, data: u8)
    {
        self.add_to_register(Register8080::A, data, true /* update_carry */);
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, mut data: u8)
    {
        if self.read_flag(Flag8080::Carry) {
            data = data.wrapping_add(1);
        }
        self.add_to_register(Register8080::A, data, true /* update_carry */);
    }
    fn subtract_immediate_from_accumulator(&mut self, data: u8)
    {
        self.subtract_from_register_using_twos_complement(Register8080::A, data);
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, mut data: u8)
    {
        if self.read_flag(Flag8080::Carry) {
            data = data.wrapping_add(1);
        }
        self.subtract_from_register_using_twos_complement(Register8080::A, data);
    }
    fn and_immediate_with_accumulator(&mut self, data: u8)
    {
        let value = self.read_register(Register8080::A);
        let new_value = value & data;
        self.set_register(Register8080::A, new_value);
        self.update_flags_for_new_value(new_value);
        self.set_flag(Flag8080::Carry, false);
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data: u8)
    {
        let value = self.read_register(Register8080::A);
        let new_value = value ^ data;
        self.set_register(Register8080::A, new_value);
        self.update_flags_for_new_value(new_value);
        self.set_flag(Flag8080::Carry, false);
    }
    fn or_immediate_with_accumulator(&mut self, data: u8)
    {
        let value = self.read_register(Register8080::A);
        let new_value = value | data;
        self.set_register(Register8080::A, new_value);
        self.update_flags_for_new_value(new_value);
        self.set_flag(Flag8080::Carry, false);
    }
    fn compare_immediate_with_accumulator(&mut self, data: u8)
    {
        let accumulator = self.read_register(Register8080::A);
        self.perform_subtraction_using_twos_complement(accumulator, data);
    }

    fn return_if_not_zero(&mut self)
    {
        panic!("Not Implemented")
    }
    fn jump_if_parity_even(&mut self, _address1: u16)
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
    fn disable_interrupts(&mut self)
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
    fn load_h_and_l_direct(&mut self, _address1: u16)
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
    fn input(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn jump_if_parity_odd(&mut self, _address1: u16)
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
    fn load_register_pair_immediate(&mut self, _register1: Register8080, _data2: u16)
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
    fn output(&mut self, _data1: u8)
    {
        panic!("Not Implemented")
    }
    fn return_if_carry(&mut self)
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
    fn not_implemented(&mut self)
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
    fn load_accumulator_direct(&mut self, _address1: u16)
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

#[test]
fn increment_register_or_memory_increments_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x99);
    e.increment_register_or_memory(Register8080::B);
    assert_eq!(e.read_register(Register8080::B), 0x9A);
}

#[test]
fn increment_register_or_memory_doesnt_update_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0xFF);
    e.increment_register_or_memory(Register8080::B);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn decrement_register_or_memory_decrements_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x40);
    e.decrement_register_or_memory(Register8080::B);
    assert_eq!(e.read_register(Register8080::B), 0x3F);
}

#[test]
fn decrement_register_or_memory_doesnt_update_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::B, 0x40);
    e.decrement_register_or_memory(Register8080::B);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn complement_accumulator()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10010101);
    e.complement_accumulator();
    assert_eq!(e.read_register(Register8080::A), 0b01101010);
}

#[test]
fn decimal_adjust_accumulator_low_and_high_bits()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    /*
     * Suppose the accumulator contains 9BH, and both carry bits aren't set.
     */
    e.set_register(Register8080::A, 0x9B);
    e.set_flag(Flag8080::Carry, false);
    e.set_flag(Flag8080::AuxiliaryCarry, false);

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
    assert_eq!(e.read_register(Register8080::A), 0x01);
    assert!(e.read_flag(Flag8080::Carry));
    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn decimal_adjust_accumulator_low_bits_increment_only()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0x0F);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Register8080::A), 0x0F + 6);
    assert!(!e.read_flag(Flag8080::Carry));
    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn decimal_adjust_accumulator_high_bits_increment_only()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0xA0);
    e.decimal_adjust_accumulator();

    assert_eq!(e.read_register(Register8080::A), 0xA0u8.wrapping_add(6 << 4));
    assert!(e.read_flag(Flag8080::Carry));
    assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
}

#[test]
fn no_instruction()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.no_instruction();
}

#[test]
fn move_data_moves_to_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0xA0);
    e.move_data(Register8080::E, Register8080::A);

    assert_eq!(e.read_register(Register8080::E), 0xA0);
}

#[test]
fn move_data_same_destination_and_source()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0xA0);
    e.move_data(Register8080::A, Register8080::A);

    assert_eq!(e.read_register(Register8080::A), 0xA0);
}

#[test]
fn move_data_moves_to_memory()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0xBE);
    e.set_register_pair(Register8080::H, 0x2BE9);
    e.move_data(Register8080::M, Register8080::A);

    assert_eq!(e.main_memory[0x2BE9], 0xBE);
}

#[test]
fn store_accumulator_at_address_in_b()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0xAF);
    e.set_register_pair(Register8080::B, 0x3F16);
    e.store_accumulator(Register8080::B);

    assert_eq!(e.main_memory[0x3F16], 0xAF);
}

#[test]
fn store_accumulator_at_address_in_d()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0xAF);
    e.set_register_pair(Register8080::D, 0x3F16);
    e.store_accumulator(Register8080::D);

    assert_eq!(e.main_memory[0x3F16], 0xAF);
}

#[test]
fn load_accumulator_from_address_in_b()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.main_memory[0x9388] = 0xAF;
    e.set_register_pair(Register8080::B, 0x9388);
    e.load_accumulator(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0xAF);
}

#[test]
fn load_accumulator_from_address_in_d()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.main_memory[0x9388] = 0xAF;
    e.set_register_pair(Register8080::D, 0x9388);
    e.load_accumulator(Register8080::D);

    assert_eq!(e.read_register(Register8080::A), 0xAF);
}

#[test]
fn add_to_accumulator_from_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::D, 0xBD);
    e.set_register(Register8080::A, 0x09);
    e.add_to_accumulator(Register8080::D);

    assert_eq!(e.read_register(Register8080::A), 0xBD + 0x09);
}

#[test]
fn add_to_accumulator_with_carry_from_register_and_carry_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::D, 0xBD);
    e.set_register(Register8080::A, 0x09);
    e.set_flag(Flag8080::Carry, true);
    e.add_to_accumulator_with_carry(Register8080::D);

    assert_eq!(e.read_register(Register8080::A), 0xBD + 0x09 + 1);
}

#[test]
fn add_to_accumulator_with_carry_from_register_and_carry_not_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::D, 0xBD);
    e.set_register(Register8080::A, 0x09);
    e.add_to_accumulator_with_carry(Register8080::D);

    assert_eq!(e.read_register(Register8080::A), 0xBD + 0x09);
}

#[test]
fn subtract_from_accumulator_from_register()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0xF2);
    e.set_register(Register8080::A, 0x1A);
    e.subtract_from_accumulator(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0x1Au8.wrapping_sub(0xF2));
}

#[test]
fn subtract_from_accumulator_with_borrow_condition_flags_get_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::L, 0x02);
    e.set_register(Register8080::A, 0x04);
    e.set_flag(Flag8080::Carry, true);
    e.subtract_from_accumulator_with_borrow(Register8080::L);

    assert_eq!(e.read_register(Register8080::A), 0x01);
    assert!(!e.read_flag(Flag8080::Zero));
    assert!(!e.read_flag(Flag8080::Carry));
    assert!(e.read_flag(Flag8080::AuxiliaryCarry));
    assert!(!e.read_flag(Flag8080::Parity));
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn subtract_from_accumulator_with_borrow_and_carry_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0xF2);
    e.set_register(Register8080::A, 0x1A);
    e.set_flag(Flag8080::Carry, true);
    e.subtract_from_accumulator_with_borrow(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0x1Au8.wrapping_sub(0xF2 + 1));
}

#[test]
fn subtract_from_accumulator_with_borrow_and_carry_not_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0xF2);
    e.set_register(Register8080::A, 0x1A);
    e.subtract_from_accumulator_with_borrow(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0x1Au8.wrapping_sub(0xF2));
}

#[test]
fn logical_and_with_accumulator()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b00100110);
    e.set_register(Register8080::A, 0b01000111);
    e.logical_and_with_accumulator(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0b00000110);
}

#[test]
fn logical_and_with_accumulator_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x0);
    e.set_register(Register8080::A, 0x4F);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn logical_and_with_accumulator_sets_parity()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b11011100);
    e.set_register(Register8080::A, 0b11000000);
    e.logical_and_with_accumulator(Register8080::B);

    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn logical_and_with_accumulator_sets_sign()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b10000000);
    e.set_register(Register8080::A, 0b10000000);
    e.logical_and_with_accumulator(Register8080::B);

    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn logical_and_with_accumulator_clears_the_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.logical_and_with_accumulator(Register8080::B);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn logical_exclusive_or_with_accumulator()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b00100110);
    e.set_register(Register8080::A, 0b01000111);
    e.logical_exclusive_or_with_accumulator(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0b01100001);
}

#[test]
fn logical_exclusive_or_with_accumulator_zeros_accumulator()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b01000111);
    e.logical_exclusive_or_with_accumulator(Register8080::A);

    assert_eq!(e.read_register(Register8080::A), 0x0);
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0xFF);
    e.set_register(Register8080::A, 0xFF);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_parity()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b01001001);
    e.set_register(Register8080::A, 0b00000001);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn logical_exclusive_or_with_accumulator_sets_sign()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b10000000);
    e.set_register(Register8080::A, 0b00100100);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn logical_exclusive_or_with_accumulator_clears_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.logical_exclusive_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn logical_or_with_accumulator()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b00100110);
    e.set_register(Register8080::A, 0b01000111);
    e.logical_or_with_accumulator(Register8080::B);

    assert_eq!(e.read_register(Register8080::A), 0b01100111);
}

#[test]
fn logical_or_with_accumulator_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x00);
    e.set_register(Register8080::A, 0x00);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn logical_or_with_accumulator_sets_parity()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b01001011);
    e.set_register(Register8080::A, 0b00000001);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn logical_or_with_accumulator_sets_sign()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0b10000000);
    e.set_register(Register8080::A, 0b00100100);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn logical_or_with_accumulator_clears_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.logical_or_with_accumulator(Register8080::B);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn compare_with_accumulator_doesnt_affect_register_values()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x05);
    e.compare_with_accumulator(Register8080::E);

    assert_eq!(e.read_register(Register8080::A), 0x0A);
    assert_eq!(e.read_register(Register8080::E), 0x05);
}

#[test]
fn compare_with_accumulator_clears_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Carry, true);

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x05);
    e.compare_with_accumulator(Register8080::E);

    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn compare_with_accumulator_sets_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x0B);
    e.compare_with_accumulator(Register8080::E);

    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn compare_with_accumulator_clears_zero()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Zero, true);

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x05);
    e.compare_with_accumulator(Register8080::E);

    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn compare_with_accumulator_sets_zero()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x0A);
    e.compare_with_accumulator(Register8080::E);

    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn compare_with_accumulator_clears_sign()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Sign, true);
    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x01);
    e.compare_with_accumulator(Register8080::E);

    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn compare_with_accumulator_sets_sign()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_register(Register8080::A, 0x0A);
    e.set_register(Register8080::E, 0x0B);
    e.compare_with_accumulator(Register8080::E);

    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn rotate_accumulator_left_carry_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_left();

    assert_eq!(e.read_register(Register8080::A), 0b01001111);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_left_carry_cleared()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b00100111);
    e.rotate_accumulator_left();

    assert_eq!(e.read_register(Register8080::A), 0b01001110);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_carry_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_right();

    assert_eq!(e.read_register(Register8080::A), 0b11010011);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_carry_cleared()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b10100110);
    e.rotate_accumulator_right();

    assert_eq!(e.read_register(Register8080::A), 0b01010011);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_set_to_reset()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b01100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b11001111);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_stays_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b01001111);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_reset_to_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b01001110);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_left_through_carry_and_carry_stays_reset()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b00100111);
    e.rotate_accumulator_left_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b01001110);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_set_to_unset()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b10100110);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b11010011);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_stays_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b11010011);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_reset_to_set()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10100111);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b01010011);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn rotate_accumulator_right_through_carry_and_carry_stays_reset()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0b10100110);
    e.rotate_accumulator_right_through_carry();

    assert_eq!(e.read_register(Register8080::A), 0b01010011);
    assert!(!e.read_flag(Flag8080::Carry));
}

#[test]
fn push_register_pair_onto_stack()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::B, 0x34);
    e.set_register(Register8080::C, 0xA7);
    e.set_register(Register8080::SP, 0x20);
    e.push_data_onto_stack(Register8080::B);
    assert_eq!(e.main_memory[0x20 - 1], 0x34);
    assert_eq!(e.main_memory[0x20 - 2], 0xA7);
    assert_eq!(e.read_register(Register8080::SP), 0x20 - 2);
}

#[test]
fn push_psw_onto_stack()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_register(Register8080::A, 0x34);
    e.set_register(Register8080::FLAGS, 0xA7);
    e.set_register(Register8080::SP, 0x20);
    e.push_data_onto_stack(Register8080::PSW);
    assert_eq!(e.main_memory[0x20 - 1], 0x34);
    assert_eq!(e.main_memory[0x20 - 2], 0xA7);
    assert_eq!(e.read_register(Register8080::SP), 0x20 - 2);
}

#[test]
fn pop_register_pair_from_stack()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.main_memory[0x20 - 1] = 0x78;
    e.main_memory[0x20 - 2] = 0xF2;
    e.set_register(Register8080::SP, 0x20);
    e.pop_data_off_stack(Register8080::B);
    assert_eq!(e.read_register_pair(Register8080::B), 0x78F2);
    assert_eq!(e.read_register(Register8080::SP), 0x20 + 2);
}

#[test]
fn pop_psw_from_stack()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.main_memory[0x20 - 1] = 0x78;
    e.main_memory[0x20 - 2] = 0x99;
    e.set_register(Register8080::SP, 0x20);
    e.pop_data_off_stack(Register8080::PSW);
    assert_eq!(e.read_register_pair(Register8080::PSW), 0x7899);
    assert_eq!(e.read_register(Register8080::SP), 0x20 + 2);
}

impl Emulator8080 {
    fn _run_opcode(&mut self)
    {
        let mut full_opcode = vec![];
        {
            let pc = self._program_counter as usize;
            let opcode = self.main_memory[pc];
            let size = opcode_size(opcode) as usize;

            full_opcode.resize(size, 0);
            full_opcode.clone_from_slice(&self.main_memory[pc..pc + size]);
            self._program_counter += size as u16;
        }

        dispatch_opcode(&full_opcode, self);
    }

    fn _run(&mut self)
    {
        self._program_counter = ROM_ADDRESS as u16;
        loop {
            self._run_opcode();
        }
    }
}

pub fn run_emulator<'a>(rom: &'a [u8]) {
    let _e = Emulator8080::new(rom);
}

// This test is disabled, but when the emulation is complete, it should pass.
//
// #[cfg(test)]
// use std::fs::File;
// #[cfg(test)]
// use std::io::Read;
//
// #[test]
// fn cpu_diagnostic_8008() {
//     // Load up the ROM
//     let mut rom : Vec<u8> = vec![];
//     {
//         let mut file = File::open("cpudiag.bin").ok().expect("open fail");
//         file.read_to_end(&mut rom).ok().expect("Failed to read ROM");
//     }
//
//     let mut emulator = Emulator8080::new(&rom);
//     emulator._run();
// }
