// Copyright 2023 Remi Bernotavicius

use super::types::{
    spaces1, spanned, Condition, ConditionType, Error, LabelOrAddress, Result, SourcePosition, Span,
};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::Parser;
use combine::{
    attempt, choice, optional,
    parser::char::{char, spaces, string},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Jr,
    Jp,
}

impl Keyword {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("jr")).map(|_| Self::Jr),
            attempt(string("jp")).map(|_| Self::Jp),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum JumpDestination {
    HlAddress(Span),
    Address(LabelOrAddress),
}

impl JumpDestination {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            spanned(string("[hl]")).map(|(_, span)| Self::HlAddress(span)),
            LabelOrAddress::parser().map(Self::Address),
        ))
    }

    fn require_address(self) -> Result<LabelOrAddress> {
        match self {
            Self::Address(address) => Ok(address),
            Self::HlAddress(span) => Err(Error::new("[hl] not allowed", span)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    keyword: Keyword,
    condition: Option<Condition>,
    destination: JumpDestination,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            attempt(Keyword::parser().skip(spaces1())),
            optional(Condition::parser().skip((spaces(), char(','), spaces()))),
            JumpDestination::parser(),
        )
            .map(|(keyword, condition, destination)| Self {
                keyword,
                condition,
                destination,
            })
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self.keyword {
            Keyword::Jr => {
                let dest = label_table.resolve(self.destination.require_address()?)?;
                let next_address = current_address + 2;
                let difference = dest.value as i32 - next_address as i32;
                if difference > i8::MAX as i32 || difference < i8::MIN as i32 {
                    return Err(Error::new("relative jump too far", dest.span));
                }
                let data1 = (difference as i8) as u8;

                if let Some(condition) = self.condition {
                    Ok(match condition.value {
                        ConditionType::NoCarry => {
                            LR35902Instruction::JumpRelativeIfNoCarry { data1 }
                        }
                        ConditionType::NotZero => {
                            LR35902Instruction::JumpRelativeIfNotZero { data1 }
                        }
                        ConditionType::Carry => LR35902Instruction::JumpRelativeIfCarry { data1 },
                        ConditionType::Zero => LR35902Instruction::JumpRelativeIfZero { data1 },
                    })
                } else {
                    Ok(LR35902Instruction::JumpRelative { data1 })
                }
            }
            Keyword::Jp => match self.destination {
                JumpDestination::Address(address) => {
                    let dest = label_table.resolve(address)?;

                    let address1 = dest.value;
                    if let Some(condition) = self.condition {
                        Ok(match condition.value {
                            ConditionType::NoCarry => {
                                LR35902Instruction::JumpIfNoCarry { address1 }
                            }
                            ConditionType::NotZero => {
                                LR35902Instruction::JumpIfNotZero { address1 }
                            }
                            ConditionType::Carry => LR35902Instruction::JumpIfCarry { address1 },
                            ConditionType::Zero => LR35902Instruction::JumpIfZero { address1 },
                        })
                    } else {
                        Ok(LR35902Instruction::Jump { address1 })
                    }
                }
                JumpDestination::HlAddress { .. } => {
                    if let Some(condition) = self.condition {
                        return Err(Error::new("condition not allowed", condition.span));
                    }
                    Ok(LR35902Instruction::LoadProgramCounter)
                }
            },
        }
    }
}
