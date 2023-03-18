// Copyright 2023 Remi Bernotavicius

use super::types::{Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::Parser;
use combine::{attempt, choice, parser::char::string};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Di,
    Ei,
    Halt,
    Nop,
    Stop,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("di")).map(|_| Self::Di),
            attempt(string("ei")).map(|_| Self::Ei),
            attempt(string("halt")).map(|_| Self::Halt),
            attempt(string("nop")).map(|_| Self::Nop),
            attempt(string("stop")).map(|_| Self::Stop),
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
            Self::Di => Ok(LR35902Instruction::DisableInterrupts),
            Self::Ei => Ok(LR35902Instruction::EnableInterrupts),
            Self::Halt => Ok(LR35902Instruction::Halt),
            Self::Nop => Ok(LR35902Instruction::NoOperation),
            Self::Stop => Ok(LR35902Instruction::HaltUntilButtonPress),
        }
    }
}
