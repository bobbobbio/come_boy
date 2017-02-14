// Copyright 2017 Remi Bernotavicius

extern crate argparse;
extern crate come_boy;

use argparse::ArgumentParser;
use std::fs::File;
use std::io::{Read, Result};
use std::process::exit;

use come_boy::lr35902_emulator;

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

    // Parse the arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Come Boy Debugger");
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
        lr35902_emulator::run_debugger(&rom);
    }
    exit(return_code);
}
