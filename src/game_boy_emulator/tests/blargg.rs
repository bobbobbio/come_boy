// Copyright 2021 Remi Bernotavicius

use super::run_emulator_until_pc;
use crate::game_boy_emulator::{memory_controller::GameBoyMemoryMap, GameBoyEmulator, GamePak};
use crate::lr35902_emulator::tests::blargg::{
    assert_blargg_test_rom_success, read_blargg_test_rom,
};

pub(crate) fn run_blargg_test_rom(e: &mut GameBoyEmulator, stop_address: u16) {
    run_emulator_until_pc(e, stop_address);
    assert_blargg_test_rom_success(&game_boy_memory_map!(e));
}

#[test]
fn blargg_test_rom_cpu_instrs_2_interrupts() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("cpu_instrs/individual/02-interrupts.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc7f4);
}

#[test]
fn blargg_test_rom_instr_timing() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("instr_timing/instr_timing.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc8b0);
}
