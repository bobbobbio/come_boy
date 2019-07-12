// Copyright 2017 Remi Bernotavicius

#[macro_use]
mod memory_controller;

mod debugger;
mod disassembler;
mod game_pak;
mod joypad_register;
mod lcd_controller;
mod sound_controller;
mod tandem;

use std::io::{self, Write};
use std::ops::Range;

use self::game_pak::GamePak;
use self::joypad_register::JoyPadRegister;
use self::lcd_controller::{InterruptFlag, LCDController};
use self::memory_controller::{GameBoyFlags, GameBoyRegister, MemoryChunk};
use self::sound_controller::SoundController;
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
/*
const ECHO_RAM: Range<u16> = Range {
    start: 0xE000,
    end: 0xFE00,
};
*/

#[derive(Default)]
struct GameBoyRegisters {
    interrupt_flag: GameBoyFlags<InterruptFlag>,
    interrupt_enable: GameBoyFlags<InterruptFlag>,

    serial_transfer_data: GameBoyRegister,
    serial_transfer_control: GameBoyRegister,
    divider: GameBoyRegister,
}

#[derive(Default)]
struct GameBoyTimer {
    counter: GameBoyRegister,
    modulo: GameBoyRegister,
    control: GameBoyFlags<TimerFlags>,
    scheduler: Scheduler<GameBoyTimer>,
    interrupt_requested: bool,
}

enum TimerFlags {
    Enabled = 0b00000100,
    Speed = 0b00000011,
}

from_u8!(TimerFlags);

impl GameBoyTimer {
    fn enabled(&self) -> bool {
        self.control.read_flag(TimerFlags::Enabled)
    }

    fn timer_speed(&self) -> u64 {
        let speed = match self.control.read_flag_value(TimerFlags::Speed) {
            0b00 => 4096,
            0b01 => 262144,
            0b10 => 65536,
            0b11 => 16384,
            _ => panic!(),
        };

        // 4Mhz = 4M clock ticks/s / speed ticks/s
        4194304 / speed
    }

    fn set_state_post_bios(&mut self) {
        self.counter.set_value(0x0);
        self.modulo.set_value(0x0);
        self.control.set_value(0xf8);
    }

    fn schedule_initial_events(&mut self, now: u64) {
        let speed = self.timer_speed();
        self.scheduler.schedule(now + speed, Self::tick);
    }

    fn tick(&mut self, now: u64) {
        if self.enabled() {
            let counter = self.counter.read_value().wrapping_add(1);
            if counter == 0 {
                self.interrupt_requested = true;
                let modulo_value = self.modulo.read_value();
                self.counter.set_value(modulo_value);
            } else {
                self.counter.set_value(counter);
            }
        }
        let speed = self.timer_speed();
        self.scheduler.schedule(now + speed, Self::tick);
    }

    fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.interrupt_requested {
            interrupt_flag.set_flag(InterruptFlag::Timer, true);
            self.interrupt_requested = false;
        }
    }

    fn deliver_events(&mut self, now: u64) {
        for (time, event) in self.scheduler.poll(now) {
            event(self, time);
        }
    }
}

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator,
    sound_controller: SoundController,
    lcd_controller: LCDController<'a>,
    high_ram: MemoryChunk,
    internal_ram_a: MemoryChunk,
    internal_ram_b: MemoryChunk,

    registers: GameBoyRegisters,
    scheduler: Scheduler<GameBoyEmulator<'a>>,
    timer: GameBoyTimer,
    game_pak: Option<GamePak>,
    joypad_register: JoyPadRegister,
}

