// Copyright 2023 Remi Bernotavicius

use super::types::{Register, Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{spaces, string};
use combine::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    And { register: Register },
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        string("and")
            .skip(spaces())
            .with(Register::parser())
            .map(|register| Self::And { register })
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::And { register } => Ok(LR35902Instruction::LogicalAndWithAccumulator {
                register1: register.value,
            }),
        }
    }
}
