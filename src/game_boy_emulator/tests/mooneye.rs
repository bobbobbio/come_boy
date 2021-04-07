// Copyright 2021 Remi Bernotavicius

use super::run_emulator_until_pc;
use crate::game_boy_emulator::{memory_controller::GameBoyMemoryMap, GameBoyEmulator, GamePak};
use crate::lr35902_emulator::tests::{read_screen_message, read_test_rom};
use std::{fs, path::PathBuf};

fn read_mooneye_test_rom(name: &str) -> Vec<u8> {
    read_test_rom("mooneye_test_roms", name)
}

// XXX: Should replace with fully fleshed symbol file parsing eventually.
fn get_mooneye_address(rom_path: &str, symbol: &str) -> u16 {
    let rom_path = PathBuf::from(format!("mooneye_test_roms/{}", rom_path));
    let stem = rom_path.file_stem().unwrap().to_str().unwrap().to_owned();
    let sym_path = rom_path.with_file_name(stem + ".sym");

    println!("sym-path {:?}", sym_path);
    let contents = fs::read_to_string(sym_path).unwrap();
    let sym_line = contents
        .lines()
        .filter(|l| l.contains(symbol))
        .next()
        .unwrap();

    let location = sym_line.split(" ").next().unwrap();
    let address = location.split(":").skip(1).next().unwrap();
    u16::from_str_radix(address, 16).unwrap()
}

pub(crate) fn run_mooneye_test_rom(rom_path: &str) {
    let success_address = get_mooneye_address(rom_path, "quit@success");
    let fail_address = get_mooneye_address(rom_path, "quit@failure");

    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(&read_mooneye_test_rom(rom_path), None));
    run_emulator_until_pc(&mut e, |pc| pc == success_address || pc == fail_address);

    if e.cpu.read_program_counter() == fail_address {
        e.lcd_controller.background_display_data_1.release_all();
        panic!("{}", read_screen_message(&game_boy_memory_map!(&e)));
    } else {
        assert_eq!(e.cpu.read_program_counter(), success_address);
    }
}

/// "Tests the DAA instruction with all possible input combinations"
#[test]
fn mooneye_test_rom_acceptance_instr_daa() {
    run_mooneye_test_rom("acceptance/instr/daa.gb");
}

/// "This test checks that bottom 4 bits of the F register always return 0"
#[test]
fn mooneye_test_rom_acceptance_bits_reg_f() {
    run_mooneye_test_rom("acceptance/bits/reg_f.gb");
}

/// "This test checks all unused bits in working $FFxx IO, and all unused $FFxx IO. Unused bits and
/// unused IO all return 1s"
#[test]
fn mooneye_test_rom_acceptance_bits_unused_hwio_gs() {
    run_mooneye_test_rom("acceptance/bits/unused_hwio-GS.gb");
}

/// "Tests what happens if the IE register is the target for one of the PC pushes during interrupt
/// dispatch."
#[test]
fn mooneye_test_rom_acceptance_interrupts_ie_push() {
    run_mooneye_test_rom("acceptance/interrupts/ie_push.gb");
}
