// Copyright 2022 Remi Bernotavicius

use bin_common::Result;
use structopt::StructOpt;

#[path = "../../bin_common/mod.rs"]
mod bin_common;

mod coverage;
mod debugger;
mod disassembler;
mod game_pak;
mod replay;
mod screenshot;
mod tandem;

#[derive(StructOpt)]
#[structopt(name = "Come Boy Diagnostics Runner")]
enum Options {
    Coverage(coverage::Options),
    Debugger(debugger::Options),
    Disassembler(disassembler::Options),
    GamePak(game_pak::Options),
    Replay(replay::Options),
    Screenshot(screenshot::Options),
    Tandem(tandem::Options),
}

impl Options {
    fn main(self) -> Result<()> {
        match self {
            Self::Coverage(opts) => coverage::main(opts)?,
            Self::Debugger(opts) => debugger::main(opts)?,
            Self::Disassembler(opts) => disassembler::main(opts)?,
            Self::GamePak(opts) => game_pak::main(opts)?,
            Self::Replay(opts) => replay::main(opts)?,
            Self::Screenshot(opts) => screenshot::main(opts)?,
            Self::Tandem(opts) => tandem::main(opts)?,
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let options = Options::from_args();
    options.main()
}
