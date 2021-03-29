#[cfg(test)]
#[allow(unused_imports)]
use crate::game_boy_emulator::{do_rom_test, do_save_state_rom_test, Result};
#[test]
fn pokemon_red_100000000() -> Result<()> {
    do_rom_test(
        "test/roms/pokemon_red.gb",
        100000000u64,
        "test/expectations/pokemon_red/100000000.bmp",
        None,
    )
}
#[test]
fn pokemon_red_100000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/pokemon_red.gb",
        100000000u64,
        "test/expectations/pokemon_red/100000000.bmp",
        None,
    )
}
#[test]
fn f1race_80000000() -> Result<()> {
    do_rom_test(
        "test/roms/f1race.gb",
        80000000u64,
        "test/expectations/f1race/80000000.bmp",
        None,
    )
}
#[test]
fn f1race_80000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/f1race.gb",
        80000000u64,
        "test/expectations/f1race/80000000.bmp",
        None,
    )
}
#[test]
fn zelda_60000000() -> Result<()> {
    do_rom_test(
        "test/roms/zelda.gb",
        60000000u64,
        "test/expectations/zelda/60000000.bmp",
        None,
    )
}
#[test]
fn zelda_60000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/zelda.gb",
        60000000u64,
        "test/expectations/zelda/60000000.bmp",
        None,
    )
}
#[test]
fn zelda_200000000_replay1() -> Result<()> {
    do_rom_test(
        "test/roms/zelda.gb",
        200000000u64,
        "test/expectations/zelda/200000000_replay1.bmp",
        Some("test/expectations/zelda/replay1.replay"),
    )
}
#[test]
fn zelda_200000000_replay1_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/zelda.gb",
        200000000u64,
        "test/expectations/zelda/200000000_replay1.bmp",
        Some("test/expectations/zelda/replay1.replay"),
    )
}
#[test]
fn zelda_7000000_replay1() -> Result<()> {
    do_rom_test(
        "test/roms/zelda.gb",
        7000000u64,
        "test/expectations/zelda/7000000_replay1.bmp",
        Some("test/expectations/zelda/replay1.replay"),
    )
}
#[test]
fn zelda_7000000_replay1_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/zelda.gb",
        7000000u64,
        "test/expectations/zelda/7000000_replay1.bmp",
        Some("test/expectations/zelda/replay1.replay"),
    )
}
#[test]
fn aladdin_60000000() -> Result<()> {
    do_rom_test(
        "test/roms/aladdin.gb",
        60000000u64,
        "test/expectations/aladdin/60000000.bmp",
        None,
    )
}
#[test]
fn aladdin_60000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/aladdin.gb",
        60000000u64,
        "test/expectations/aladdin/60000000.bmp",
        None,
    )
}
#[test]
fn tetris_10000000() -> Result<()> {
    do_rom_test(
        "test/roms/tetris.gb",
        10000000u64,
        "test/expectations/tetris/10000000.bmp",
        None,
    )
}
#[test]
fn tetris_10000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/tetris.gb",
        10000000u64,
        "test/expectations/tetris/10000000.bmp",
        None,
    )
}
#[test]
fn tetris_3000000() -> Result<()> {
    do_rom_test(
        "test/roms/tetris.gb",
        3000000u64,
        "test/expectations/tetris/3000000.bmp",
        None,
    )
}
#[test]
fn tetris_3000000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/tetris.gb",
        3000000u64,
        "test/expectations/tetris/3000000.bmp",
        None,
    )
}
#[test]
fn tetris_40000000_replay1() -> Result<()> {
    do_rom_test(
        "test/roms/tetris.gb",
        40000000u64,
        "test/expectations/tetris/40000000_replay1.bmp",
        Some("test/expectations/tetris/replay1.replay"),
    )
}
#[test]
fn tetris_40000000_replay1_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/tetris.gb",
        40000000u64,
        "test/expectations/tetris/40000000_replay1.bmp",
        Some("test/expectations/tetris/replay1.replay"),
    )
}
#[test]
fn kirby_dream_land_6800000() -> Result<()> {
    do_rom_test(
        "test/roms/kirby.gb",
        6800000u64,
        "test/expectations/kirby_dream_land/6800000.bmp",
        None,
    )
}
#[test]
fn kirby_dream_land_6800000_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/kirby.gb",
        6800000u64,
        "test/expectations/kirby_dream_land/6800000.bmp",
        None,
    )
}
#[test]
fn kirby_dream_land_27000000_replay1() -> Result<()> {
    do_rom_test(
        "test/roms/kirby.gb",
        27000000u64,
        "test/expectations/kirby_dream_land/27000000_replay1.bmp",
        Some("test/expectations/kirby_dream_land/replay1.replay"),
    )
}
#[test]
fn kirby_dream_land_27000000_replay1_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/kirby.gb",
        27000000u64,
        "test/expectations/kirby_dream_land/27000000_replay1.bmp",
        Some("test/expectations/kirby_dream_land/replay1.replay"),
    )
}
#[test]
fn kirby_dream_land_50000000_replay1() -> Result<()> {
    do_rom_test(
        "test/roms/kirby.gb",
        50000000u64,
        "test/expectations/kirby_dream_land/50000000_replay1.bmp",
        Some("test/expectations/kirby_dream_land/replay1.replay"),
    )
}
#[test]
fn kirby_dream_land_50000000_replay1_save_state() -> Result<()> {
    do_save_state_rom_test(
        "test/roms/kirby.gb",
        50000000u64,
        "test/expectations/kirby_dream_land/50000000_replay1.bmp",
        Some("test/expectations/kirby_dream_land/replay1.replay"),
    )
}
