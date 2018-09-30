// Copyright 2017 Remi Bernotavicius

mod debugger;
mod disassembler;
mod lcd_controller;
mod memory_controller;
mod tandem;

use std::io::{self, Write};
use std::ops::Range;

use self::lcd_controller::{InterruptFlag, LCDController};
use self::memory_controller::{GameBoyMemoryMap, GameBoyRegister, MemoryChunk};
use emulator_common::disassembler::MemoryAccessor;
use lr35902_emulator::{Intel8080Register, LR35902Emulator, LR35902Flag};
use util::{super_fast_hash, Scheduler};

pub use self::debugger::run_debugger;
pub use self::disassembler::disassemble_game_boy_rom;

/*   ____                      ____              _____                 _       _
 *  / ___| __ _ _ __ ___   ___| __ )  ___  _   _| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * | |  _ / _` | '_ ` _ \ / _ \  _ \ / _ \| | | |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |_| | (_| | | | | | |  __/ |_) | (_) | |_| | |___| | | | | | |_| | | (_| | || (_) | |
 *  \____|\__,_|_| |_| |_|\___|____/ \___/ \__, |_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 *                                         |___/
 */

const LCDCSTATUS_INTERRUPT_ADDRESS: u16 = 0x0048;
const TIMER_INTERRUPT_ADDRESS: u16 = 0x0050;
const VERTICAL_BLANKING_INTERRUPT_ADDRESS: u16 = 0x0040;

const CHARACTER_DATA: Range<u16> = Range {
    start: 0x8000,
    end: 0x9800,
};
const CHARACTER_DATA_1: Range<u16> = Range {
    start: 0x0,
    end: 0x1000,
};
const CHARACTER_DATA_2: Range<u16> = Range {
    start: 0x800,
    end: 0x1800,
};
const BACKGROUND_DISPLAY_DATA_1: Range<u16> = Range {
    start: 0x9800,
    end: 0x9C00,
};
const BACKGROUND_DISPLAY_DATA_2: Range<u16> = Range {
    start: 0x9C00,
    end: 0xA000,
};
const OAM_DATA: Range<u16> = Range {
    start: 0xFE00,
    end: 0xFEA0,
};

// const CARTRIDGE_RAM: Range<u16> = Range { start: 0xA000, end: 0xC000 };
const UNUSABLE_MEMORY: Range<u16> = Range {
    start: 0xFEA0,
    end: 0xFF00,
};
const IO_PORTS_A: Range<u16> = Range {
    start: 0xFF10,
    end: 0xFF40,
};
const IO_PORTS_B: Range<u16> = Range {
    start: 0xFF4C,
    end: 0xFF80,
};
const HIGH_RAM: Range<u16> = Range {
    start: 0xFF80,
    end: 0xFFFF,
};
const INTERNAL_RAM_A: Range<u16> = Range {
    start: 0xC000,
    end: 0xDE00,
};
const INTERNAL_RAM_B: Range<u16> = Range {
    start: 0xDE00,
    end: 0xE000,
};
const ECHO_RAM: Range<u16> = Range {
    start: 0xE000,
    end: 0xFE00,
};

#[derive(Default)]
struct GameBoyRegisters {
    interrupt_flag: GameBoyRegister,
    interrupt_enable: GameBoyRegister,

    p1_joypad: GameBoyRegister,
    serial_transfer_data: GameBoyRegister,
    serial_transfer_control: GameBoyRegister,
    divider: GameBoyRegister,

    timer_counter: GameBoyRegister,
    timer_modulo: GameBoyRegister,
    timer_control: GameBoyRegister,
}

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator<GameBoyMemoryMap>,
    lcd_controller: LCDController<'a>,
    io_ports_a: MemoryChunk,
    io_ports_b: MemoryChunk,
    high_ram: MemoryChunk,
    internal_ram_a: MemoryChunk,
    internal_ram_b: MemoryChunk,

    registers: GameBoyRegisters,
    scheduler: Scheduler<GameBoyEmulator<'a>>,
}

