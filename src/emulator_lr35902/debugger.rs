use std::{fmt, str};
use std::io::{self, Result};

use emulator_8080::{dispatch_8080_instruction, get_8080_instruction, InstructionSetOps8080};
use emulator_common::InstructionOption::*;
use emulator_common::{Register8080, DebuggerOps, Debugger, SimulatedInstruction};
use emulator_lr35902::opcodes::{
    create_disassembler, dispatch_lr35902_instruction, get_lr35902_instruction};
use emulator_lr35902::{EmulatorLR35902, FlagLR35902, InstructionSetOpsLR35902};

struct SimulatedInstructionLR35902<'a> {
    emulator: &'a EmulatorLR35902,
    instruction: &'a mut SimulatedInstruction
}

impl<'a> SimulatedInstructionLR35902<'a> {
    fn new(
        emulator: &'a EmulatorLR35902,
        instruction: &'a mut SimulatedInstruction) -> SimulatedInstructionLR35902<'a>
    {
        SimulatedInstructionLR35902 {
            emulator: emulator,
            instruction: instruction
        }
    }
}

impl<'a> InstructionSetOpsLR35902 for SimulatedInstructionLR35902<'a> {
    fn set_flag(&mut self, _flag: FlagLR35902, _value: bool)
    {
    }

    fn read_flag(&self, flag: FlagLR35902) -> bool
    {
        self.emulator.read_flag(flag)
    }

    fn read_memory(&self, address: u16) -> u8
    {
        self.emulator.read_memory(address)
    }

    fn set_memory(&mut self, address: u16, value: u8)
    {
        self.instruction.set_memory(address, value);
    }

    fn read_memory_u16(&self, address: u16) -> u16
    {
        self.emulator.read_memory_u16(address)
    }

    fn set_memory_u16(&mut self, address: u16, value: u16)
    {
        self.instruction.set_memory(address, (value >> 8) as u8);
        if address != 0xFFFF {
            self.instruction.set_memory(address.wrapping_add(1), value as u8);
        }
    }

    fn read_raw_register(&self, index: usize) -> u8
    {
        self.emulator.read_raw_register(index)
    }

    fn set_raw_register(&mut self, _index: usize, _value: u8)
    {
    }

    fn read_raw_register_pair(&self, index: usize) -> u16
    {
        self.emulator.read_raw_register_pair(index)
    }

    fn set_raw_register_pair(&mut self, _index: usize, _value: u16)
    {

    }

    fn read_program_counter(&self) -> u16
    {
        self.emulator.read_program_counter()
    }

    fn set_program_counter(&mut self, _address: u16)
    {
    }

    fn set_interrupts_enabled(&mut self, _value: bool)
    {
    }

    fn get_interrupts_enabled(&self) -> bool
    {
        self.emulator.get_interrupts_enabled()
    }
}

impl fmt::Debug for EmulatorLR35902 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        try!(writeln!(f, "B: {:x}, C: {:x}, D: {:x}, E: {:x}, H: {:x}, L: {:x}, A: {:x}",
            self.read_register(Register8080::B),
            self.read_register(Register8080::C),
            self.read_register(Register8080::D),
            self.read_register(Register8080::E),
            self.read_register(Register8080::H),
            self.read_register(Register8080::L),
            self.read_register(Register8080::A)));
        try!(writeln!(f, "Zero: {}, Subtract: {}, HalfCarry: {}, Carry: {}",
            self.read_flag(FlagLR35902::Zero),
            self.read_flag(FlagLR35902::Subtract),
            self.read_flag(FlagLR35902::HalfCarry),
            self.read_flag(FlagLR35902::Carry)));
        try!(writeln!(f, "PC: {:x}, SP: {:x}, M: {:x}",
            self.read_program_counter(),
            self.read_register_pair(Register8080::SP),
            self.read_register(Register8080::M)));

        let mut buffer = vec![];
        {
            let mut dis = create_disassembler(&self.main_memory, &mut buffer);
            dis.index = self.read_program_counter() as u64;
            dis.disassemble_one().unwrap();
        }
        try!(write!(f, "{}", str::from_utf8(&buffer).unwrap()));

        Ok(())
    }
}

impl DebuggerOps for EmulatorLR35902 {
    fn read_memory(&self, address: u16) -> u8
    {
        self.read_memory(address)
    }

    fn format<'b>(&self, s: &'b mut io::Write) -> Result<()>
    {
        write!(s, "{:?}", self)
    }

    fn next(&mut self)
    {
        self.run_one_instruction();
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction)
    {
        let pc = self.read_program_counter() as usize;
        let mut instr = get_lr35902_instruction(&self.main_memory[pc..]);
        match instr {
            SomeInstruction(res) => {
                let mut wrapping_instruction = SimulatedInstructionLR35902::new(
                    self, instruction);
                dispatch_lr35902_instruction(&res, &mut wrapping_instruction);
                return;
            },
            NotImplemented => {
                return;
            }
            _ => { },
        }
        instr = get_8080_instruction(&self.main_memory[pc..]);
        match instr {
            SomeInstruction(res) => {
                let mut wrapping_instruction = SimulatedInstructionLR35902::new(
                    self, instruction);
                dispatch_8080_instruction(&res, &mut wrapping_instruction);
            },
            _ => { },
        };
    }

    fn read_program_counter(&self) -> u16
    {
        self.read_program_counter()
    }

    fn crashed(&self) -> Option<&String>
    {
        self.crash_message.as_ref()
    }

    fn set_program_counter(&mut self, address: u16)
    {
        self.set_program_counter(address)
    }
}

pub fn run_debugger(rom: &[u8])
{
    let mut e = EmulatorLR35902::new();
    e.load_rom(&rom);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let mut stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run();
}
