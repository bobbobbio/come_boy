// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::tests::blargg::run_blargg_test_rom;
use crate::game_boy_emulator::{GameBoyEmulator, GamePak};
use crate::lr35902_emulator::tests::blargg::read_blargg_test_rom;

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_1_lcd_sync() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/1-lcd_sync.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc88b);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_2_causes() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/2-causes.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc9d3);
}

#[test]
fn blargg_test_rom_oam_bug_3_non_causes() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/3-non_causes.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc9bb);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_4_scanline_timing() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/4-scanline_timing.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc933);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_5_timing_bug() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/5-timing_bug.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc920);
}

#[test]
fn blargg_test_rom_oam_bug_6_timing_no_bug() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/6-timing_no_bug.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc85d);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_7_timing_effect() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/7-timing_effect.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc84b);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_8_instr_effect() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_blargg_test_rom("oam_bug/rom_singles/8-instr_effect.gb"),
        None,
    ));
    run_blargg_test_rom(&mut e, 0xc922);
}
