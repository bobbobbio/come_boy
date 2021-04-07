// Copyright 2021 Remi Bernotavicius

use super::run_emulator_until_pc;
use crate::emulator_common::disassembler::MemoryAccessor;
use crate::game_boy_emulator::{memory_controller::GameBoyMemoryMap, GameBoyEmulator, GamePak};
use crate::lr35902_emulator::tests::{read_screen_message, read_test_rom};

fn assert_mooneye_test_rom_success<M: MemoryAccessor>(memory_accessor: &M) {
    let message = read_screen_message(memory_accessor);

    let message: String = message.chars().filter(|&c| c != '\0').collect();
    let lines: Vec<_> = message
        .split('\n')
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .collect();

    if let Some(assertions_index) = lines.iter().position(|&l| l == "Assertions") {
        let assertions = &lines[(assertions_index + 1)..];
        assert!(assertions.iter().all(|l| l.contains("OK")), "{}", message);
    } else {
        assert_eq!(lines[0], "Test OK", "{}", message);
    }
}

fn read_mooneye_test_rom(name: &str) -> Vec<u8> {
    read_test_rom("mooneye_test_roms", name)
}

pub(crate) fn run_mooneye_test_rom(rom_path: &str, stop_address: u16) {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(&read_mooneye_test_rom(rom_path), None));
    run_emulator_until_pc(&mut e, stop_address);
    assert_mooneye_test_rom_success(&game_boy_memory_map!(&e));
}

/// "Tests the DAA instruction with all possible input combinations"
#[test]
fn mooneye_test_rom_acceptance_instr_daa() {
    run_mooneye_test_rom("acceptance/instr/daa.gb", 0x686e);
}

/// "This test checks that bottom 4 bits of the F register always return 0"
#[test]
fn mooneye_test_rom_acceptance_bits_reg_f() {
    run_mooneye_test_rom("acceptance/bits/reg_f.gb", 0x4b2e);
}

/// "This test checks all unused bits in working $FFxx IO, and all unused $FFxx IO. Unused bits and
/// unused IO all return 1s"
#[test]
fn mooneye_test_rom_acceptance_bits_unused_hwio_gs() {
    run_mooneye_test_rom("acceptance/bits/unused_hwio-GS.gb", 0x486e);
}

/// "Tests what happens if the IE register is the target for one of the PC pushes during interrupt
/// dispatch."
#[test]
fn mooneye_test_rom_acceptance_interrupts_ie_push() {
    run_mooneye_test_rom("acceptance/interrupts/ie_push.gb", 0x486e);
}
