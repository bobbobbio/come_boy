// Copyright 2023 Remi Bernotavicius

extern crate test;

use crate::game_boy_emulator::assemble;
use crate::game_boy_emulator::{GameBoyEmulator, GameBoyOps, GamePak};

fn benchmark_assembly(b: &mut test::Bencher, assembly: &str, start_pc: u16) {
    let rom = assemble(assembly).unwrap();

    let mut ops = GameBoyOps::null();

    let game_pak = GamePak::new(&rom, &mut ops.storage, None).unwrap();
    ops.load_game_pak(game_pak);

    b.iter(|| {
        let mut e = GameBoyEmulator::new();
        e.cpu.set_program_counter(start_pc);

        let goal = e.cpu.elapsed_cycles + 10_000;
        while e.cpu.elapsed_cycles < goal {
            e.tick(&mut ops);
        }
    });
}

#[bench]
fn small_loop(b: &mut test::Bencher) {
    benchmark_assembly(
        b,
        "
    SECTION test,ROM0[$036C]
    .loop
        ldh  a, [$FF85]
        and  a
        jr   z, .loop
    ",
        0x036c,
    );
}
