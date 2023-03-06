// Copyright 2023 Remi Bernotavicius

use super::types::{LabelOrAddress, SourcePosition};
use super::Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{choice, optional, Parser};

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

pub(super) fn jr<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        string("jr").skip(spaces()).with(Condition::parser()),
        char(',').skip(optional(spaces())),
        LabelOrAddress::parser(),
    )
        .map(|(condition, _, label_or_address)| Instruction::Jr {
            condition,
            label_or_address,
        })
}
