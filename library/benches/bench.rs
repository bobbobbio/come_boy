// Copyright 2026 Remi Bernotavicius

use come_boy::game_boy_emulator::assemble;
use come_boy::game_boy_emulator::{GameBoyEmulator, GameBoyOps, GamePak};
use criterion::{criterion_group, criterion_main, measurement::WallTime, Bencher, Criterion};

fn benchmark_assembly(b: &mut Bencher<'_, WallTime>, assembly: &str) {
    let rom = assemble(assembly).unwrap();

    let mut ops = GameBoyOps::null();

    let game_pak = GamePak::new(&rom.bin, &mut ops.storage, None).unwrap();
    ops.load_game_pak(game_pak);

    let start_pc = *rom.labels.get("start").unwrap();
    let end_pc = *rom.labels.get("end").unwrap();

    b.iter(|| {
        let mut e = GameBoyEmulator::new();
        e.cpu.set_program_counter(start_pc);

        while e.cpu.read_program_counter() != end_pc {
            e.tick(&mut ops);
        }
    });
}

fn small_loop(c: &mut Criterion) {
    c.bench_function("small_loop", |b| {
        let loop_times = 1000u16;

        benchmark_assembly(
            b,
            &format!(
                "
                SECTION test,ROM0[$036C]
                .start
                    ld   a, ${loop_hi:x}
                    ld   b, ${loop_lo:x}
                .loop
                    dec  b
                    jr   nz, .loop
                    dec  a
                    jr   nz, .loop
                .end
                    nop
                ",
                loop_hi = (loop_times >> 8) as u8,
                loop_lo = loop_times as u8
            ),
        )
    });
}

criterion_group!(benches, small_loop);
criterion_main!(benches);
