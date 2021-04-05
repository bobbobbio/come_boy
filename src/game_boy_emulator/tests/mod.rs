// Copyright 2021 Remi Bernotavicius

use crate::emulator_common::disassembler::MemoryAccessor;
use crate::game_boy_emulator::{memory_controller::GameBoyMemoryMap, GameBoyEmulator};
use crate::rendering::NullRenderer;

pub(crate) mod blargg;
pub(crate) mod mooneye;
mod rom_tests;

fn run_emulator_until_pc(e: &mut GameBoyEmulator, stop_address: u16) {
    let mut pc = e.cpu.read_program_counter();
    // This address is where the rom ends.  At this address is an infinite loop where normally the
    // rom will sit at forever.
    while pc != stop_address {
        e.tick(&mut NullRenderer);
        pc = e.cpu.read_program_counter();
    }

    // If the LCD Controller is in mode 3, we can't access the message. Wait for the memory to be
    // available.
    while game_boy_memory_map!(e).read_memory(0x9800) == 0xFF {
        e.tick(&mut NullRenderer);
    }
}
