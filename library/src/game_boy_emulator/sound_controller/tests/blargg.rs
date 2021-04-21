// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::tests::blargg::run_blargg_test_rom;

#[test]
#[ignore]
fn blargg_test_rom_dmg_sound_04_sweep() {
    run_blargg_test_rom("dmg_sound/rom_singles/04-sweep.gb", 0xcc63);
}
