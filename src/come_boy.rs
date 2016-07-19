extern crate argparse;

use argparse::ArgumentParser;
use std::fs::File;
use std::io::Read;

fn main()
{
    // List of files
    let mut files : Vec<String> = Vec::new();

    // Parse the arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("8080 Emulator");
        ap.refer(&mut files).add_argument("files", argparse::Collect, "Files");
        ap.parse_args_or_exit();
    }

    for arg in &files {
        let mut file = File::open(&arg).ok().expect("open fail");
        let mut rom : Vec<u8> = vec![];
        file.read_to_end(&mut rom).ok().expect("Failed to read ROM");
    }
}
