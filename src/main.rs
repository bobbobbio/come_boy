extern crate argparse;

use argparse::ArgumentParser;
use std::fs::File;
use std::io::BufReader;

mod opcodes;

fn main() {
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
        let mut reader =
            BufReader::new(File::open(&arg).ok().expect("open fail"));
        let mut index: u64 = 0;
        loop {
            match opcodes::print_opcode(&mut reader, &mut index) {
                Err(_) => break,
                Ok(_) => continue
            }
        }
    }
}
