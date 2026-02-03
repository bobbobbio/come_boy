// Copyright 2026 Remi Bernotavicius

use come_boy::game_boy_emulator::assemble;
use come_boy::game_boy_emulator::{GameBoyEmulator, GameBoyOps, GamePak};
use criterion::{criterion_group, criterion_main, measurement::WallTime, Bencher, Criterion};

fn benchmark_assembly(b: &mut Bencher<'_, WallTime>, assembly: &str, start_pc: u16) {
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

fn small_loop(c: &mut Criterion) {
    c.bench_function("small_loop", |b| {
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
        )
    });
}

criterion_group!(benches, small_loop);
criterion_main!(benches);
