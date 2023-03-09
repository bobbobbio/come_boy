// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, Condition, LabelOrAddress, Result, SourcePosition};
use super::LabelTable;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{attempt, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    condition: Option<Condition>,
    address: LabelOrAddress,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            string("call").skip(spaces1()),
            optional(attempt(Condition::parser().skip((
                spaces(),
                char(','),
                spaces(),
            )))),
            LabelOrAddress::parser(),
        )
            .map(|(_, condition, address)| Self { condition, address })
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        let address1 = label_table.resolve(self.address)?.value;
        match self.condition {
            Some(Condition::Carry) => Ok(LR35902Instruction::CallIfCarry { address1 }),
            Some(Condition::NoCarry) => Ok(LR35902Instruction::CallIfNoCarry { address1 }),
            Some(Condition::Zero) => Ok(LR35902Instruction::CallIfZero { address1 }),
            Some(Condition::NotZero) => Ok(LR35902Instruction::CallIfNotZero { address1 }),
            None => Ok(LR35902Instruction::Call { address1 }),
        }
    }
}
