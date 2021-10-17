// Copyright 2018 Remi Bernotavicius

use crate::emulator_common::disassembler::Disassembler;
use crate::emulator_common::Intel8080Register;
use crate::game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;
use crate::game_boy_emulator::memory_controller::GameBoyMemoryMap;
use crate::game_boy_emulator::{GameBoyEmulator, GamePak, LR35902Flag, Result};
use crate::rendering::NullRenderer;
use crate::sound::NullSoundStream;
use core::fmt::{self, Debug};
use core::str;
use std::fs::File;
use std::io::{Bytes, Read, Write};
use std::path::Path;

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
    write!(f, "0x{:02x}: {:?}", flags, set)
}

impl Debug for AbstractEmulatorRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let le = if f.alternate() { "\n" } else { "" };
        let sep = if f.alternate() { "    " } else { " " };
        write!(f, "AbstractEmulatorRegisters {{{}", le)?;
        write!(f, "{}pc: 0x{:x},{}", sep, self.pc, le)?;
        write!(f, "{}sp: 0x{:x},{}", sep, self.sp, le)?;
        write!(f, "{}a: 0x{:x},{}", sep, self.a, le)?;
        write!(f, "{}b: 0x{:x},{}", sep, self.b, le)?;
        write!(f, "{}c: 0x{:x},{}", sep, self.c, le)?;
        write!(f, "{}d: 0x{:x},{}", sep, self.d, le)?;
        write!(f, "{}e: 0x{:x},{}", sep, self.e, le)?;
        write!(f, "{}h: 0x{:x},{}", sep, self.h, le)?;
        write!(f, "{}l: 0x{:x},{}", sep, self.l, le)?;
        write!(f, "{}flags: ", sep)?;
        fmt_flags(self.flags, f)?;
        write!(f, "{}", le)?;

        write!(f, "{}lcdc: 0x{:x},{}", sep, self.lcdc, le)?;
        write!(f, "{}stat: 0x{:x},{}", sep, self.stat, le)?;

        write!(f, "{}scy: 0x{:x},{}", sep, self.scy, le)?;
        write!(f, "{}scx: 0x{:x},{}", sep, self.scx, le)?;
        write!(f, "{}ly: 0x{:x},{}", sep, self.ly, le)?;
        write!(f, "{}lyc: 0x{:x},{}", sep, self.lyc, le)?;
        write!(f, "{}dma: 0x{:x},{}", sep, self.dma, le)?;
        write!(f, "{}bgp: 0x{:x},{}", sep, self.bgp, le)?;
        write!(f, "{}obp0: 0x{:x},{}", sep, self.obp0, le)?;
        write!(f, "{}obp1: 0x{:x},{}", sep, self.obp1, le)?;
        write!(f, "{}wx: 0x{:x},{}", sep, self.wx, le)?;
        write!(f, "{}wy: 0x{:x},{}", sep, self.wy, le)?;
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

    return (a_state, b_state, runs);
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
            states: states,
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

impl AbstractEmulator for GameBoyEmulator {
    fn run_one(&mut self) {
        self.tick(&mut NullRenderer, &mut NullSoundStream);
    }

