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

/// "This test checks that OAM DMA copies all bytes correctly"
#[test]
fn mooneye_test_rom_acceptance_oam_dma_basic() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_mooneye_test_rom("acceptance/oam_dma/basic.gb"),
        None,
    ));
    run_mooneye_test_rom(&mut e, 0x486e);
}

/// "This test checks what happens if you read the DMA register. Reads should always simply return
/// the last written value, regardless of the state of the OAM DMA transfer or other things"
#[test]
fn mooneye_test_rom_acceptance_oam_dma_reg_read() {
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(GamePak::new(
        &read_mooneye_test_rom("acceptance/oam_dma/reg_read.gb"),
        None,
    ));
    run_mooneye_test_rom(&mut e, 0x486e);
}
