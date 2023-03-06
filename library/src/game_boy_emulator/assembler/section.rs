// Copyright 2023 Remi Bernotavicius

use super::types::{identifier, Address, SourcePosition};
use alloc::string::String;
use combine::parser::char::{char, spaces, string};
use combine::{between, choice, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum SectionType {
    Rom0,
    RomX,
    Vram,
    Sram,
    Wram0,
    WramX,
    Oam,
    Hram,
}

impl SectionType {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            string("ROM0").map(|_| Self::Rom0),
            string("ROMX").map(|_| Self::RomX),
            string("VRAM").map(|_| Self::Vram),
            string("Sram").map(|_| Self::Sram),
            string("WRAM0").map(|_| Self::Wram0),
            string("WRAMX").map(|_| Self::WramX),
            string("OAM").map(|_| Self::Oam),
            string("HRAM").map(|_| Self::Hram),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub _name: String,
    pub _type_: SectionType,
    pub address: Option<Address>,
}

impl Section {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            string("SECTION").skip(spaces()),
            identifier().skip(char(',')),
            SectionType::parser(),
            optional(between(char('['), char(']'), Address::parser())),
        )
            .map(|(_, name, type_, address)| Self {
                _name: name,
                _type_: type_,
                address,
            })
    }
}

#[test]
fn section() {
    use super::types::Span;
    use combine::{eof, stream::position, EasyParser as _};

    let input = "SECTION foo,ROM0[$FF34]";
    let (section, _) = Section::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();
    assert_eq!(
        section,
        Section {
            _name: "foo".into(),
            _type_: SectionType::Rom0,
            address: Some(Address::new(0xFF34, Span::from(((1, 18), (1, 23))))),
        }
    );
}
