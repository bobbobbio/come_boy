// Copyright 2018 Remi Bernotavicius

extern crate argparse;
extern crate come_boy;

use argparse::{ArgumentParser, StoreTrue};
use std::process::exit;

use come_boy::game_boy_emulator;

use std::fs::File;
use std::io::Read;

fn main() {
    // Path to emulator
    let mut emulator_path = String::new();
    let mut rom_path = String::new();
    let mut pc_only = false;

    // Parse the arguments
    let mut ap = ArgumentParser::new();
    ap.set_description("Come Boy Tandem Runner");
    ap.refer(&mut emulator_path).add_argument(
        "emulator path",
        argparse::Store,
        "Path to replay file.",
    );
    ap.refer(&mut rom_path)
        .add_argument("rom path", argparse::Store, "Path to rom");
    ap.refer(&mut pc_only)
        .add_option(&["--pc-only"], StoreTrue, "Only compare program counters");
    ap.parse_args_or_exit();
    drop(ap);

    let mut file = File::open(&rom_path).unwrap();
    let mut rom: Vec<u8> = vec![];
    file.read_to_end(&mut rom).unwrap();

    game_boy_emulator::run_in_tandem_with(&emulator_path, &rom, pc_only);
    exit(0);
}
