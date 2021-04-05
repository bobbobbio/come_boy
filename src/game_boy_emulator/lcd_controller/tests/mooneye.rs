// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::tests::mooneye::{read_mooneye_test_rom, run_mooneye_test_rom};
use crate::game_boy_emulator::{GameBoyEmulator, GamePak};

/// "This test checks that the OAM area has no unused bits. On DMG the sprite flags have unused
/// bits, but they are still writable and readable normally"
#[test]
fn mooneye_test_rom_acceptance_bits_mem_oam() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_mooneye_test_rom("acceptance/bits/mem_oam.gb"),
        None,
    ));
    run_mooneye_test_rom(&mut e, 0x486e);
}
