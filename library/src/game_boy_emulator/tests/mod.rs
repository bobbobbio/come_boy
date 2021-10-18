// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::{GameBoyEmulator, NullGameBoyOps};

pub(crate) mod blargg;
pub(crate) mod mooneye;
mod rom_tests;

fn run_emulator_until_pc<F: Fn(u16) -> bool>(
    e: &mut GameBoyEmulator,
    ops: &mut NullGameBoyOps,
    stop_when: F,
) {
    let mut pc = e.cpu.read_program_counter();
    // This address is where the rom ends.  At this address is an infinite loop where normally the
    // rom will sit at forever.
    while !stop_when(pc) {
        e.tick(ops);
        pc = e.cpu.read_program_counter();
    }
}
