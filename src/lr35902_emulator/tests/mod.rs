// Copyright 2021 Remi Bernotavicius

use crate::emulator_common::disassembler::{MemoryAccessor, MemoryIterator};
use std::fs::File;
use std::io::Read;

pub mod blargg;

pub fn read_test_rom(rom_dir: &str, name: &str) -> Vec<u8> {
    let mut rom: Vec<u8> = vec![];
    let mut file = File::open(format!("{}/{}", rom_dir, name))
        .ok()
        .expect("Did you forget to download the test roms?");
    file.read_to_end(&mut rom).unwrap();
    rom
}

/// Reads the visible tiles on the screen as text. It assumes the tile mapping is ASCII.
pub fn read_screen_message<M: MemoryAccessor>(memory_accessor: &M) -> String {
    let mut message = String::new();

    // Repeat some LCD logic here to read the tiles on screen to keep these test independent of the
    // LCD controller code.
    let screen_width = 160;
    let screen_height = 144;
    let pixels_per_tile = 8;

    // When there is too much output the tiles scroll on the screen vertically, so we have to take
    // that into account here.
    let scy = memory_accessor.read_memory(0xFF42);
    let scx = memory_accessor.read_memory(0xFF43);
    assert_eq!(scx, 0);

    let bg_tiles_per_row = 32;
    let skip = scy as u16 / pixels_per_tile * bg_tiles_per_row;

    let screen_tiles_per_row = screen_width / pixels_per_tile;
    let screen_tiles_per_column = screen_height / pixels_per_tile;
    let visible_rows = bg_tiles_per_row * screen_tiles_per_column;

    // Two ranges because of the vertical wrapping of the tiles
    let start1 = 0x9800 + skip;
    let end1 = std::cmp::min(start1 + visible_rows, 0x9c00);

    let start2 = 0x9800;
    let end2 = start2 + visible_rows - (end1 - start1);

    for r in vec![start1..end1, start2..end2].into_iter() {
        let iter = &mut MemoryIterator::new(memory_accessor, r).peekable();
        while iter.peek() != None {
            for c in iter.take(screen_tiles_per_row as usize) {
                // This is where we assume the tile number uses ASCII
                message.push(c as char);
            }
            message = String::from(message.trim_end());
            message.push('\n');

            // Skip the part that is off-screen
            for _ in 0..(bg_tiles_per_row - screen_tiles_per_row) {
                iter.next();
            }
        }
    }
    message
}
