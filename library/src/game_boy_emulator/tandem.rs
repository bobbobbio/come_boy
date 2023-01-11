// Copyright 2018 Remi Bernotavicius

use crate::emulator_common::disassembler::Disassembler;
use crate::emulator_common::Intel8080Register;
use crate::game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;
use crate::game_boy_emulator::{
    GameBoyEmulator, GameBoyOps, GamePak, LR35902Flag, NullGameBoyOps, Result,
};
use crate::io::{self, Bytes, Read, Write};
use crate::rendering::NullRenderer;
use crate::sound::NullSoundStream;
use crate::storage::{OpenMode, PersistentStorage};
use alloc::vec;
use core::fmt::{self, Debug};
use core::str;

#[derive(Clone, Copy, Default, PartialEq)]
struct AbstractEmulatorRegisters {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: u8,
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
}

pub fn fmt_flags(flags: u8, f: &mut fmt::Formatter) -> fmt::Result {
    let all = [
        LR35902Flag::Zero,
        LR35902Flag::Subtract,
        LR35902Flag::HalfCarry,
        LR35902Flag::Carry,
    ];
    let mut set = vec![];

    for f in all.iter() {
        if *f as u8 & flags != 0 {
            set.push(*f);
        }
    }
    write!(f, "0x{flags:02x}: {set:?}")
}

impl Debug for AbstractEmulatorRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let le = if f.alternate() { "\n" } else { "" };
        let sep = if f.alternate() { "    " } else { " " };
        write!(f, "AbstractEmulatorRegisters {{{le}")?;
        write!(f, "{sep}pc: 0x{:x},{le}", self.pc)?;
        write!(f, "{sep}sp: 0x{:x},{le}", self.sp)?;
        write!(f, "{sep}a: 0x{:x},{le}", self.a)?;
        write!(f, "{sep}b: 0x{:x},{le}", self.b)?;
        write!(f, "{sep}c: 0x{:x},{le}", self.c)?;
        write!(f, "{sep}d: 0x{:x},{le}", self.d)?;
        write!(f, "{sep}e: 0x{:x},{le}", self.e)?;
        write!(f, "{sep}h: 0x{:x},{le}", self.h)?;
        write!(f, "{sep}l: 0x{:x},{le}", self.l)?;
        write!(f, "{sep}flags: ")?;
        fmt_flags(self.flags, f)?;
        write!(f, "{le}")?;

        write!(f, "{sep}lcdc: 0x{:x},{le}", self.lcdc)?;
        write!(f, "{sep}stat: 0x{:x},{le}", self.stat)?;

        write!(f, "{sep}scy: 0x{:x},{le}", self.scy)?;
        write!(f, "{sep}scx: 0x{:x},{le}", self.scx)?;
        write!(f, "{sep}ly: 0x{:x},{le}", self.ly)?;
        write!(f, "{sep}lyc: 0x{:x},{le}", self.lyc)?;
        write!(f, "{sep}dma: 0x{:x},{le}", self.dma)?;
        write!(f, "{sep}bgp: 0x{:x},{le}", self.bgp)?;
        write!(f, "{sep}obp0: 0x{:x},{le}", self.obp0)?;
        write!(f, "{sep}obp1: 0x{:x},{le}", self.obp1)?;
        write!(f, "{sep}wx: 0x{:x},{le}", self.wx)?;
        write!(f, "{sep}wy: 0x{:x},{le}", self.wy)?;
        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AbstractEmulatorState {
    registers: AbstractEmulatorRegisters,
    hash: u32,
    clock: u64,
}

trait AbstractEmulator {
    fn run_one(&mut self);
    fn get_state(&self) -> Option<AbstractEmulatorState>;
    fn write_memory(&self, w: &mut dyn Write) -> Result<()>;
}

fn compare_emulators<A: AbstractEmulator, B: AbstractEmulator>(
    a: &mut A,
    b: &mut B,
    pc_only: bool,
) -> (
    Option<AbstractEmulatorState>,
    Option<AbstractEmulatorState>,
    u64,
) {
    let compare = if pc_only {
        fn pc_compare(
            a: &Option<AbstractEmulatorState>,
            b: &Option<AbstractEmulatorState>,
        ) -> bool {
            a.map(|v| v.registers.pc) == b.map(|v| v.registers.pc)
        }
        pc_compare
    } else {
        PartialEq::eq
    };

    let mut runs = 0;
    let (mut a_state, mut b_state) = (a.get_state(), b.get_state());
    while compare(&a_state, &b_state) {
        runs += 1;
        a.run_one();
        b.run_one();
        a_state = a.get_state();
        b_state = b.get_state();
    }

    (a_state, b_state, runs)
}

