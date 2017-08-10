// Copyright 2017 Remi Bernotavicius

use std::{fmt, str};
use std::io::{self, Result};

use emulator_common::{
    Debugger,
    DebuggerOps,
    Intel8080Register,
    MemoryAccessor,
    MemoryStream,
    SimulatedInstruction,
    SimpleMemoryAccessor,
};
use lr35902_emulator::opcodes::{
    create_disassembler, dispatch_lr35902_instruction, get_lr35902_instruction};
use lr35902_emulator::{LR35902Emulator, LR35902Flag, LR35902InstructionSetOps};

struct SimulatedInstructionLR35902<'a, M: MemoryAccessor + 'a> {
    emulator: &'a LR35902Emulator<M>,
    instruction: &'a mut SimulatedInstruction
}

impl<'a, M: MemoryAccessor + 'a> SimulatedInstructionLR35902<'a, M> {
    fn new(
        emulator: &'a LR35902Emulator<M>,
        instruction: &'a mut SimulatedInstruction) -> SimulatedInstructionLR35902<'a, M>
    {
        SimulatedInstructionLR35902 {
            emulator: emulator,
            instruction: instruction
        }
    }
}

impl<'a, M: MemoryAccessor> LR35902InstructionSetOps for SimulatedInstructionLR35902<'a, M> {
    fn set_flag(&mut self, _flag: LR35902Flag, _value: bool)
    {
    }

    fn read_flag(&self, flag: LR35902Flag) -> bool
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

impl<M: MemoryAccessor> fmt::Debug for LR35902Emulator<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        try!(writeln!(f, "B: {:x}, C: {:x}, D: {:x}, E: {:x}, H: {:x}, L: {:x}, A: {:x}",
            self.read_register(Intel8080Register::B),
            self.read_register(Intel8080Register::C),
            self.read_register(Intel8080Register::D),
            self.read_register(Intel8080Register::E),
            self.read_register(Intel8080Register::H),
            self.read_register(Intel8080Register::L),
            self.read_register(Intel8080Register::A)));
        try!(writeln!(f, "Zero: {}, Subtract: {}, HalfCarry: {}, Carry: {}",
            self.read_flag(LR35902Flag::Zero),
            self.read_flag(LR35902Flag::Subtract),
            self.read_flag(LR35902Flag::HalfCarry),
            self.read_flag(LR35902Flag::Carry)));
        try!(writeln!(f, "PC: {:x}, SP: {:x}, M: {:x}",
            self.read_program_counter(),
            self.read_register_pair(Intel8080Register::SP),
            self.read_register(Intel8080Register::M)));

        let mut buffer = vec![];
        {
            let mut dis = create_disassembler(&self.memory_accessor, &mut buffer);
            dis.index = self.read_program_counter();
            dis.disassemble_one(true).unwrap();
        }
        try!(write!(f, "{}", str::from_utf8(&buffer).unwrap()));

        Ok(())
    }
}

impl<M: MemoryAccessor> DebuggerOps for LR35902Emulator<M> {
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
        let pc = self.read_program_counter();
        let stream = MemoryStream::new(&self.memory_accessor, pc);
        let instr = get_lr35902_instruction(stream);
        match instr {
            Some(res) => {
                let mut wrapping_instruction = SimulatedInstructionLR35902::new(
                    self, instruction);
                dispatch_lr35902_instruction(&res, &mut wrapping_instruction);
                return;
            },
            None => { },
        }
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
    let mut ma = SimpleMemoryAccessor::new();
    ma.memory[0..rom.len()].clone_from_slice(rom);
    let mut e = LR35902Emulator::new(ma);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let mut stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run();
}
