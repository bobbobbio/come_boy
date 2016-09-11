extern crate argparse;
extern crate come_boy;

use argparse::ArgumentParser;
use std::fs::File;
use std::io::Read;

use come_boy::emulator_lr35902::emulator_8080::{disassemble_8080_rom};

fn main()
{
    // List of files
    let mut files : Vec<String> = Vec::new();

    // Parse the arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("8080 Dissasembler");
        ap.refer(&mut files).add_argument("files", argparse::Collect, "Files");
        ap.parse_args_or_exit();
    }

    for arg in &files {
        let mut file = File::open(&arg).unwrap();
        let mut rom : Vec<u8> = vec![];
        file.read_to_end(&mut rom).unwrap();

        disassemble_8080_rom(&rom).unwrap();
    }
}