#[cfg(test)]
struct TestEmulator {
    states: Vec<AbstractEmulatorState>,
    state: AbstractEmulatorState,
}

#[cfg(test)]
impl TestEmulator {
    fn new(mut states: Vec<AbstractEmulatorState>) -> TestEmulator {
        TestEmulator {
            state: states.remove(0),
            states,
        }
    }
}

#[cfg(test)]
impl AbstractEmulator for TestEmulator {
    fn run_one(&mut self) {
        self.state = self.states.pop().unwrap()
    }

    fn get_state(&self) -> Option<AbstractEmulatorState> {
        Some(self.state)
    }

    fn write_memory(&self, _w: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
const TEST_STATE1: AbstractEmulatorState = AbstractEmulatorState {
    registers: AbstractEmulatorRegisters {
        pc: 0,
        sp: 0,
        a: 0x11,
        b: 0x12,
        c: 0x13,
        d: 0x14,
        e: 0x15,
        h: 0x16,
        l: 0x17,
        flags: 0x18,
        lcdc: 0,
        stat: 0,
        scy: 0,
        scx: 0,
        ly: 0,
        lyc: 0,
        dma: 0,
        bgp: 0,
        obp0: 0,
        obp1: 0,
        wy: 0,
        wx: 0,
    },
    hash: 1,
    clock: 0,
};

#[cfg(test)]
const TEST_STATE2: AbstractEmulatorState = AbstractEmulatorState {
    registers: AbstractEmulatorRegisters {
        pc: 0,
        sp: 0,
        a: 0x21,
        b: 0x22,
        c: 0x23,
        d: 0x24,
        e: 0x25,
        h: 0x26,
        l: 0x27,
        flags: 0x28,
        lcdc: 0,
        stat: 0,
        scy: 0,
        scx: 0,
        ly: 0,
        lyc: 0,
        dma: 0,
        bgp: 0,
        obp0: 0,
        obp1: 0,
        wy: 0,
        wx: 0,
    },
    hash: 2,
    clock: 0,
};

#[cfg(test)]
const TEST_STATE3: AbstractEmulatorState = AbstractEmulatorState {
    registers: AbstractEmulatorRegisters {
        pc: 0,
        sp: 0,
        a: 0x31,
        b: 0x32,
        c: 0x33,
        d: 0x34,
        e: 0x35,
        h: 0x36,
        l: 0x37,
        flags: 0x38,
        lcdc: 0,
        stat: 0,
        scy: 0,
        scx: 0,
        ly: 0,
        lyc: 0,
        dma: 0,
        bgp: 0,
        obp0: 0,
        obp1: 0,
        wy: 0,
        wx: 0,
    },
    hash: 3,
    clock: 0,
};

#[test]
fn compares_states() {
    let mut te1 = TestEmulator::new(vec![TEST_STATE1, TEST_STATE2]);
    let mut te2 = TestEmulator::new(vec![TEST_STATE1, TEST_STATE3]);
    let (a_state, b_state, _) = compare_emulators(&mut te1, &mut te2, false);
    assert_ne!(a_state, b_state);
    assert_eq!(a_state, Some(TEST_STATE2));
    assert_eq!(b_state, Some(TEST_STATE3));
}

struct TandemGameBoyEmulator {
    ops: NullGameBoyOps,
    emulator: GameBoyEmulator,
}

impl TandemGameBoyEmulator {
    fn new() -> Self {
        Self {
            ops: GameBoyOps::null(),
            emulator: GameBoyEmulator::new(),
        }
    }
}

impl AbstractEmulator for TandemGameBoyEmulator {
    fn run_one(&mut self) {
        self.emulator.tick(&mut self.ops);
    }

    fn get_state(&self) -> Option<AbstractEmulatorState> {
        let hash = self.emulator.hash(&self.ops);

        let cpu = &self.emulator.cpu;
        let lcd_controller = &self.emulator.bridge.lcd_controller;

        Some(AbstractEmulatorState {
            registers: AbstractEmulatorRegisters {
                pc: cpu.read_program_counter(),
                sp: cpu.read_register_pair(Intel8080Register::SP),
                a: cpu.read_register(Intel8080Register::A),
                b: cpu.read_register(Intel8080Register::B),
                c: cpu.read_register(Intel8080Register::C),
                d: cpu.read_register(Intel8080Register::D),
                e: cpu.read_register(Intel8080Register::E),
                h: cpu.read_register(Intel8080Register::H),
                l: cpu.read_register(Intel8080Register::L),
                flags: cpu.read_register(Intel8080Register::FLAGS),
                lcdc: lcd_controller.registers.lcdc.read_value(),
                stat: lcd_controller.registers.stat.read_value(),
                scy: lcd_controller.registers.scy.read_value(),
                scx: lcd_controller.registers.scx.read_value(),
                ly: lcd_controller.registers.ly.read_value(),
                lyc: lcd_controller.registers.lyc.read_value(),
                dma: lcd_controller.registers.dma.read_value(),
                bgp: lcd_controller.registers.bgp.read_value(),
                obp0: lcd_controller.registers.obp0.read_value(),
                obp1: lcd_controller.registers.obp1.read_value(),
                wy: lcd_controller.registers.wy.read_value(),
                wx: lcd_controller.registers.wx.read_value(),
            },
            clock: cpu.elapsed_cycles,
            hash,
        })
    }

    fn write_memory(&self, w: &mut dyn Write) -> Result<()> {
        self.emulator.write_memory(&self.ops, w)?;
        Ok(())
    }
}

struct EmulatorReplayer<R: Read> {
    bytes: Bytes<R>,
    state: Option<AbstractEmulatorState>,
    memory: [u8; 0x10000],
}

impl<T: Read> EmulatorReplayer<T> {
    fn new(read: T) -> EmulatorReplayer<T> {
        let mut er = EmulatorReplayer {
            bytes: read.bytes(),
            state: None,
            memory: [0u8; 0x10000],
        };
        er.run_one();

        // We must have an initial state
        assert!(er.state.is_some());

        er
    }
}

impl<R: Read> AbstractEmulator for EmulatorReplayer<R> {
    fn run_one(&mut self) {
        let first_byte = match self.bytes.next() {
            Some(n) => n.unwrap(),
            None => {
                self.state = None;
                return;
            }
        };
        self.state = Some(AbstractEmulatorState {
            registers: AbstractEmulatorRegisters {
                pc: (first_byte as u16) << 8 | (self.bytes.next().unwrap().unwrap() as u16),
                sp: (self.bytes.next().unwrap().unwrap() as u16) << 8
                    | (self.bytes.next().unwrap().unwrap() as u16),
                a: self.bytes.next().unwrap().unwrap(),
                b: self.bytes.next().unwrap().unwrap(),
                c: self.bytes.next().unwrap().unwrap(),
                d: self.bytes.next().unwrap().unwrap(),
                e: self.bytes.next().unwrap().unwrap(),
                h: self.bytes.next().unwrap().unwrap(),
                l: self.bytes.next().unwrap().unwrap(),
                flags: self.bytes.next().unwrap().unwrap(),
                ..Default::default()
            },
            hash: (self.bytes.next().unwrap().unwrap() as u32) << 24
                | (self.bytes.next().unwrap().unwrap() as u32) << 16
                | (self.bytes.next().unwrap().unwrap() as u32) << 8
                | (self.bytes.next().unwrap().unwrap() as u32),

            clock: (self.bytes.next().unwrap().unwrap() as u64) << 56
                | (self.bytes.next().unwrap().unwrap() as u64) << 48
                | (self.bytes.next().unwrap().unwrap() as u64) << 40
                | (self.bytes.next().unwrap().unwrap() as u64) << 32
                | (self.bytes.next().unwrap().unwrap() as u64) << 24
                | (self.bytes.next().unwrap().unwrap() as u64) << 16
                | (self.bytes.next().unwrap().unwrap() as u64) << 8
                | (self.bytes.next().unwrap().unwrap() as u64),
        });

        let num = (self.bytes.next().unwrap().unwrap() as u32) << 16
            | (self.bytes.next().unwrap().unwrap() as u32) << 8
            | (self.bytes.next().unwrap().unwrap() as u32);

        for _ in 0..num {
            let address = (self.bytes.next().unwrap().unwrap() as u16) << 8
                | (self.bytes.next().unwrap().unwrap() as u16);
            let value = self.bytes.next().unwrap().unwrap();
            self.memory[address as usize] = value;
        }

        self.state.as_mut().unwrap().registers.lcdc = self.memory[0xFF40];
        self.state.as_mut().unwrap().registers.stat = self.memory[0xFF41];
        self.state.as_mut().unwrap().registers.scy = self.memory[0xFF42];
        self.state.as_mut().unwrap().registers.scx = self.memory[0xFF43];
        self.state.as_mut().unwrap().registers.ly = self.memory[0xFF44];
        self.state.as_mut().unwrap().registers.lyc = self.memory[0xFF45];
        self.state.as_mut().unwrap().registers.dma = self.memory[0xFF46];
        self.state.as_mut().unwrap().registers.bgp = self.memory[0xFF47];
        self.state.as_mut().unwrap().registers.obp0 = self.memory[0xFF48];
        self.state.as_mut().unwrap().registers.obp1 = self.memory[0xFF49];
        self.state.as_mut().unwrap().registers.wy = self.memory[0xFF4A];
        self.state.as_mut().unwrap().registers.wx = self.memory[0xFF4B];
    }

    fn get_state(&self) -> Option<AbstractEmulatorState> {
        self.state
    }

    fn write_memory(&self, w: &mut dyn Write) -> Result<()> {
        w.write_all(&self.memory)?;
        Ok(())
    }
}

#[test]
fn emulator_replayer() {
    let bytes = [
        0x0u8, 0x1, 0x0, 0x2, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    let er = EmulatorReplayer::new(&bytes[..]);
    assert_eq!(
        er.get_state(),
        Some(AbstractEmulatorState {
            registers: AbstractEmulatorRegisters {
                pc: 0x1,
                sp: 0x2,
                a: 0x1,
                b: 0x2,
                c: 0x3,
                d: 0x4,
                e: 0x5,
                h: 0x6,
                l: 0x7,
                flags: 0x8,
                ..Default::default()
            },
            hash: 0x1 << 24 | 0x2 << 16 | 0x3 << 8 | 0x4,
            clock: 0
        })
    );
}

fn print_hex<F: FnMut(usize) -> bool>(
    out: &mut impl Write,
    memory: &[u8],
    start: usize,
    end: usize,
    mut color: F,
) -> io::Result<()> {
    for line in 0..((end - start) / 16) {
        let line_start = start + line * 16;
        let line_range = line_start..(line_start + 16);
        write!(out, "{line_start:04x}:")?;
        for addr in line_range.clone() {
            if addr & 0xF == 0x8 {
                write!(out, " ")?;
            }
            write!(out, " ")?;
            if color(addr) {
                write!(out, "\u{001b}[31m")?;
            }
            write!(out, "{:02x}", memory[addr])?;
            if color(addr) {
                write!(out, "\u{001b}[0m")?;
            }
        }
        writeln!(out)?;
    }
    Ok(())
}

fn print_memory_diff(out: &mut impl Write, a: &[u8], b: &[u8]) -> io::Result<()> {
    // Find the address where the differences start and end
    let iter = a.iter().zip(b.iter()).enumerate();
    let start = iter
        .clone()
        .find_map(|(i, (a, b))| if a != b { Some(i) } else { None })
        .unwrap_or(0);
    let end = iter
        .rev()
        .find_map(|(i, (a, b))| if a != b { Some(i) } else { None })
        .unwrap_or(0);

    // Extend the start and end to be aligned to 16 bytes
    let start = start - (start % 16);
    let end = end + if end % 16 == 0 { 0 } else { 16 - end % 16 };

    print_hex(out, a, start, end, |addr| a[addr] != b[addr])?;

    writeln!(
        out,
        "======================================================"
    )?;

    print_hex(out, b, start, end, |addr| a[addr] != b[addr])?;

    Ok(())
}

pub fn run<Storage: PersistentStorage>(
    out: &mut impl Write,
    mut storage: Storage,
    replay_file_path: &str,
    game_pak: GamePak<Storage>,
    pc_only: bool,
) -> Result<()> {
    let f = storage.open(OpenMode::Read, replay_file_path)?;
    let mut e1 = EmulatorReplayer::new(f);

    let mut ops = GameBoyOps::new(NullRenderer, NullSoundStream, storage);
    ops.load_game_pak(game_pak);

    let mut e2 = TandemGameBoyEmulator::new();

    let (a, b, runs) = compare_emulators(&mut e1, &mut e2, pc_only);

    writeln!(out, "differed after {runs} runs")?;
    writeln!(out, "Replay from path {:?}:", &replay_file_path)?;
    writeln!(out, "{a:#?}")?;
    writeln!(out, "Comeboy:")?;
    writeln!(out, "{b:#?}")?;

    writeln!(out)?;

    let mut buffer = vec![];
    let memory_map = ops.memory_map(&e2.emulator.bridge);
    let mut dis = Disassembler::new(&memory_map, RGBDSInstructionPrinterFactory, &mut buffer);
    dis.index = e2.emulator.cpu.read_program_counter();
    dis.disassemble_multiple().unwrap();
    writeln!(out, "{}", str::from_utf8(&buffer).unwrap())?;

    writeln!(out)?;

    let mut memory_a = vec![];
    e1.write_memory(&mut memory_a).unwrap();

    let mut memory_b = vec![];
    e2.write_memory(&mut memory_b).unwrap();

    print_memory_diff(out, &memory_a[..], &memory_b[..])?;

    Ok(())
}
