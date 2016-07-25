pub mod opcodes;

use std::io::Result;
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
    fn subtract_from_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn return_if_not_zero(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn add_immediate_to_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn pop_data_off_stack(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn add_to_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_parity_even(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn move_data(&mut self, _register1: Register8080, _register2: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn double_add(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn or_immediate_with_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn call_if_carry(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn jump(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn logical_or(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn rim(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn call_if_parity_even(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_positive(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_zero(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn no_instruction(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn disable_interrupts(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn set_carry(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn compare_with_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn call_if_not_zero(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn call_if_parity_odd(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn subtract_immediate_from_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn rotate_accumulator_left_through_carry(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn load_sp_from_h_and_l(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn logical_and_with_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn load_h_and_l_direct(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn add_immediate_with_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn call(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn enable_interrupts(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn load_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn input(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_parity_odd(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn increment_register_pair(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn logical_exclusive_or(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn exchange_registers(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn rotate_accumulator_right(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn call_if_no_carry(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn return_if_parity_even(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn call_if_zero(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn return_unconditionally(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn halt(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn call_if_plus(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn increment_register_or_memory(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn compare_immediate_with_accumulator(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn load_program_counter(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn return_if_minus(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_carry(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn call_if_minus(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn decimal_adjust_accumulator(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn load_register_pair_immediate(&mut self, _register1: Register8080, _data2: u16) -> Result<()>
    {
        Ok(())
    }
    fn move_immediate_data(&mut self, _register1: Register8080, _data2: u8) -> Result<()>
    {
        Ok(())
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn return_if_plus(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn restart(&mut self, _implicit_data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn store_accumulator_direct(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_not_zero(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_minus(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn decrement_register_or_memory(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn output(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn store_accumulator(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn add_to_accumulator_with_carry(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn return_if_carry(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn complement_accumulator(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn return_if_no_carry(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn return_if_zero(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn return_if_parity_odd(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn store_h_and_l_direct(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn subtract_from_accumulator_with_borrow(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn not_implemented(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn push_data_onto_stack(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn jump_if_no_carry(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn sim(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn decrement_register_pair(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn complement_carry(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn rotate_accumulator_left(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn load_accumulator_direct(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn exchange_stack(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn rotate_accumulator_right_through_carry(&mut self) -> Result<()>
    {
        Ok(())
    }

}

impl Z8080Emulator {
    fn _run_opcode(&mut self, stream: &[u8]) -> Result<u8>
    {
        Ok(try!(dispatch_opcode(stream, self)))
    }
}

pub fn run_emulator<'a>(rom: &'a [u8]) {
    let _e = Z8080Emulator::new(rom);
}
