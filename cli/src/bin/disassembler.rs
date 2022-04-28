// Copyright 2017 Remi Bernotavicius

use come_boy::game_boy_emulator::{disassemble_game_boy_rom, Result};
use come_boy::intel_8080_emulator::disassemble_8080_rom;
use come_boy::lr35902_emulator::disassemble_lr35902_rom;
use std::fs::File;
use std::io::{self, Read as _};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

enum InstructionSet {
    GameBoy,
    LR35902,
    Intel8080,
}

impl FromStr for InstructionSet {
    type Err = clap::Error;
    fn from_str(s: &str) -> clap::Result<Self> {
        match s.to_uppercase().as_ref() {
            "GAMEBOY" => Ok(InstructionSet::GameBoy),
            "LR35902" => Ok(InstructionSet::LR35902),
            "INTEL8080" => Ok(InstructionSet::Intel8080),
            _ => Err(clap::error::Error::raw(
                clap::ErrorKind::InvalidValue,
                "invalid instruction-set",
            )),
        }
    }
}

fn disassemble_rom(
    rom: &[u8],
    instruction_set: InstructionSet,
    include_opcodes: bool,
    output: impl io::Write,
) -> Result<()> {
    match instruction_set {
        InstructionSet::GameBoy => disassemble_game_boy_rom(rom, include_opcodes, output),
        InstructionSet::LR35902 => disassemble_lr35902_rom(rom, include_opcodes, output),
        InstructionSet::Intel8080 => disassemble_8080_rom(rom, include_opcodes, output),
    }?;
    Ok(())
}

#[derive(StructOpt)]
#[structopt(
    name = "Come Boy Disassembler",
    about = "Game Boy / LR35902 / Intel 8080 disassembler"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    #[structopt(long = "instruction-set", default_value = "GameBoy")]
    instruction_set: InstructionSet,
    #[structopt(long = "hide-opcodes")]
    hide_opcodes: bool,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let mut rom_file = File::open(&options.rom)?;
    let mut rom: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom)?;

    disassemble_rom(
        &rom,
        options.instruction_set,
        !options.hide_opcodes,
        io::stdout(),
    )
}
