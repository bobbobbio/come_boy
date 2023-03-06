// Copyright 2023 Remi Bernotavicius

use super::types::{Register, SourcePosition};
use super::Instruction;
use combine::parser::char::{spaces, string};
use combine::Parser;

pub(super) fn and<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    string("and")
        .skip(spaces())
        .with(Register::parser())
        .map(|register| Instruction::And { register })
}
