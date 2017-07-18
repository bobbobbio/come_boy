// Copyright 2017 Remi Bernotavicius

extern crate argparse;
extern crate come_boy;

use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{Read, Result, ErrorKind};
use std::process::exit;

use come_boy::intel_8080_emulator::disassemble_8080_rom;
use come_boy::lr35902_emulator::disassemble_lr35902_rom;
use come_boy::game_boy_emulator::disassemble_game_boy_rom;

macro_rules! println_stderr {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(_) = writeln!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.")
            }
        }
    )
}

fn disassemble_rom(rom: &Vec<u8>, instruction_set: &String, include_opcodes: bool) -> Result<()>
{
    if instruction_set == "GameBoy" {
        return disassemble_game_boy_rom(&rom, include_opcodes);
    } else if instruction_set == "LR35902" {
        return disassemble_lr35902_rom(&rom, include_opcodes);
    } else if instruction_set == "8080" {
        return disassemble_8080_rom(&rom, include_opcodes);
    } else {
        panic!("Unknown instruction set {}", instruction_set);
    }
}

fn read_rom_from_file(file_path: &String, mut rom: &mut Vec<u8>) -> Result<()>
{
    let mut file = try!(File::open(&file_path));
    try!(file.read_to_end(&mut rom));
    Ok(())
}

fn main()
{
    // List of files
    let mut files : Vec<String> = Vec::new();
    let mut instruction_set = String::new();
    let mut include_opcodes = true;

    // Parse the arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("GameBoy/LR35902/8080 Dissasembler");
        ap.refer(&mut instruction_set)
            .add_option(&["-i", "--instruction-set"], Store,
                "Instruction set to use (LR35902 or 8080)")
            .required();
        ap.refer(&mut include_opcodes)
            .add_option(&["-p", "--include-opcodes"], Store,
                "Include raw opcodes along with assembly");
        ap.refer(&mut files).add_argument("files", argparse::Collect, "Files");
        ap.parse_args_or_exit();
    }

    let mut return_code = 0;
    for file_path in &files {
        let mut rom : Vec<u8> = vec![];
        if let Err(e) = read_rom_from_file(file_path, &mut rom) {
            println_stderr!("Failed to read file {}: {}", file_path, e);
            return_code = 1;
            continue;
        }
        if let Err(e) = disassemble_rom(&rom, &instruction_set, include_opcodes) {
            match e.kind() {
                ErrorKind::BrokenPipe => {},
                _ => {
                    println_stderr!("Error {}", e);
                    return_code = 1;
                }
            }
        }
    }
    exit(return_code);
}
