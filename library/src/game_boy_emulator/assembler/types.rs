// Copyright 2023 Remi Bernotavicius

use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::IllegalInstructionError;
use alloc::format;
use alloc::string::String;
use combine::parser::char::{alpha_num, char, hex_digit};
use combine::stream::{easy, position};
use combine::{choice, many1, position, Parser};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePosition {
    line: u64,
    column: u64,
}

impl From<position::SourcePosition> for SourcePosition {
    fn from(p: position::SourcePosition) -> Self {
        Self {
            line: p.line.try_into().unwrap_or(0),
            column: p.column.try_into().unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: SourcePosition,
    pub end: Option<SourcePosition>,
}

impl From<((u64, u64), (u64, u64))> for Span {
    fn from(((line1, column1), (line2, column2)): ((u64, u64), (u64, u64))) -> Self {
        Span {
            start: SourcePosition {
                line: line1,
                column: column1,
            },
            end: Some(SourcePosition {
                line: line2,
                column: column2,
            }),
        }
    }
}

impl Span {
    fn new(start: SourcePosition, end: SourcePosition) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }

    fn unknown_end(start: SourcePosition) -> Self {
        Self { start, end: None }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub message: String,
    pub span: Span,
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    pub fn illegal_instruction(e: IllegalInstructionError, span: Span) -> Self {
        let instr = e.0;
        Self::new(format!("illegal instruction: {instr:?}"), span)
    }
}

impl From<easy::Errors<char, &str, position::SourcePosition>> for Error {
    fn from(e: easy::Errors<char, &str, position::SourcePosition>) -> Self {
        let errors = e.errors;
        Self::new(format!("{errors:?}"), Span::unknown_end(e.position.into()))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub value: Intel8080Register,
    pub span: Span,
}

impl Register {
    #[cfg(test)]
    pub fn new(value: Intel8080Register, span: Span) -> Self {
        Self { value, span }
    }

    pub fn parser<Input>() -> impl Parser<Input, Output = Register>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        spanned(choice((
            char('a').map(|_| Intel8080Register::A),
            char('b').map(|_| Intel8080Register::B),
            char('d').map(|_| Intel8080Register::D),
            char('e').map(|_| Intel8080Register::E),
            char('h').map(|_| Intel8080Register::H),
            char('l').map(|_| Intel8080Register::L),
        )))
        .map(|(value, span)| Self { value, span })
    }

    pub fn require_value(self, requirement: Intel8080Register) -> Result<()> {
        if self.value != requirement {
            return Err(Error::new(
                format!("must be register {requirement:?}"),
                self.span,
            ));
        }
        Ok(())
    }
}

fn hex_u8<Input>() -> impl Parser<Input, Output = u8>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (hex_digit(), hex_digit()).map(|(h1, h2)| u8::from_str_radix(&format!("{h1}{h2}"), 16).unwrap())
}

fn hex_u16<Input>() -> impl Parser<Input, Output = u16>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (hex_u8(), hex_u8()).map(|(h, l)| (h as u16) << 8 | l as u16)
}

pub fn identifier<Input>() -> impl Parser<Input, Output = String>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    many1(choice((alpha_num(), char('_'))))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    pub span: Span,
}

impl Label {
    #[cfg(test)]
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        spanned(char('.').with(identifier())).map(|(name, span)| Self { name, span })
    }
}

pub fn spanned<Input, Output>(
    inner: impl Parser<Input, Output = Output>,
) -> impl Parser<Input, Output = (Output, Span)>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (position(), inner, position())
        .map(|(before, thing, after)| (thing, Span::new(before.into(), after.into())))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    pub value: u16,
    pub span: Span,
}

impl Address {
    pub fn new(value: u16, span: Span) -> Self {
        Self { value, span }
    }

    pub fn shorten(self) -> Result<u8> {
        if self.value >> 8 != 0xFF {
            return Err(Error::new("address must start with FF", self.span));
        }
        Ok(self.value as u8)
    }

    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        spanned(char('$').with(hex_u16())).map(|(value, span)| Self { value, span })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LabelOrAddress {
    Label(Label),
    Address(Address),
}

impl LabelOrAddress {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            Address::parser().map(Self::Address),
            Label::parser().map(Self::Label),
        ))
    }
}
