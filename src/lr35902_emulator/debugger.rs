// Copyright 2017 Remi Bernotavicius

use std::fmt;
use std::io::{self, Result};

use crate::emulator_common::debugger::{Debugger, DebuggerOps, SimulatedInstruction};
use crate::emulator_common::disassembler::{MemoryAccessor, MemoryStream, SimpleMemoryAccessor};
use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::opcodes::LR35902Instruction;
use crate::lr35902_emulator::{LR35902Emulator, LR35902Flag, LR35902InstructionSetOps};

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

    fn get_interrupts_enabled(&self) -> bool {
        self.emulator.get_interrupts_enabled()
    }

    fn add_cycles(&mut self, _cycles: u8) {}

    fn push_frame(&mut self, _address: u16) {}

    fn pop_frame(&mut self) {}

    fn wait_until_interrupt(&mut self) {}
}

impl fmt::Debug for LR35902Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "B: {:x}, C: {:x}, D: {:x}, E: {:x}, H: {:x}, L: {:x}, A: {:x}",
            self.read_register(Intel8080Register::B),
            self.read_register(Intel8080Register::C),
            self.read_register(Intel8080Register::D),
            self.read_register(Intel8080Register::E),
            self.read_register(Intel8080Register::H),
            self.read_register(Intel8080Register::L),
            self.read_register(Intel8080Register::A)
        )?;
        writeln!(
            f,
            "Zero: {}, Subtract: {}, HalfCarry: {}, Carry: {}",
            self.read_flag(LR35902Flag::Zero),
            self.read_flag(LR35902Flag::Subtract),
            self.read_flag(LR35902Flag::HalfCarry),
            self.read_flag(LR35902Flag::Carry)
        )?;
        write!(
            f,
            "PC: {:x}, SP: {:x}, M: {:x}",
            self.read_program_counter(),
            self.read_register_pair(Intel8080Register::SP),
            self.read_register(Intel8080Register::M)
        )?;

        Ok(())
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

    fn format<'b>(&self, s: &'b mut dyn io::Write) -> Result<()> {
        writeln!(s, "{:?}", &self.emulator)
    }

    fn next(&mut self) {
        self.emulator.run_one_instruction(self.memory_accessor);
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        let pc = self.read_program_counter();
        let maybe_instr =
            LR35902Instruction::from_reader(MemoryStream::new(self.memory_accessor, pc)).unwrap();
        match maybe_instr {
            Some(res) => {
                let mut wrapping_instruction = SimulatedInstructionLR35902::new(
                    self.emulator,
                    self.memory_accessor,
                    instruction,
                );
                res.dispatch(&mut wrapping_instruction);
                return;
            }
            None => {}
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

pub fn run_debugger(rom: &[u8], is_interrupted: &dyn Fn() -> bool) {
    let mut ma = SimpleMemoryAccessor::new();
    ma.memory[0..rom.len()].clone_from_slice(rom);
    let mut e = LR35902Emulator::new();
    let mut d = LR35902Debugger::new(&mut e, &mut ma);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut d);
    debugger.run(is_interrupted);
}
