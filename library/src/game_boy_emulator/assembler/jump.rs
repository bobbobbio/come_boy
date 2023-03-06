// Copyright 2023 Remi Bernotavicius

use super::types::{Error, LabelOrAddress, Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{choice, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Jr {
        condition: Condition,
        label_or_address: LabelOrAddress,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Condition {
    NotZero,
    NoCarry,
    Zero,
    Carry,
}

impl Condition {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            string("nc").map(|_| Self::NoCarry),
            string("nz").map(|_| Self::NotZero),
            string("c").map(|_| Self::Carry),
            string("z").map(|_| Self::Zero),
        ))
    }
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            string("jr").skip(spaces()).with(Condition::parser()),
            char(',').skip(optional(spaces())),
            LabelOrAddress::parser(),
        )
            .map(|(condition, _, label_or_address)| Self::Jr {
                condition,
                label_or_address,
            })
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Jr {
                condition,
                label_or_address,
            } => {
                let next_address = current_address + 2;
                let dest = label_table.resolve(label_or_address)?;
                let difference = dest.value as i32 - next_address as i32;
                if difference > i8::MAX as i32 || difference < i8::MIN as i32 {
                    return Err(Error::new("relative jump too far", dest.span));
                }
                let data1 = (difference as i8) as u8;

                Ok(match condition {
                    Condition::NoCarry => LR35902Instruction::JumpRelativeIfNoCarry { data1 },
                    Condition::NotZero => LR35902Instruction::JumpRelativeIfNotZero { data1 },
                    Condition::Carry => LR35902Instruction::JumpRelativeIfCarry { data1 },
                    Condition::Zero => LR35902Instruction::JumpRelativeIfZero { data1 },
                })
            }
        }
    }
}