impl<'a> GameBoyEmulator<'a> {
    fn new(pixel_scale: u32) -> GameBoyEmulator<'a> {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(),
            lcd_controller: LCDController::new(pixel_scale),
            sound_controller: Default::default(),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            internal_ram_a: MemoryChunk::from_range(INTERNAL_RAM_A),
            internal_ram_b: MemoryChunk::from_range(INTERNAL_RAM_B),
            registers: Default::default(),
            scheduler: Scheduler::new(),
            timer: Default::default(),
            game_pak: None,
            joypad_register: JoyPadRegister::new(),
        };
        e.set_state_post_bios();
        e.schedule_initial_events();
        e
    }

    fn unknown_event(&mut self, _time: u64) {
        let value = self.registers.interrupt_flag.read_value();
        self.registers.interrupt_flag.set_value(value | 0xE0);
    }

    fn load_game_pak(&mut self, game_pak: GamePak) {
        println!("Loading {:?}", &game_pak);
        self.game_pak = Some(game_pak);
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

    fn deliver_events(&mut self, now: u64) {
        for (time, event) in self.scheduler.poll(now) {
            event(self, time);
        }

        self.lcd_controller.deliver_events(now);
        self.timer.deliver_events(now);
    }

    fn tick(&mut self) {
        self.cpu.load_instruction(&mut build_memory_map_mut!(self));

        let now = self.cpu.elapsed_cycles;
        self.deliver_events(now);

        self.cpu
            .execute_instruction(&mut build_memory_map_mut!(self));

        if let Some(game_pak) = &mut self.game_pak {
            game_pak.tick();
        }

        self.lcd_controller.tick(now);
        self.execute_dma();

        let now = self.cpu.elapsed_cycles;
        self.deliver_events(now);

        self.lcd_controller
            .schedule_interrupts(&mut self.registers.interrupt_flag);
        self.timer
            .schedule_interrupts(&mut self.registers.interrupt_flag);

        self.handle_interrupts();
    }

    fn drive_joypad(&mut self, time: u64) {
        let key_events = self.lcd_controller.poll_renderer();
        self.joypad_register.tick(key_events);
        self.scheduler.schedule(time + 456, Self::drive_joypad);
    }

    fn execute_dma(&mut self) {
        // XXX This is suppose to take about 40 cycles to complete.
        if let Some(address) = self.lcd_controller.registers.dma.take_request() {
            let mut memory_map = build_memory_map_mut!(self);
            for (dst_address, src_address) in OAM_DATA.zip(address..) {
                let value = memory_map.read_memory(src_address);
                memory_map.set_memory(dst_address, value);
            }
        }
    }

    fn deliver_interrupt(&mut self, flag: InterruptFlag, address: u16) {
        let interrupt_flag_value = self.registers.interrupt_flag.read_flag(flag);
        let interrupt_enable_value = self.registers.interrupt_enable.read_flag(flag);

        if interrupt_flag_value && interrupt_enable_value {
            self.cpu.resume();

            if self.cpu.get_interrupts_enabled() {
                self.registers.interrupt_flag.set_flag(flag, false);
                self.cpu
                    .interrupt(&mut build_memory_map_mut!(self), address);
            }
        }
    }

    fn handle_interrupts(&mut self) {
        self.deliver_interrupt(
            InterruptFlag::VerticalBlanking,
            VERTICAL_BLANKING_INTERRUPT_ADDRESS,
        );
        self.deliver_interrupt(InterruptFlag::LCDSTAT, LCDCSTATUS_INTERRUPT_ADDRESS);
        self.deliver_interrupt(InterruptFlag::Timer, TIMER_INTERRUPT_ADDRESS);
    }

    fn set_state_post_bios(&mut self) {
        self.sound_controller.set_state_post_bios();
        self.lcd_controller.set_state_post_bios();

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

        self.registers.serial_transfer_data.set_value(0x0);
        self.registers.serial_transfer_control.set_value(0x7e);

        self.registers.divider.set_value(0xab);
        self.timer.set_state_post_bios();

        self.registers.interrupt_flag.set_value(0xe1);

        /* 10 - 26 Sound Controller */

        /* 40 - 4B LCD Controller */

        let high_ram = include_bytes!("assets/high_ram.bin");
        self.high_ram.clone_from_slice(&high_ram[..]);

        let internal_ram = include_bytes!("assets/internal_ram.bin");

        let split = (INTERNAL_RAM_A.end - INTERNAL_RAM_A.start) as usize;
        self.internal_ram_a
            .clone_from_slice(&internal_ram[0..split]);
        self.internal_ram_b.clone_from_slice(&internal_ram[split..]);

        self.registers.interrupt_enable.set_value(0x0);
    }

    fn schedule_initial_events(&mut self) {
        let now = self.cpu.elapsed_cycles;
        self.scheduler.schedule(now + 52, Self::divider_tick);
        self.scheduler.schedule(now + 98584, Self::unknown_event);
        self.scheduler.schedule(now + 456, Self::drive_joypad);

        self.lcd_controller.schedule_initial_events(now);
        self.timer.schedule_initial_events(now);
    }

    fn run(&mut self) {
        self.lcd_controller.start_rendering();

        let mut last_cycles = self.cpu.elapsed_cycles;
        let mut last_instant = std::time::Instant::now();

        while self.crashed().is_none() {
            self.tick();

            let elapsed_cycles = self.cpu.elapsed_cycles - last_cycles;

            // We can't sleep every tick, so just so it every so often.
            if elapsed_cycles > 3648 {
                // 4.19 Mhz means each cycles takes roughly 238 nanoseconds;
                let expected_time = std::time::Duration::from_nanos(elapsed_cycles * 238);

                // If we didn't take long enough, sleep the difference.
                if let Some(sleep_time) = expected_time.checked_sub(last_instant.elapsed()) {
                    std::thread::sleep(sleep_time);
                }

                last_cycles = self.cpu.elapsed_cycles;
                last_instant = std::time::Instant::now();
            }
        }

        if self.cpu.crashed() {
            println!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
    }

    fn write_memory(&self, w: &mut Write) -> io::Result<()> {
        let memory_map = build_memory_map!(self);
        let mut mem = [0u8; 0x10000];
        for i in 0..0x10000 {
            mem[i] = memory_map.read_memory(i as u16);
        }

        w.write(&mem)?;
        Ok(())
    }

    fn hash(&self) -> u32 {
        let memory_map = build_memory_map!(self);
        let mut mem = [0u8; 0x10000 + 15];
        for i in 0..0x10000 {
            mem[i] = memory_map.read_memory(i as u16);
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
    let e = GameBoyEmulator::new(4);

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
use lr35902_emulator::{assert_blargg_test_rom_success, read_blargg_test_rom};

#[cfg(test)]
fn run_blargg_test_rom(e: &mut GameBoyEmulator, stop_address: u16) {
    let mut pc = e.cpu.read_program_counter();
    // This address is where the rom ends.  At this address is an infinite loop where normally the
    // rom will sit at forever.
    while pc != stop_address {
        e.tick();
        pc = e.cpu.read_program_counter();
    }

    let memory_map = build_memory_map!(e);
    assert_blargg_test_rom_success(&memory_map);
}

#[test]
fn blargg_test_rom_cpu_instrs_2_interrupts() {
    let mut e = GameBoyEmulator::new(4);
    e.load_game_pak(GamePak::from(&read_blargg_test_rom(
        "cpu_instrs/individual/02-interrupts.gb",
    )));
    run_blargg_test_rom(&mut e, 0xc7f4);
}

pub fn run_emulator(rom: &[u8], pixel_scale: u32) {
    let mut e = GameBoyEmulator::new(pixel_scale);
    e.load_game_pak(GamePak::from(rom));
    e.run();
}

pub fn run_in_tandem_with(other_emulator_path: &str, rom: &[u8]) {
    println!("loading {}", other_emulator_path);

    tandem::run(other_emulator_path, rom);
}
