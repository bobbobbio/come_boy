// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, Condition, ConditionType, Result, SourcePosition};
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
    attempt(string("reti")).map(|_| Instruction::Reti)
}

pub fn ret<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(string("ret"))
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
            Self::Ret { condition } => {
                if let Some(condition) = condition {
                    match condition.value {
                        ConditionType::Carry => Ok(LR35902Instruction::ReturnIfCarry),
                        ConditionType::NoCarry => Ok(LR35902Instruction::ReturnIfNoCarry),
                        ConditionType::Zero => Ok(LR35902Instruction::ReturnIfZero),
                        ConditionType::NotZero => Ok(LR35902Instruction::ReturnIfNotZero),
                    }
                } else {
                    Ok(LR35902Instruction::ReturnUnconditionally)
                }
            }
            Self::Reti => Ok(LR35902Instruction::ReturnAndEnableInterrupts),
        }
    }
}
