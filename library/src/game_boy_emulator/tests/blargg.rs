// Copyright 2021 Remi Bernotavicius

use super::run_emulator_until_pc;
use crate::game_boy_emulator::{GameBoyEmulator, GameBoyOps, GamePak};
use crate::lr35902_emulator::tests::blargg::{
    assert_blargg_test_rom_success, read_blargg_test_rom,
};

pub(crate) fn run_blargg_test_rom(rom_path: &str, stop_address: u16) {
    let mut ops = GameBoyOps::null();
    let mut e = GameBoyEmulator::new();
    let game_pak = GamePak::new(&read_blargg_test_rom(rom_path), &mut ops.storage, None).unwrap();
    ops.load_game_pak(game_pak);
    run_emulator_until_pc(&mut e, &mut ops, |pc| pc == stop_address);

    e.bridge
        .lcd_controller
        .background_display_data_1
        .release_all();
    assert_blargg_test_rom_success(&ops.memory_map(&e.bridge));
}

#[test]
fn blargg_test_rom_cpu_instrs_2_interrupts() {
    run_blargg_test_rom("cpu_instrs/individual/02-interrupts.gb", 0xc7f4);
}

#[test]
fn blargg_test_rom_instr_timing() {
    run_blargg_test_rom("instr_timing/instr_timing.gb", 0xc8b0);
}
