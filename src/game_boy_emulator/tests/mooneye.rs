// Copyright 2021 Remi Bernotavicius

use super::run_emulator_until_pc;
use crate::emulator_common::disassembler::MemoryAccessor;
use crate::game_boy_emulator::{memory_controller::GameBoyMemoryMap, GameBoyEmulator, GamePak};
use crate::lr35902_emulator::tests::{read_screen_message, read_test_rom};

fn assert_mooneye_test_rom_success<M: MemoryAccessor>(memory_accessor: &M) {
    let message = read_screen_message(memory_accessor);

    let message: String = message
        .chars()
        .filter(|&c| c != '\0' && c != '\n')
        .collect();
    assert_eq!(message, "Test OK");
}

pub(crate) fn read_mooneye_test_rom(name: &str) -> Vec<u8> {
    read_test_rom("mooneye_test_roms", name)
}

pub(crate) fn run_mooneye_test_rom(e: &mut GameBoyEmulator, stop_address: u16) {
    run_emulator_until_pc(e, stop_address);
    assert_mooneye_test_rom_success(&game_boy_memory_map!(e));
}

/// "Tests the DAA instruction with all possible input combinations"
#[test]
fn mooneye_test_rom_acceptance_instr_daa() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_mooneye_test_rom("acceptance/instr/daa.gb"),
        None,
    ));
    run_mooneye_test_rom(&mut e, 0x686e);
}
