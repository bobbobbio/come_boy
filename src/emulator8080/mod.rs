pub mod opcodes;

use std::mem;

use emulator8080::opcodes::opcode_gen::{InstructionSet8080, Register8080, dispatch_opcode};

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

    fn add_to_register(
        &mut self,
        register: Register8080,
        value: u8,
        update_carry: bool)
    {
        let old_value = self.read_register(register);
        let new_value = old_value.wrapping_add(value);
        self.set_register(register, new_value);
        self.update_flags_for_new_value(new_value);

        if update_carry {
            self.set_flag(Flag8080::Carry, value > 0xFF - old_value);
        }

        self.set_flag(Flag8080::AuxiliaryCarry, new_value & 0x0F < old_value & 0x0F);
    }

    fn subtract_from_register(&mut self, register: Register8080, value: u8, update_carry: bool)
    {
        let old_value = self.read_register(register);
        let new_value = old_value.wrapping_sub(value);
        self.set_register(register, new_value);
        self.update_flags_for_new_value(new_value);
        self.set_flag(Flag8080::AuxiliaryCarry, false);
        if update_carry {
            self.set_flag(Flag8080::Carry, false);
        }
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
    delta: u8,
    update_carry: bool)
{
    e.set_register(register, starting_value);
    e.subtract_from_register(register, delta, update_carry);
}

#[test]
fn subtract_from_register_underflows()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::C, 0x00, 2, true /* update carry */);
    assert_eq!(e.read_register(Register8080::C), 0xFE);
}

#[test]
fn subtract_from_register_doesnt_set_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x99, 1, true /* update carry */);
    assert!(!e.read_flag(Flag8080::Zero));
}

#[test]
fn subtract_from_register_update_sets_zero_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x01, 1, true /* update carry */);
    assert!(e.read_flag(Flag8080::Zero));
}

#[test]
fn subtract_from_register_doesnt_set_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x0B, 1, true /* update carry */);
    assert!(!e.read_flag(Flag8080::Sign));
}

#[test]
fn subtract_from_register_sets_sign_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x00, 1, true /* update carry */);
    assert!(e.read_flag(Flag8080::Sign));
}

#[test]
fn subtract_from_register_clears_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());

    e.set_flag(Flag8080::Parity, true);
    subtract_from_register_test(&mut e, Register8080::B, 0x02, 1, true /* update carry */);

    // 00000001 -> odd parity = false
    assert!(!e.read_flag(Flag8080::Parity));
}

#[test]
fn subtract_from_register_sets_parity_flag()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    subtract_from_register_test(&mut e, Register8080::B, 0x04, 1, true /* update carry */);

    // 00000011 -> even parity = true
    assert!(e.read_flag(Flag8080::Parity));
}

#[test]
fn subtract_from_register_never_sets_auxiliary_carry()
{
    for delta in 0x00..0xFF {
        let mut e = Emulator8080::new(vec![].as_slice());
        subtract_from_register_test(&mut e, Register8080::C, 0x88, delta, true /* update carry */);
        assert!(!e.read_flag(Flag8080::AuxiliaryCarry));
    }
}

#[test]
fn subtract_from_register_doesnt_set_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    subtract_from_register_test(&mut e, Register8080::C, 0x88, 4, false /* update carry */);
    assert!(e.read_flag(Flag8080::Carry));
}

#[test]
fn subtract_from_register_updates_carry()
{
    let mut e = Emulator8080::new(vec![].as_slice());
    e.set_flag(Flag8080::Carry, true);
    subtract_from_register_test(&mut e, Register8080::C, 0x88, 4, true /* update carry */);
    assert!(!e.read_flag(Flag8080::Carry));
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
        self.subtract_from_register(register, 1, false, /* update carry */);
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
    fn output(&mut self, _data1: u8)
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

impl Emulator8080 {
    fn _run_opcode(&mut self, stream: &[u8]) -> u8
    {
        dispatch_opcode(stream, self)
    }
}

pub fn run_emulator<'a>(rom: &'a [u8]) {
    let _e = Emulator8080::new(rom);
}
