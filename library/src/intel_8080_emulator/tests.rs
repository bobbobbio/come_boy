// Copyright 2021 Remi Bernotavicius

use super::{Intel8080Emulator, Intel8080InstructionSetOps, Intel8080Register};
use crate::io::{self, Read};
use std::{fs::File, str};

/*  ____  _                             _   _        ____   ___  __  __
 * |  _ \(_) __ _  __ _ _ __   ___  ___| |_(_) ___  |  _ \ / _ \|  \/  |
 * | | | | |/ _` |/ _` | '_ \ / _ \/ __| __| |/ __| | |_) | | | | |\/| |
 * | |_| | | (_| | (_| | | | | (_) \__ \ |_| | (__  |  _ <| |_| | |  | |
 * |____/|_|\__,_|\__, |_| |_|\___/|___/\__|_|\___| |_| \_\\___/|_|  |_|
 *                |___/
 *
 */

fn console_print(e: &mut Intel8080Emulator, stream: &mut dyn io::Write) {
    match e.read_register(Intel8080Register::C) {
        9 => {
            let mut msg_addr = e.read_register_pair(Intel8080Register::D) as usize;
            while e.main_memory[msg_addr] != b'$' {
                write!(stream, "{}", e.main_memory[msg_addr] as char).unwrap();
                msg_addr += 1;
            }
        }
        2 => {
            write!(stream, "{}", e.read_register(Intel8080Register::E) as char).unwrap();
        }
        op => panic!("{} unknown print operation", op),
    }
}

/*
 * This test runs a ROM entitled "MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC" stored in
 * cpudiag.bin.  It tests most the instructions, and when an instruction doesn't behave the way it
 * is suppose to, it will print out the address where it failed.
 */
#[test]
fn cpu_diagnostic_8080() {
    // Load up the ROM
    let mut rom: Vec<u8> = vec![];
    {
        let mut file = File::open("cpudiag.bin").unwrap();
        file.read_to_end(&mut rom).unwrap();
    }

    let mut console_buffer: Vec<u8> = vec![];
    let mut console_print_closure =
        |e: &mut Intel8080Emulator| console_print(e, &mut console_buffer);

    let mut emulator = Intel8080Emulator::new();
    emulator.load_rom(&rom);
    // The program write to the console via a routine at address 0x0005
    emulator.add_routine(0x0005, &mut console_print_closure);
    emulator.run();
    let ascii_output = str::from_utf8(&console_buffer).unwrap();

    // When we see this string it means the program succeeded.  When it fails, we see
    // 'CPU HAS FAILED! EXIT=xxxx' where xxxx is the address it failed at.
    assert_eq!(ascii_output, "\u{c}\r\n CPU IS OPERATIONAL");
}
