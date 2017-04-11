use std::fmt;
use std::io::{self, Result};

use emulator_common::{Debugger, DebuggerOps, SimulatedInstruction};
use game_boy_emulator::GameBoyEmulator;

impl<'a> fmt::Debug for GameBoyEmulator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        writeln!(f, "{:?}", self.cpu)
    }
}

impl<'a> DebuggerOps for GameBoyEmulator<'a> {
    fn read_memory(&self, address: u16) -> u8
    {
        self.cpu.read_memory(address)
    }

    fn format<'b>(&self, s: &'b mut io::Write) -> Result<()>
    {
        write!(s, "{:?}", self)
    }

    fn next(&mut self)
    {
        self.tick();
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction)
    {
        self.cpu.simulate_next(instruction);
    }

    fn read_program_counter(&self) -> u16
    {
        self.cpu.read_program_counter()
    }

    fn crashed(&self) -> Option<&String>
    {
        self.crashed()
    }

    fn set_program_counter(&mut self, address: u16)
    {
        self.cpu.set_program_counter(address)
    }
}

pub fn run_debugger(rom: &[u8])
{
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let mut stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run();
}