    fn get_state(&self) -> Option<AbstractEmulatorState> {
        Some(AbstractEmulatorState {
            registers: AbstractEmulatorRegisters {
                pc: self.cpu.read_program_counter(),
                sp: self.cpu.read_register_pair(Intel8080Register::SP),
                a: self.cpu.read_register(Intel8080Register::A),
                b: self.cpu.read_register(Intel8080Register::B),
                c: self.cpu.read_register(Intel8080Register::C),
                d: self.cpu.read_register(Intel8080Register::D),
                e: self.cpu.read_register(Intel8080Register::E),
                h: self.cpu.read_register(Intel8080Register::H),
                l: self.cpu.read_register(Intel8080Register::L),
                flags: self.cpu.read_register(Intel8080Register::FLAGS),
                lcdc: self.lcd_controller.registers.lcdc.read_value(),
                stat: self.lcd_controller.registers.stat.read_value(),
                scy: self.lcd_controller.registers.scy.read_value(),
                scx: self.lcd_controller.registers.scx.read_value(),
                ly: self.lcd_controller.registers.ly.read_value(),
                lyc: self.lcd_controller.registers.lyc.read_value(),
                dma: self.lcd_controller.registers.dma.read_value(),
                bgp: self.lcd_controller.registers.bgp.read_value(),
                obp0: self.lcd_controller.registers.obp0.read_value(),
                obp1: self.lcd_controller.registers.obp1.read_value(),
                wy: self.lcd_controller.registers.wy.read_value(),
                wx: self.lcd_controller.registers.wx.read_value(),
            },
            clock: self.cpu.elapsed_cycles,
            hash: self.hash(),
        })
    }

    fn write_memory(&self, w: &mut dyn Write) -> Result<()> {
        self.write_memory(w)?;
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

        return er;
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
        w.write(&self.memory)?;
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

fn print_memory_diff(a: &[u8], b: &[u8]) {
    // Find the address where the differences start and end
    let iter = a.iter().zip(b.iter()).enumerate();
    let start = iter
        .clone()
        .find_map(|(i, (a, b))| if a != b { Some(i) } else { None })
        .unwrap_or(0);
    let end = iter
        .clone()
        .rev()
        .find_map(|(i, (a, b))| if a != b { Some(i) } else { None })
        .unwrap_or(0);

    // Extend the start and end to be aligned to 16 bytes
    let start = start - (start % 16);
    let end = end + if end % 16 == 0 { 0 } else { 16 - end % 16 };

    fn print_hex<F: FnMut(usize) -> bool>(memory: &[u8], start: usize, end: usize, mut color: F) {
        for line in 0..((end - start) / 16) {
            let line_start = start + line * 16;
            let line_range = line_start..(line_start + 16);
            print!("{:04x}:", line_start);
            for addr in line_range.clone() {
                if addr & 0xF == 0x8 {
                    print!(" ");
                }
                print!(" ");
                if color(addr) {
                    print!("\u{001b}[31m");
                }
                print!("{:02x}", memory[addr]);
                if color(addr) {
                    print!("\u{001b}[0m");
                }
            }
            println!();
        }
    }

    print_hex(a, start, end, |addr| a[addr] != b[addr]);

    println!("======================================================");

    print_hex(b, start, end, |addr| a[addr] != b[addr]);
}

pub fn run<P: AsRef<Path> + Debug>(
    replay_file_path: P,
    game_pak: GamePak,
    pc_only: bool,
) -> Result<()> {
    let f = File::open(&replay_file_path)?;
    let mut e1 = EmulatorReplayer::new(&f);

    let mut e2 = GameBoyEmulator::new();
    e2.load_game_pak(game_pak);

    let (a, b, runs) = compare_emulators(&mut e1, &mut e2, pc_only);

    println!("differed after {} runs", runs);
    println!("Replay from path {:?}:", &replay_file_path);
    println!("{:#?}", a);
    println!("Comeboy:");
    println!("{:#?}", b);

    println!();

    let mut buffer = vec![];
    let memory_map = game_boy_memory_map!(e2);
    let mut dis = Disassembler::new(&memory_map, RGBDSInstructionPrinterFactory, &mut buffer);
    dis.index = e2.cpu.read_program_counter();
    dis.disassemble_multiple().unwrap();
    println!("{}", str::from_utf8(&buffer).unwrap());

    println!();

    let mut memory_a = vec![];
    e1.write_memory(&mut memory_a).unwrap();

    let mut memory_b = vec![];
    e2.write_memory(&mut memory_b).unwrap();

    print_memory_diff(&memory_a[..], &memory_b[..]);
    Ok(())
}
