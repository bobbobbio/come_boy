// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, Constant, Register, Result, SourcePosition};
use super::LabelTable;
use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{attempt, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    left: Register,
    right: Option<Constant<u8>>,
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            string("cp").skip(spaces1()).with(Register::parser()),
            optional(attempt(
                (spaces(), char(','), spaces()).with(Constant::<u8>::parser()),
            )),
        )
            .map(|(left, right)| Self { left, right })
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        if let Some(right) = self.right {
            self.left.require_value(Intel8080Register::A)?;
            Ok(LR35902Instruction::CompareImmediateWithAccumulator { data1: right.value })
        } else {
            Ok(LR35902Instruction::CompareWithAccumulator {
                register1: self.left.value,
            })
        }
    }
}
