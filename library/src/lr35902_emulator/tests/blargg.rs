// Copyright 2021 Remi Bernotavicius

use super::{read_screen_message, read_test_rom};
use crate::emulator_common::{MemoryAccessor, SimpleMemoryAccessor};
use crate::lr35902_emulator::LR35902Emulator;

fn load_rom(memory_accessor: &mut SimpleMemoryAccessor, rom: &[u8]) {
    memory_accessor.memory[0..rom.len()].clone_from_slice(rom);
}

pub fn read_blargg_test_rom(name: &str) -> Vec<u8> {
    read_test_rom("blargg_test_roms", name)
}

fn run_emulator_until_pc<M: MemoryAccessor>(
    e: &mut LR35902Emulator,
    memory_accessor: &mut M,
    stop_address: u16,
) {
    let mut pc = e.read_program_counter();
    // This address is where the ROM ends.  At this address is an infinite loop where normally the
    // ROM will sit at forever.
    while pc != stop_address {
        e.run_one_instruction(memory_accessor);
        pc = e.read_program_counter();
    }
}

fn run_blargg_test_rom<M: MemoryAccessor>(
    e: &mut LR35902Emulator,
    memory_accessor: &mut M,
    stop_address: u16,
) {
    run_emulator_until_pc(e, memory_accessor, stop_address);
    assert_blargg_test_rom_success(memory_accessor);
}

pub fn assert_blargg_test_rom_success<M: MemoryAccessor>(memory_accessor: &M) {
    let message = read_screen_message(memory_accessor);

    // The message ends with 'Passed' when the test was successful
    assert!(message.ends_with("Passed\n"), "{}", message);
}

fn run_blargg_test_rom_cpu_instrs(name: &str, address: u16) {
    let mut e = LR35902Emulator::new();
    let mut memory_accessor = SimpleMemoryAccessor::new();
    load_rom(&mut memory_accessor, &read_blargg_test_rom(name));
    run_blargg_test_rom(&mut e, &mut memory_accessor, address);
}

#[test]
fn blargg_test_rom_cpu_instrs_1_special() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/01-special.gb", 0xc7d2);
}

#[test]
fn blargg_test_rom_cpu_instrs_3_op_sp_hl() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/03-op sp,hl.gb", 0xcb44);
}

#[test]
fn blargg_test_rom_cpu_instrs_4_op_r_imm() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/04-op r,imm.gb", 0xcb35);
}

#[test]
fn blargg_test_rom_cpu_instrs_5_op_rp() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/05-op rp.gb", 0xcb31);
}

#[test]
fn blargg_test_rom_cpu_instrs_6_ld_r_r() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/06-ld r,r.gb", 0xcc5f);
}

#[test]
fn blargg_test_rom_cpu_instrs_7_jr_jp_call_ret_rst() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/07-jr,jp,call,ret,rst.gb", 0xcbb0);
}

#[test]
fn blargg_test_rom_cpu_instrs_8_misc_instrs() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/08-misc instrs.gb", 0xcb91);
}

#[test]
fn blargg_test_rom_cpu_instrs_9_op_r_r() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/09-op r,r.gb", 0xce67);
}

#[test]
fn blargg_test_rom_cpu_instrs_10_bit_ops() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/10-bit ops.gb", 0xcf58);
}

#[test]
fn blargg_test_rom_cpu_instrs_11_op_a_hl() {
    run_blargg_test_rom_cpu_instrs("cpu_instrs/individual/11-op a,(hl).gb", 0xcc62);
}
