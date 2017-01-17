extern crate argparse;
extern crate come_boy;

use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{Read, Result, ErrorKind};
use std::process::exit;

use come_boy::emulator_8080::disassemble_8080_rom;
use come_boy::emulator_lr35902::disassemble_lr35902_rom;

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

fn disassemble_rom(rom: &Vec<u8>, instruction_set: &String) -> Result<()>
{
    if instruction_set == "LR35902" {
        return disassemble_lr35902_rom(&rom);
    } else if instruction_set == "8080" {
        return disassemble_8080_rom(&rom);
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

    // Parse the arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("LR35902/8080 Dissasembler");
        ap.refer(&mut instruction_set)
            .add_option(&["-i", "--instruction-set"], Store,
                "Instruction set to use (LR35902 or 8080)")
            .required();
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
        if let Err(e) = disassemble_rom(&rom, &instruction_set) {
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
