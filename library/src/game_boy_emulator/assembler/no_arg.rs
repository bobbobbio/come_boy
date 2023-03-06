// Copyright 2023 Remi Bernotavicius

use super::types::{Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::string;
use combine::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Daa,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        string("daa").map(|_| Self::Daa)
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Daa => Ok(LR35902Instruction::DecimalAdjustAccumulator),
        }
    }
}
