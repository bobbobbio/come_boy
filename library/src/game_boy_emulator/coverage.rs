// Copyright 2022 Remi Bernotavicius

use super::{disassembler, GameBoyEmulator};
use crate::io;
use crate::io::Write as _;
use crate::lr35902_emulator::{LR35902InstructionType, NUM_INSTRUCTIONS};
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use enum_iterator::IntoEnumIterator as _;
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

#[derive(Serialize, Deserialize)]
pub struct CoverageData {
    address_to_count: BTreeMap<u16, u64>,
    instruction_stats: Vec<u64>,
}

impl CoverageData {
    pub fn new() -> Self {
        Self {
            address_to_count: BTreeMap::new(),
            instruction_stats: vec![0; NUM_INSTRUCTIONS],
        }
    }

    pub fn sample(&mut self, e: &GameBoyEmulator) {
        let pc = e.cpu.read_program_counter();
        *self.address_to_count.entry(pc).or_insert(0) += 1;
        if let Some(instr) = e.cpu.get_last_instruction() {
            self.instruction_stats[instr.to_type() as usize] += 1;
        }
    }
}

pub fn display(
    rom: &[u8],
    input: impl io::Read,
    visited_threshold: Option<f64>,
    mut output: impl io::Write,
) -> Result<()> {
    let data: CoverageData = crate::codec::deserialize_from(input)?;
    let ma = disassembler::ROMAccessor::new(rom);
    let mut index = 0;
    writeln!(
        &mut output,
        "{} distinct PC addresses captured",
        data.address_to_count.len()
    )?;

    if let Some(visited_threshold) = visited_threshold {
        writeln!(
            &mut output,
            "only showing lines visited more than or equal to {visited_threshold:.2}% of the time"
        )?;
    } else {
        writeln!(&mut output, "only showing lines visited at least once",)?;
    }

    let total: u64 = data.address_to_count.values().sum();

    while index < rom.len() as u16 {
        let mut line = Vec::new();

        let count = data.address_to_count.get(&index).copied().unwrap_or(0);
        let percentage = (count * 100) as f64 / total as f64;
        write!(&mut line, "{count:010} ({percentage:.2}%) times ")?;

        let mut disassembler = disassembler::create_disassembler(&ma, &mut line);
        disassembler.index = index;
        disassembler.disassemble_one(true)?;
        index = disassembler.index;

        writeln!(&mut line)?;

        if let Some(visited_threshold) = visited_threshold {
            if percentage < visited_threshold {
                continue;
            }
        }
        if count == 0 {
            continue;
        }

        output.write_all(&line[..])?;
    }

    writeln!(&mut output)?;

    let mut instr_counts = vec![];
    for instr_type in LR35902InstructionType::into_enum_iter() {
        instr_counts.push((instr_type, data.instruction_stats[instr_type as usize]));
    }
    instr_counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    for (instr_type, amount) in instr_counts {
        writeln!(&mut output, "{instr_type:?}: {amount}")?;
    }

    Ok(())
}
