use std::{fmt, str};
use std::io::{self, Result};

use emulator_common::{
    Debugger,
    DebuggerOps,
    Disassembler,
    SimulatedInstruction
};
use game_boy_emulator::{
    GameBoyEmulator,
    LCDControlFlag,
    LCDController,
    LCDRegister,
    LCDStatusFlag,
    LCD_REGISTERS,
};

use game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;

impl<'a> fmt::Debug for GameBoyEmulator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        try!(writeln!(f, "{:?}", self.cpu));

        try!(writeln!(f));

        try!(write!(f, "{:?}", self.lcd_controller));

        Ok(())
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

    fn disassemble(&mut self, f: &mut io::Write) -> Result<()>
    {
        let mut buffer = vec![];
        {
            let mut dis = Disassembler::new(
                &self.cpu.memory_accessor, RGBDSInstructionPrinterFactory, &mut buffer);
            dis.index = self.cpu.read_program_counter();
            dis.disassemble_multiple().unwrap();
        }
        write!(f, "{}", str::from_utf8(&buffer).unwrap())
    }
}

impl<'a> fmt::Debug for LCDController<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let registers = [
            LCDRegister::LCDC,
            LCDRegister::STAT,
            LCDRegister::SCY,
            LCDRegister::SCX,
            LCDRegister::LY,
            LCDRegister::LYC,
            LCDRegister::DMA,
            LCDRegister::BGP,
            LCDRegister::OBP0,
            LCDRegister::OBP1,
            LCDRegister::WY,
            LCDRegister::WX,
        ];

        for r in &registers {
            let value = self.read_register(*r);
            try!(write!(
                f, "{:05} ({:02X}): ", format!("{:?}", *r), LCD_REGISTERS.start + *r as u16));
            try!(fmt_lcd_register(*r, value, f));
            try!(writeln!(f));
        }

        Ok(())
    }
}

fn fmt_lcd_register(register: LCDRegister, value: u8, f: &mut fmt::Formatter) -> fmt::Result
{
    match register {
        LCDRegister::LCDC => fmt_lcdc(value, f),
        LCDRegister::STAT => fmt_stat(value, f),
        _ => write!(f, "{:02x}", value)
    }
}

fn fmt_lcdc(lcdc: u8, f: &mut fmt::Formatter) -> fmt::Result
{
    let all = [
        LCDControlFlag::OperationStop,
        LCDControlFlag::WindowCodeAreaSelection,
        LCDControlFlag::WindowingOn,
        LCDControlFlag::BGCharacterDataSelection,
        LCDControlFlag::BGCodeAreaSelection,
        LCDControlFlag::ObjectBlockCompositionSelection,
        LCDControlFlag::ObjectOn,
        LCDControlFlag::BGDisplayOn,
    ];
    let mut set = vec![];

    for f in all.iter() {
        if *f as u8 & lcdc != 0 {
            set.push(*f);
        }
    }
    write!(f, "{:02x}: {:?}", lcdc, set)
}

fn fmt_stat(stat: u8, f: &mut fmt::Formatter) -> fmt::Result
{
    let all = [
        LCDStatusFlag::InterruptLYMatching,
        LCDStatusFlag::InterruptMode10,
        LCDStatusFlag::InterruptMode01,
        LCDStatusFlag::InterruptMode00,
        LCDStatusFlag::LYMatch,
        LCDStatusFlag::Mode,
    ];
    let mut set = vec![];

    for f in all.iter() {
        if *f as u8 & stat != 0 {
            set.push(*f);
        }
    }
    write!(f, "{:02x}: {:?}", stat, set)
}

pub fn run_debugger(rom: &[u8], is_interrupted: &Fn() -> bool)
{
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let stdout = &mut io::stdout();
    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run(is_interrupted);
}
