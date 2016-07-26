pub mod opcodes;

use z8080::opcodes::opcode_gen::{InstructionSet8080, Register8080, dispatch_opcode};

const MAX_ADDRESS: u16 = 0xffff;
const ROM_ADDRESS: u16 = 0x0100;

struct Z8080Emulator {
    main_memory: [u8; MAX_ADDRESS as usize + 1],
    _registers: [u8; Register8080::Count as usize]
}

impl Z8080Emulator {
    fn new(rom: &[u8]) -> Z8080Emulator
    {
        let mut emu = Z8080Emulator {
            main_memory: [0; MAX_ADDRESS as usize + 1],
            _registers: [0; Register8080::Count as usize]
        };

        emu.main_memory[
            ROM_ADDRESS as usize..(ROM_ADDRESS as usize + rom.len())].clone_from_slice(rom);

        return emu;
    }
}

impl InstructionSet8080 for Z8080Emulator {
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
    fn set_carry(&mut self)
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
    fn increment_register_or_memory(&mut self, _register1: Register8080)
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
    fn complement_carry(&mut self)
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

impl Z8080Emulator {
    fn _run_opcode(&mut self, stream: &[u8]) -> u8
    {
        dispatch_opcode(stream, self)
    }
}

pub fn run_emulator<'a>(rom: &'a [u8]) {
    let _e = Z8080Emulator::new(rom);
}
