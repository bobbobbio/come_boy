// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::tests::blargg::run_blargg_test_rom;

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_1_lcd_sync() {
    run_blargg_test_rom("oam_bug/rom_singles/1-lcd_sync.gb", 0xc88b);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_2_causes() {
    run_blargg_test_rom("oam_bug/rom_singles/2-causes.gb", 0xc9d3);
}

#[test]
fn blargg_test_rom_oam_bug_3_non_causes() {
    run_blargg_test_rom("oam_bug/rom_singles/3-non_causes.gb", 0xc9bb);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_4_scanline_timing() {
    run_blargg_test_rom("oam_bug/rom_singles/4-scanline_timing.gb", 0xc933);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_5_timing_bug() {
    run_blargg_test_rom("oam_bug/rom_singles/5-timing_bug.gb", 0xc920);
}

#[test]
fn blargg_test_rom_oam_bug_6_timing_no_bug() {
    run_blargg_test_rom("oam_bug/rom_singles/6-timing_no_bug.gb", 0xc85d);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_7_timing_effect() {
    run_blargg_test_rom("oam_bug/rom_singles/7-timing_effect.gb", 0xc84b);
}

#[test]
#[ignore]
fn blargg_test_rom_oam_bug_8_instr_effect() {
    run_blargg_test_rom("oam_bug/rom_singles/8-instr_effect.gb", 0xc922);
}
