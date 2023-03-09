// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, Condition, Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::string;
use combine::{attempt, choice, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Ret { condition: Option<Condition> },
    Reti,
}

pub fn reti<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    string("reti").map(|_| Instruction::Reti)
}

pub fn ret<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    string("ret")
        .with(optional(attempt(spaces1().with(Condition::parser()))))
        .map(|condition| Instruction::Ret { condition })
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((attempt(reti()), ret()))
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Ret { condition } => match condition {
                Some(Condition::Carry) => Ok(LR35902Instruction::ReturnIfCarry),
                Some(Condition::NoCarry) => Ok(LR35902Instruction::ReturnIfNoCarry),
                Some(Condition::Zero) => Ok(LR35902Instruction::ReturnIfZero),
                Some(Condition::NotZero) => Ok(LR35902Instruction::ReturnIfNotZero),
                None => Ok(LR35902Instruction::ReturnUnconditionally),
            },
            Self::Reti => Ok(LR35902Instruction::ReturnAndEnableInterrupts),
        }
    }
}
