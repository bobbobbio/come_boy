extern crate argparse;
extern crate come_boy;

use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::Read;

use come_boy::emulator_8080::{disassemble_8080_rom};
use come_boy::emulator_lr35902::{disassemble_lr35902_rom};

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

    for arg in &files {
        let mut file = File::open(&arg).unwrap();
        let mut rom : Vec<u8> = vec![];
        file.read_to_end(&mut rom).unwrap();

        if instruction_set == "LR35902" {
            disassemble_lr35902_rom(&rom).unwrap();
        } else if instruction_set == "8080" {
            disassemble_8080_rom(&rom).unwrap();
        } else {
            panic!("Unknown instruction set {}", instruction_set);
        }
    }
}
