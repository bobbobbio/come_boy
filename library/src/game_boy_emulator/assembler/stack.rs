// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, RegisterPair, Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::Parser;
use combine::{attempt, choice, parser::char::string};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Pop { register: RegisterPair },
    Push { register: RegisterPair },
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("pop").skip(spaces1()))
                .with(RegisterPair::parser())
                .map(|register| Self::Pop { register }),
            attempt(string("push").skip(spaces1()))
                .with(RegisterPair::parser())
                .map(|register| Self::Push { register }),
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
            Self::Pop { register } => Ok(LR35902Instruction::PopDataOffStack {
                register1: register.value,
            }),
            Self::Push { register } => Ok(LR35902Instruction::PushDataOntoStack {
                register1: register.value,
            }),
        }
    }
}
