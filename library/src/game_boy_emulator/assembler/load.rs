// Copyright 2023 Remi Bernotavicius

use super::types::{LabelOrAddress, Register, SourcePosition};
use super::Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{between, optional, Parser};

pub(super) fn ldh<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        string("ldh").skip(spaces()).with(Register::parser()),
        char(',').skip(optional(spaces())),
        between(char('['), char(']'), LabelOrAddress::parser()),
    )
        .map(|(register, _, label_or_address)| Instruction::Ldh {
            register,
            label_or_address,
        })
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
