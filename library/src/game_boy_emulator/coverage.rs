// Copyright 2022 Remi Bernotavicius

use super::{disassembler, GameBoyEmulator};
use crate::io;
use alloc::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DecodingError(crate::codec::Error),
}

type Result<T> = core::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::codec::Error> for Error {
    fn from(e: crate::codec::Error) -> Self {
        Self::DecodingError(e)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CoverageData {
    address_to_count: BTreeMap<u16, u64>,
}

impl CoverageData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn sample(&mut self, e: &GameBoyEmulator) {
        let pc = e.cpu.read_program_counter();
        *self.address_to_count.entry(pc).or_insert(0) += 1;
    }
}

pub fn display(rom: &[u8], input: impl io::Read, mut output: impl io::Write) -> Result<()> {
    let data: CoverageData = crate::codec::deserialize_from(input)?;
    let ma = disassembler::ROMAccessor::new(rom);
    let mut index = 0;
    writeln!(
        &mut output,
        "{} distinct PC addresses captured",
        data.address_to_count.len()
    )?;

    while index < rom.len() as u16 {
        let count = data.address_to_count.get(&index).copied().unwrap_or(0);
        write!(&mut output, "{count:010} times ")?;

        let mut disassembler = disassembler::create_disassembler(&ma, &mut output);
        disassembler.index = index;
        disassembler.disassemble_one(true)?;
        index = disassembler.index;

        writeln!(&mut output)?;
    }
    Ok(())
}
