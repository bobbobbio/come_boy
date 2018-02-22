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
    GameBoyRegister,
    LCDControlFlag,
    LCDController,
    LCDStatusFlag,
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
        // XXX: I don't like how this mapping information is repeated here.
        try!(fmt_lcd_register(0xFF40, &self.lcdc, "LCDC", f));
        try!(fmt_lcd_register(0xFF41, &self.stat, "STAT", f));
        try!(fmt_lcd_register(0xFF42, &self.scy, "SCY", f));
        try!(fmt_lcd_register(0xFF43, &self.scx, "SCX", f));
        try!(fmt_lcd_register(0xFF44, &self.ly, "LY", f));
        try!(fmt_lcd_register(0xFF45, &self.lyc, "LYC", f));
        try!(fmt_lcd_register(0xFF46, &self.dma, "DMA", f));
        try!(fmt_lcd_register(0xFF47, &self.bgp, "BGP", f));
        try!(fmt_lcd_register(0xFF48, &self.obp0, "OBP0", f));
        try!(fmt_lcd_register(0xFF49, &self.obp1, "OBP1", f));
        try!(fmt_lcd_register(0xFF4A, &self.wy, "WX", f));
        try!(fmt_lcd_register(0xFF4B, &self.wx, "WY", f));

        Ok(())
    }
}

fn fmt_lcd_register<'a>(
    address: u16, register: &GameBoyRegister, name: &str, f: &mut fmt::Formatter) -> fmt::Result
{
    try!(write!(f, "{} ({:02x}): ", name, address));

    match name {
        "LCDC" => try!(fmt_lcdc(register.read_value(), f)),
        "STAT" => try!(fmt_stat(register.read_value(), f)),
        _ => try!(write!(f, "{:02x}", register.read_value())),
    }

    try!(writeln!(f));

    Ok(())
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
