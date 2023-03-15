// Copyright 2023 Remi Bernotavicius

use super::types::{Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::Parser;
use combine::{attempt, choice, parser::char::string};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Daa,
    Ei,
    Rla,
    Rlca,
    Rra,
    Rrca,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("daa")).map(|_| Self::Daa),
            attempt(string("ei")).map(|_| Self::Ei),
            attempt(string("rlca")).map(|_| Self::Rlca),
            attempt(string("rra")).map(|_| Self::Rra),
            attempt(string("rrca")).map(|_| Self::Rrca),
            attempt(string("rla")).map(|_| Self::Rla),
        ))
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
            Self::Ei => Ok(LR35902Instruction::EnableInterrupts),
            Self::Rla => Ok(LR35902Instruction::RotateAccumulatorLeftThroughCarry),
            Self::Rlca => Ok(LR35902Instruction::RotateAccumulatorLeft),
            Self::Rra => Ok(LR35902Instruction::RotateAccumulatorRightThroughCarry),
            Self::Rrca => Ok(LR35902Instruction::RotateAccumulatorRight),
        }
    }
}
