// Copyright 2017 Remi Bernotavicius

use crate::emulator_common::debugger::{Debugger, DebuggerOps, SimulatedInstruction};
use crate::emulator_common::Intel8080Register;
use crate::emulator_common::{MemoryAccessor, SimpleMemoryAccessor};
use crate::io::{self, Result};
use crate::lr35902_emulator::opcodes::LR35902Instruction;
use crate::lr35902_emulator::{LR35902Emulator, LR35902Flag, LR35902InstructionSetOps};
use alloc::{string::String, vec::Vec};
use core::fmt;

struct SimulatedInstructionLR35902<'a, M: MemoryAccessor> {
    emulator: &'a LR35902Emulator,
    memory_accessor: &'a M,
    instruction: &'a mut SimulatedInstruction,
}

impl<'a, M: MemoryAccessor> SimulatedInstructionLR35902<'a, M> {
    fn new(
        emulator: &'a LR35902Emulator,
        memory_accessor: &'a M,
        instruction: &'a mut SimulatedInstruction,
    ) -> SimulatedInstructionLR35902<'a, M> {
        SimulatedInstructionLR35902 {
            emulator,
            memory_accessor,
            instruction,
        }
    }
}

impl<'a, M: MemoryAccessor> LR35902InstructionSetOps for SimulatedInstructionLR35902<'a, M> {
    fn set_flag(&mut self, _flag: LR35902Flag, _value: bool) {}

    fn read_flag(&self, flag: LR35902Flag) -> bool {
        self.emulator.read_flag(flag)
    }

    fn read_memory(&self, address: u16) -> u8 {
        self.memory_accessor.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.instruction.set_memory(address, value);
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        self.memory_accessor.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.instruction.set_memory(address, (value >> 8) as u8);
        if address != 0xFFFF {
            self.instruction
                .set_memory(address.wrapping_add(1), value as u8);
        }
    }

    fn read_raw_register(&self, index: usize) -> u8 {
        self.emulator.read_raw_register(index)
    }

    fn set_raw_register(&mut self, _index: usize, _value: u8) {}

    fn read_raw_register_pair(&self, index: usize) -> u16 {
        self.emulator.read_raw_register_pair(index)
    }

    fn set_raw_register_pair(&mut self, _index: usize, _value: u16) {}

    fn read_program_counter(&self) -> u16 {
        self.emulator.read_program_counter()
    }

    fn set_program_counter(&mut self, _address: u16) {}

    fn set_interrupts_enabled(&mut self, _value: bool) {}

    fn add_cycles(&mut self, _cycles: u8) {}

    fn push_frame(&mut self, _address: u16) {}

    fn pop_frame(&mut self) {}

    fn wait_until_interrupt(&mut self) {}
}

struct Hex<T>(T);

impl fmt::Debug for Hex<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", self.0)
    }
}

impl fmt::Debug for Hex<u16> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

impl fmt::Debug for LR35902Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LR35902Emulator")
            .field("A", &Hex(self.read_register(Intel8080Register::A)))
            .field("B", &Hex(self.read_register(Intel8080Register::B)))
            .field("C", &Hex(self.read_register(Intel8080Register::C)))
            .field("D", &Hex(self.read_register(Intel8080Register::D)))
            .field("E", &Hex(self.read_register(Intel8080Register::E)))
            .field("H", &Hex(self.read_register(Intel8080Register::H)))
            .field("L", &Hex(self.read_register(Intel8080Register::L)))
            .field("Zero", &self.read_flag(LR35902Flag::Zero))
            .field("Subtract", &self.read_flag(LR35902Flag::Subtract))
            .field("HalfCarry", &self.read_flag(LR35902Flag::HalfCarry))
            .field("Carry", &self.read_flag(LR35902Flag::Carry))
            .field("PC", &Hex(self.read_program_counter()))
            .field("SP", &Hex(self.read_register_pair(Intel8080Register::SP)))
            .field("M", &Hex(self.read_register(Intel8080Register::M)))
            .field("clock", &self.elapsed_cycles)
            .finish()
    }
}

pub struct LR35902Debugger<'a, M: MemoryAccessor> {
    emulator: &'a mut LR35902Emulator,
    memory_accessor: &'a mut M,
}

impl<'a, M: MemoryAccessor> LR35902Debugger<'a, M> {
    pub fn new(emulator: &'a mut LR35902Emulator, memory_accessor: &'a mut M) -> Self {
        Self {
            emulator,
            memory_accessor,
        }
    }
}

impl<'a, M: MemoryAccessor> DebuggerOps for LR35902Debugger<'a, M> {
    fn read_memory(&self, address: u16) -> u8 {
        self.memory_accessor.read_memory(address)
    }

    fn format(&self, s: &mut dyn io::Write) -> Result<()> {
        writeln!(s, "{:#?}", &self.emulator)
    }

    fn next(&mut self) {
        self.emulator.run_one_instruction(self.memory_accessor);
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        let pc = self.read_program_counter();
        let maybe_instr = LR35902Instruction::from_memory(self.memory_accessor, pc).unwrap();
        if let Some(res) = maybe_instr {
            let mut wrapping_instruction =
                SimulatedInstructionLR35902::new(self.emulator, self.memory_accessor, instruction);
            res.dispatch(&mut wrapping_instruction);
        }
    }

    fn read_program_counter(&self) -> u16 {
        self.emulator.read_program_counter()
    }

    fn crashed(&self) -> Option<&String> {
        self.emulator.crash_message.as_ref()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.emulator.set_program_counter(address)
    }

    fn disassemble(&mut self, _address: u16, _f: &mut dyn io::Write) -> Result<()> {
        Ok(())
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.emulator.call_stack.clone()
    }
}

pub fn run_debugger(
    rom: &[u8],
    mut input: impl Iterator<Item = io::Result<String>>,
    mut output: impl io::Write,

    is_interrupted: &dyn Fn() -> bool,
) {
    let mut ma = SimpleMemoryAccessor::new();
    ma.memory[0..rom.len()].clone_from_slice(rom);

    let mut e = LR35902Emulator::new();
    let mut d = LR35902Debugger::new(&mut e, &mut ma);

    let mut debugger = Debugger::new(&mut input, &mut output, &mut d);
    debugger.run(is_interrupted);
}
