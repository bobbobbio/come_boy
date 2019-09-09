use std::io::{self, Result};
use std::{fmt, str};

use crate::emulator_common::debugger::{Debugger, DebuggerOps, SimulatedInstruction};
use crate::emulator_common::disassembler::{Disassembler, MemoryAccessor};
use crate::game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;
use crate::game_boy_emulator::game_pak::GamePak;
use crate::game_boy_emulator::lcd_controller::{LCDControlFlag, LCDController, LCDStatusFlag};
use crate::game_boy_emulator::memory_controller::GameBoyMemoryMap;
use crate::game_boy_emulator::GameBoyEmulator;
use crate::lr35902_emulator::debugger::LR35902Debugger;
use crate::rendering::{sdl2::Sdl2WindowRenderer, Renderer};

impl<'a, R> fmt::Debug for GameBoyEmulator<'a, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.cpu)?;
        writeln!(f)?;
        write!(f, "{:?}", self.lcd_controller)?;

        Ok(())
    }
}

impl<'a, R: Renderer> DebuggerOps for GameBoyEmulator<'a, R> {
    fn read_memory(&self, address: u16) -> u8 {
        let memory_map = game_boy_memory_map!(self);
        memory_map.read_memory(address)
    }

    fn format<'b>(&self, s: &'b mut dyn io::Write) -> Result<()> {
        write!(s, "{:?}", self)
    }

    fn next(&mut self) {
        self.tick();
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        let mut memory_map = game_boy_memory_map!(self);
        let mut d = LR35902Debugger::new(&mut self.cpu, &mut memory_map);
        d.simulate_next(instruction);
    }

    fn read_program_counter(&self) -> u16 {
        self.cpu.read_program_counter()
    }

    fn crashed(&self) -> Option<&String> {
        self.crashed()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.cpu.set_program_counter(address)
    }

    fn disassemble(&mut self, address: u16, f: &mut dyn io::Write) -> Result<()> {
        let mut buffer = vec![];
        let memory_map = game_boy_memory_map!(self);
        let mut dis = Disassembler::new(&memory_map, RGBDSInstructionPrinterFactory, &mut buffer);
        dis.index = address;
        dis.disassemble_multiple().unwrap();
        write!(f, "{}", str::from_utf8(&buffer).unwrap())
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.cpu.call_stack.clone()
    }
}

impl<'a, R> fmt::Debug for LCDController<'a, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // XXX: I don't like how this mapping information is repeated here.
        fmt_lcd_register(0xFF40, self.registers.lcdc.read_value(), "LCDC", f)?;
        fmt_lcd_register(0xFF41, self.registers.stat.read_value(), "STAT", f)?;
        fmt_lcd_register(0xFF42, self.registers.scy.read_value(), "SCY", f)?;
        fmt_lcd_register(0xFF43, self.registers.scx.read_value(), "SCX", f)?;
        fmt_lcd_register(0xFF44, self.registers.ly.read_value(), "LY", f)?;
        fmt_lcd_register(0xFF45, self.registers.lyc.read_value(), "LYC", f)?;
        fmt_lcd_register(0xFF46, self.registers.dma.read_value(), "DMA", f)?;
        fmt_lcd_register(0xFF47, self.registers.bgp.read_value(), "BGP", f)?;
        fmt_lcd_register(0xFF48, self.registers.obp0.read_value(), "OBP0", f)?;
        fmt_lcd_register(0xFF49, self.registers.obp1.read_value(), "OBP1", f)?;
        fmt_lcd_register(0xFF4A, self.registers.wy.read_value(), "WX", f)?;
        fmt_lcd_register(0xFF4B, self.registers.wx.read_value(), "WY", f)?;

        Ok(())
    }
}

fn fmt_lcd_register<'a>(
    address: u16,
    value: u8,
    name: &str,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    write!(f, "{} ({:02x}): ", name, address)?;

    match name {
        "LCDC" => fmt_lcdc(value, f)?,
        "STAT" => fmt_stat(value, f)?,
        _ => write!(f, "{:02x}", value)?,
    }

    writeln!(f)?;

    Ok(())
}

pub fn fmt_lcdc(lcdc: u8, f: &mut fmt::Formatter) -> fmt::Result {
    let all = [
        LCDControlFlag::DisplayOn,
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

pub fn fmt_stat(stat: u8, f: &mut fmt::Formatter) -> fmt::Result {
    let all = [
        LCDStatusFlag::InterruptLYMatching,
        LCDStatusFlag::InterruptMode10,
        LCDStatusFlag::InterruptMode01,
        LCDStatusFlag::InterruptMode00,
        LCDStatusFlag::LYMatch,
    ];
    let mut set = vec![];

    for f in all.iter() {
        if *f as u8 & stat != 0 {
            set.push(*f);
        }
    }
    write!(f, "{:02x}: {:?}", stat, set)?;
    write!(f, " Mode{}", stat & 0x3)
}

pub fn run_debugger(game_pak: GamePak, pixel_scale: u32, is_interrupted: &dyn Fn() -> bool) {
    let window_title = "come boy (in debugger)";
    let mut e = GameBoyEmulator::new(Sdl2WindowRenderer::new(pixel_scale, window_title, 160, 144));
    e.load_game_pak(game_pak);
    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let stdout = &mut io::stdout();

    let mut debugger = Debugger::new(stdin_locked, stdout, &mut e);
    debugger.run(is_interrupted);
}