impl<'a> GameBoyEmulator<'a> {
    fn new() -> GameBoyEmulator<'a> {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(GameBoyMemoryMap::new()),
            lcd_controller: LCDController::new(),
            io_ports_a: MemoryChunk::from_range(IO_PORTS_A),
            io_ports_b: MemoryChunk::from_range(IO_PORTS_B),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            internal_ram_a: MemoryChunk::from_range(INTERNAL_RAM_A),
            internal_ram_b: MemoryChunk::from_range(INTERNAL_RAM_B),
            registers: Default::default(),
            scheduler: Scheduler::new(),
        };

        // Restart and interrupt vectors (unmapped) 0x0000 - 0x00FF

        // Rom (unmapped) 0x0100 - 0x7FFF

        // Character data 0x8000 - 0x97FF
        e.cpu.memory_accessor.map_chunk(
            CHARACTER_DATA.start,
            e.lcd_controller.character_data.clone(),
        );

        // Background display data 0x9800 - 0x9FFF
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_1.start,
            e.lcd_controller.background_display_data_1.clone(),
        );
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_2.start,
            e.lcd_controller.background_display_data_2.clone(),
        );

        // Cartridge RAM (unmapped) 0xA000 - 0xBFFF

        // Internal RAM 0xC000 - 0xDFFF
        e.cpu
            .memory_accessor
            .map_chunk(INTERNAL_RAM_A.start, e.internal_ram_a.clone());
        e.cpu
            .memory_accessor
            .map_chunk(INTERNAL_RAM_B.start, e.internal_ram_b.clone());

        // Echo RAM 0xE000 - 0xFDFF
        e.cpu
            .memory_accessor
            .map_chunk(ECHO_RAM.start, e.internal_ram_a.clone());

        // OAM Data 0xFE00 - 0xFE9F
        e.cpu
            .memory_accessor
            .map_chunk(OAM_DATA.start, e.lcd_controller.oam_data.clone());

        // Unusable memory 0xFEA0 - 0xFEFF
        e.cpu.memory_accessor.map_chunk(
            UNUSABLE_MEMORY.start,
            e.lcd_controller.unusable_memory.clone(),
        );

        // Registers
        e.cpu
            .memory_accessor
            .map_chunk(0xFF00, e.registers.p1_joypad.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF01, e.registers.serial_transfer_data.chunk.clone());
        e.cpu.memory_accessor.map_chunk(
            0xFF02,
            e.registers.serial_transfer_control.chunk.clone_read_only(),
        );

        e.cpu
            .memory_accessor
            .map_chunk(0xFF04, e.registers.divider.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF05, e.registers.timer_counter.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF06, e.registers.timer_modulo.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF07, e.registers.timer_control.chunk.clone());

        e.cpu
            .memory_accessor
            .map_chunk(0xFF0F, e.registers.interrupt_flag.chunk.clone());

        // Other IO Registers 0xFF10 - 0xFF3F
        e.cpu
            .memory_accessor
            .map_chunk(IO_PORTS_A.start, e.io_ports_a.clone());

        // LCD Registers 0xFF40 - 0xFF4B
        e.cpu
            .memory_accessor
            .map_chunk(0xFF40, e.lcd_controller.registers.lcdc.chunk.clone());
        e.cpu.memory_accessor.map_chunk(
            0xFF41,
            e.lcd_controller.registers.stat.chunk.clone_read_only(),
        );
        e.cpu
            .memory_accessor
            .map_chunk(0xFF42, e.lcd_controller.registers.scy.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF43, e.lcd_controller.registers.scx.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF44, e.lcd_controller.registers.ly.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF45, e.lcd_controller.registers.lyc.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF46, e.lcd_controller.registers.dma.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF47, e.lcd_controller.registers.bgp.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF48, e.lcd_controller.registers.obp0.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF49, e.lcd_controller.registers.obp1.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF4A, e.lcd_controller.registers.wy.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF4B, e.lcd_controller.registers.wx.chunk.clone());

        // Other IO Registers 0xFF4C - 0xFF7F
        e.cpu
            .memory_accessor
            .map_chunk(IO_PORTS_B.start, e.io_ports_b.clone());

        // High RAM 0xFF80 - 0xFFFE
        e.cpu
            .memory_accessor
            .map_chunk(HIGH_RAM.start, e.high_ram.clone());

        // interrupt enable register
        e.cpu
            .memory_accessor
            .map_chunk(0xFFFF, e.registers.interrupt_enable.chunk.clone());

        e.lcd_controller.set_state_post_bios();

        e.set_state_post_bios();

        let elapsed_cycles = e.cpu.elapsed_cycles;
        e.scheduler
            .schedule(elapsed_cycles + 52, Self::divider_tick);
        e.scheduler
            .schedule(elapsed_cycles + 98584, Self::unknown_event);

        e.lcd_controller
            .scheduler
            .schedule(elapsed_cycles + 56 + 4, LCDController::mode_2);
        e.lcd_controller
            .scheduler
            .schedule(elapsed_cycles + 56 + 456, LCDController::advance_ly);
        e.lcd_controller
            .scheduler
            .schedule(elapsed_cycles + 98648, LCDController::unknown_event2);

        return e;
    }

    fn unknown_event(&mut self, _time: u64) {
        let value = self.registers.interrupt_flag.read_value();
        self.registers.interrupt_flag.set_value(value | 0xE0);
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.cpu
            .memory_accessor
            .map_chunk(0, MemoryChunk::new(rom.to_vec()));
    }

    fn crashed(&self) -> Option<&String> {
        if self.cpu.crashed() {
            return self.cpu.crash_message.as_ref();
        } else if self.lcd_controller.crashed() {
            return self.lcd_controller.crash_message.as_ref();
        }

        None
    }

    fn divider_tick(&mut self, time: u64) {
        self.registers.divider.add(1);
        self.scheduler.schedule(time + 256, Self::divider_tick);
    }

    fn tick(&mut self) {
        self.cpu.run_one_instruction();

        let current_clock = self.cpu.elapsed_cycles;

        for (time, event) in self.scheduler.poll(current_clock) {
            event(self, time);
        }

        for (time, event) in self.lcd_controller.scheduler.poll(current_clock) {
            event(&mut self.lcd_controller, time);
        }

        if self.cpu.get_interrupts_enabled() {
            self.handle_interrupts();
        }
    }

    fn handle_interrupts(&mut self) {
        let interrupt_flag_value = self.registers.interrupt_flag.read_value();
        let interrupt_enable_value = self.registers.interrupt_enable.read_value();

        if interrupt_flag_value & InterruptFlag::VerticalBlanking as u8 != 0
            && interrupt_enable_value & InterruptFlag::VerticalBlanking as u8 != 0
        {
            self.registers
                .interrupt_flag
                .set_value(interrupt_flag_value & !(InterruptFlag::VerticalBlanking as u8));
            self.cpu.interrupt(VERTICAL_BLANKING_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::LCDSTAT as u8 != 0
            && interrupt_enable_value & InterruptFlag::LCDSTAT as u8 != 0
        {
            self.cpu.interrupt(LCDCSTATUS_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::Timer as u8 != 0
            && interrupt_enable_value & InterruptFlag::Timer as u8 != 0
        {
            self.cpu.interrupt(TIMER_INTERRUPT_ADDRESS);
        }
    }

    fn set_state_post_bios(&mut self) {
        /*
         * After running the BIOS (the part of the gameboy that shows the logo) the cpu is left in
         * a very certain state. Since this is always the case, certain games may rely on this fact
         * (and indeed often times do.)
         */
        self.cpu.set_register(Intel8080Register::A, 0x1);
        self.cpu.set_register(Intel8080Register::B, 0x0);
        self.cpu.set_register(Intel8080Register::C, 0x13);
        self.cpu.set_register(Intel8080Register::D, 0x0);
        self.cpu.set_register(Intel8080Register::E, 0xD8);
        self.cpu.set_register(Intel8080Register::H, 0x01);
        self.cpu.set_register(Intel8080Register::L, 0x4D);
        self.cpu.set_flag(LR35902Flag::Carry, true);
        self.cpu.set_flag(LR35902Flag::HalfCarry, true);
        self.cpu.set_flag(LR35902Flag::Subtract, false);
        self.cpu.set_flag(LR35902Flag::Zero, true);

        self.registers.p1_joypad.set_value(0xcf);
        self.registers.serial_transfer_data.set_value(0x0);
        self.registers.serial_transfer_control.set_value(0x7e);

        self.registers.divider.set_value(0xab);
        self.registers.timer_counter.set_value(0x0);
        self.registers.timer_modulo.set_value(0x0);
        self.registers.timer_control.set_value(0xf8);

        self.registers.interrupt_flag.set_value(0xe1);

        // Initialize io ports
        let io_ports_a = include_bytes!("assets/io_ports_a.bin");
        self.io_ports_a.clone_from_slice(&io_ports_a[..]);

        /* 40 - 4B LCD Controller */

        let io_ports_b = include_bytes!("assets/io_ports_b.bin");
        self.io_ports_b.clone_from_slice(&io_ports_b[..]);

        let high_ram = include_bytes!("assets/high_ram.bin");
        self.high_ram.clone_from_slice(&high_ram[..]);

        let internal_ram = include_bytes!("assets/internal_ram.bin");

        let split = (INTERNAL_RAM_A.end - INTERNAL_RAM_A.start) as usize;
        self.internal_ram_a
            .clone_from_slice(&internal_ram[0..split]);
        self.internal_ram_b.clone_from_slice(&internal_ram[split..]);

        self.registers.interrupt_enable.set_value(0x0);
    }

    fn run(&mut self) {
        self.lcd_controller.start_rendering();
        while self.crashed().is_none() {
            self.tick();
        }
        if self.cpu.crashed() {
            println!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
    }

    fn write_memory(&self, w: &mut Write) -> io::Result<()> {
        let mut mem = [0u8; 0x10000];
        for i in 0..0x10000 {
            mem[i] = self.cpu.memory_accessor.read_memory(i as u16);
        }

        w.write(&mem)?;
        Ok(())
    }

    fn hash(&self) -> u32 {
        let mut mem = [0u8; 0x10000 + 15];
        for i in 0..0x10000 {
            mem[i] = self.cpu.memory_accessor.read_memory(i as u16);
        }

        mem[0x10000 + 0] = self.cpu.read_register(Intel8080Register::A);
        mem[0x10000 + 1] = self.cpu.read_register(Intel8080Register::B);
        mem[0x10000 + 2] = self.cpu.read_register(Intel8080Register::C);
        mem[0x10000 + 3] = self.cpu.read_register(Intel8080Register::D);
        mem[0x10000 + 4] = self.cpu.read_register(Intel8080Register::E);
        mem[0x10000 + 5] = self.cpu.read_register(Intel8080Register::H);
        mem[0x10000 + 6] = self.cpu.read_register(Intel8080Register::L);
        mem[0x10000 + 7] = (self.cpu.read_program_counter() >> 8) as u8;
        mem[0x10000 + 8] = (self.cpu.read_program_counter() & 0xFF) as u8;
        mem[0x10000 + 9] = (self.cpu.read_register_pair(Intel8080Register::SP) >> 8) as u8;
        mem[0x10000 + 10] = (self.cpu.read_register_pair(Intel8080Register::SP) & 0xFF) as u8;
        mem[0x10000 + 11] = if self.cpu.read_flag(LR35902Flag::Zero) {
            1
        } else {
            0
        };
        mem[0x10000 + 12] = if self.cpu.read_flag(LR35902Flag::Subtract) {
            1
        } else {
            0
        };
        mem[0x10000 + 13] = if self.cpu.read_flag(LR35902Flag::HalfCarry) {
            1
        } else {
            0
        };
        mem[0x10000 + 14] = if self.cpu.read_flag(LR35902Flag::Carry) {
            1
        } else {
            0
        };
        return super_fast_hash(&mem);
    }
}

#[test]
fn initial_state_test() {
    let e = GameBoyEmulator::new();

    // Lock down the initial state.
    assert_eq!(e.hash(), 1497694477);
}

/*  ____  _                         _____         _     ____   ___  __  __
 * | __ )| | __ _ _ __ __ _  __ _  |_   _|__  ___| |_  |  _ \ / _ \|  \/  |___
 * |  _ \| |/ _` | '__/ _` |/ _` |   | |/ _ \/ __| __| | |_) | | | | |\/| / __|
 * | |_) | | (_| | | | (_| | (_| |   | |  __/\__ \ |_  |  _ <| |_| | |  | \__ \
 * |____/|_|\__,_|_|  \__, |\__, |   |_|\___||___/\__| |_| \_\\___/|_|  |_|___/
 *                    |___/ |___/
 *
 */

#[cfg(test)]
use lr35902_emulator::{read_blargg_test_rom, run_blargg_test_rom};

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_2_interrupts() {
    let mut e = GameBoyEmulator::new();
    e.load_rom(&read_blargg_test_rom(
        "cpu_instrs/individual/02-interrupts.gb",
    ));
    run_blargg_test_rom(&mut e.cpu, 0xc7f4);
}

pub fn run_emulator(rom: &[u8]) {
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    e.run();
}

pub fn run_in_tandem_with(other_emulator_path: &str, rom: &[u8]) {
    println!("loading {}", other_emulator_path);

    tandem::run(other_emulator_path, rom);
}
