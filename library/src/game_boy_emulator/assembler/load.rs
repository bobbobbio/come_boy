// Copyright 2023 Remi Bernotavicius

use super::types::{LabelOrAddress, Register, Result, SourcePosition};
use super::LabelTable;
use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{between, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Ldh {
        register: Register,
        label_or_address: LabelOrAddress,
    },
}

impl Instruction {
    pub(crate) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            string("ldh").skip(spaces()).with(Register::parser()),
            char(',').skip(optional(spaces())),
            between(char('['), char(']'), LabelOrAddress::parser()),
        )
            .map(|(register, _, label_or_address)| Self::Ldh {
                register,
                label_or_address,
            })
    }
}

#[test]
fn ldh_load_address_too_low() {
    use super::types::{Error, Span};

    let err = super::assemble(
        "
    SECTION test,ROM0[$036C]
        ldh  a, [$FE85]
        ",
    )
    .unwrap_err();
    assert_eq!(
        err,
        Error::new("address must start with FF", Span::from(((3, 18), (3, 23))))
    );
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Ldh {
                register,
                label_or_address,
            } => {
                register.require_value(Intel8080Register::A)?;
                let data1 = label_table.resolve(label_or_address)?.shorten()?;
                Ok(LR35902Instruction::LoadAccumulatorDirectOneByte { data1 })
            }
        }
    }
}
