// Copyright 2023 Remi Bernotavicius

use super::types::{
    spaces1, Condition, ConditionType, Constant, Error, LabelOrAddress, Result, SourcePosition,
};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{attempt, choice, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Call {
        condition: Option<Condition>,
        address: LabelOrAddress,
    },
    Restart {
        constant: Constant,
    },
}

fn call<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(string("call").skip(spaces1())),
        optional(attempt(Condition::parser().skip((
            spaces(),
            char(','),
            spaces(),
        )))),
        LabelOrAddress::parser(),
    )
        .map(|(_, condition, address)| Instruction::Call { condition, address })
}

fn restart<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (attempt(string("rst").skip(spaces1())), Constant::parser())
        .map(|(_, constant)| Instruction::Restart { constant })
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((call(), restart()))
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Call { condition, address } => {
                let address1 = label_table.resolve(address)?.value;
                if let Some(condition) = condition {
                    match condition.value {
                        ConditionType::Carry => Ok(LR35902Instruction::CallIfCarry { address1 }),
                        ConditionType::NoCarry => {
                            Ok(LR35902Instruction::CallIfNoCarry { address1 })
                        }
                        ConditionType::Zero => Ok(LR35902Instruction::CallIfZero { address1 }),
                        ConditionType::NotZero => {
                            Ok(LR35902Instruction::CallIfNotZero { address1 })
                        }
                    }
                } else {
                    Ok(LR35902Instruction::Call { address1 })
                }
            }
            Self::Restart { constant } => {
                let data1 = constant.require_u8()? >> 3;
                if data1 > 7 {
                    return Err(Error::new("value too large", constant.span));
                }
                Ok(LR35902Instruction::Restart { data1 })
            }
        }
    }
}
