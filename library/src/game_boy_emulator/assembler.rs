// Copyright 2023 Remi Bernotavicius

use combine::eof;
use combine::parser::char::{alpha_num, char, hex_digit, spaces, string};
use combine::stream::{easy, position};
use combine::{between, choice, many, many1, optional, position, EasyParser as _, Parser};

use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::{IllegalInstructionError, LR35902Instruction};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, format, vec};

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
    fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    fn illegal_instruction(e: IllegalInstructionError, span: Span) -> Self {
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
struct Register {
    value: Intel8080Register,
    span: Span,
}

impl Register {
    #[cfg(test)]
    fn new(value: Intel8080Register, span: Span) -> Self {
        Self { value, span }
    }

    fn parser<Input>() -> impl Parser<Input, Output = Register>
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

    fn require_value(self, requirement: Intel8080Register) -> Result<()> {
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

#[derive(Debug, PartialEq, Eq)]
enum Condition {
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

fn identifier<Input>() -> impl Parser<Input, Output = String>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    many1(choice((alpha_num(), char('_'))))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Label {
    name: String,
    span: Span,
}

impl Label {
    #[cfg(test)]
    fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        spanned(char('.').with(identifier())).map(|(name, span)| Self { name, span })
    }
}

fn spanned<Input, Output>(
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
struct Address {
    value: u16,
    span: Span,
}

impl Address {
    fn new(value: u16, span: Span) -> Self {
        Self { value, span }
    }

    fn shorten(self) -> Result<u8> {
        if self.value >> 8 != 0xFF {
            return Err(Error::new("address must start with FF", self.span));
        }
        Ok(self.value as u8)
    }

    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        spanned(char('$').with(hex_u16())).map(|(value, span)| Self { value, span })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LabelOrAddress {
    Label(Label),
    Address(Address),
}

impl LabelOrAddress {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
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

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Ldh {
        register: Register,
        label_or_address: LabelOrAddress,
    },
    And {
        register: Register,
    },
    Jr {
        condition: Condition,
        label_or_address: LabelOrAddress,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct InstructionLine {
    label: Option<Label>,
    instruction: Instruction,
    span: Span,
}

impl InstructionLine {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            optional(Label::parser().skip(spaces())),
            spanned(Instruction::parser()),
        )
            .map(|(label, (instruction, span))| Self {
                label,
                instruction,
                span,
            })
    }
}

#[test]
fn instruction_line() {
    let input = ".foo ldh  a, [$FF85]";
    let (instruction_line, _) = InstructionLine::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();
    assert_eq!(
        instruction_line,
        InstructionLine {
            label: Some(Label::new("foo", Span::from(((1, 1), (1, 5))))),
            instruction: Instruction::Ldh {
                register: Register::new(Intel8080Register::A, Span::from(((1, 11), (1, 12)))),
                label_or_address: LabelOrAddress::Address(Address {
                    value: 0xFF85,
                    span: Span::from(((1, 15), (1, 20)))
                }),
            },
            span: Span::from(((1, 6), (1, 21)))
        }
    );
}

#[derive(Debug, PartialEq, Eq)]
enum SectionType {
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
struct Section {
    _name: String,
    _type_: SectionType,
    address: Option<Address>,
}

impl Section {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
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

#[derive(Debug, PartialEq, Eq)]
enum AssemblyLine {
    Section(Section),
    Instruction(InstructionLine),
}

impl AssemblyLine {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            Section::parser().map(Self::Section),
            InstructionLine::parser().map(Self::Instruction),
        ))
    }
}

#[test]
fn assembly_line_section() {
    let input = "SECTION foo,ROM0[$FF34]";
    let (assembly_line, _) = AssemblyLine::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();
    assert_eq!(
        assembly_line,
        AssemblyLine::Section(Section {
            _name: "foo".into(),
            _type_: SectionType::Rom0,
            address: Some(Address::new(0xFF34, Span::from(((1, 18), (1, 23))))),
        })
    );
}

#[test]
fn assembly_line_instruction() {
    let input = ".foo
        ldh  a, [$FF85]";
    let (assembly_line, _) = AssemblyLine::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();
    assert_eq!(
        assembly_line,
        AssemblyLine::Instruction(InstructionLine {
            label: Some(Label::new("foo", Span::from(((1, 1), (1, 5))))),
            instruction: Instruction::Ldh {
                register: Register::new(Intel8080Register::A, Span::from(((2, 14), (2, 15)))),
                label_or_address: LabelOrAddress::Address(Address::new(
                    0xFF85,
                    Span::from(((2, 18), (2, 23)))
                )),
            },
            span: Span::from(((2, 9), (2, 24)))
        })
    );
}

#[derive(Default)]
struct LabelTable(BTreeMap<String, u16>);

impl LabelTable {
    fn insert(&mut self, label: Label, address: u16) {
        self.0.insert(label.name, address);
    }

    fn resolve(&self, label_or_address: LabelOrAddress) -> Result<Address> {
        match label_or_address {
            LabelOrAddress::Label(Label { name, span }) => {
                if let Some(address_value) = self.0.get(&name) {
                    Ok(Address::new(*address_value, span))
                } else {
                    Err(Error::new(format!("no such label: {name:?}"), span))
                }
            }
            LabelOrAddress::Address(address) => Ok(address),
        }
    }
}

impl Instruction {
    fn into_lr35902_instruction(
        self,
        current_address: u16,
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
            Self::And { register } => Ok(LR35902Instruction::LogicalAndWithAccumulator {
                register1: register.value,
            }),
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

fn ldh<Input>() -> impl Parser<Input, Output = Instruction>
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
    let err = assemble(
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

fn and<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    string("and")
        .skip(spaces())
        .with(Register::parser())
        .map(|register| Instruction::And { register })
}

fn jr<Input>() -> impl Parser<Input, Output = Instruction>
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

impl Instruction {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((ldh(), and(), jr()))
    }
}

fn program_parser<Input>() -> impl Parser<Input, Output = Vec<AssemblyLine>>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    optional(spaces()).with(many(AssemblyLine::parser().skip(spaces())))
}

pub fn assemble(input: &str) -> Result<Vec<u8>> {
    let (program, _) = program_parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))?;
    let mut assembled = vec![];
    let mut label_table = LabelTable::default();
    let mut current_address = 0;
    for line in program {
        match line {
            AssemblyLine::Section(section) => {
                if let Some(address) = section.address {
                    current_address = address.value;
                }
            }
            AssemblyLine::Instruction(instr) => {
                let span = instr.span;
                if let Some(label) = instr.label {
                    label_table.insert(label, current_address);
                }
                let instr = instr
                    .instruction
                    .into_lr35902_instruction(current_address, &label_table)?;
                current_address += instr
                    .to_opcode(&mut assembled)
                    .map_err(|e| Error::illegal_instruction(e, span))?
                    as u16;
            }
        }
    }
    Ok(assembled)
}

#[test]
fn small_loop() {
    let bin = assemble(
        "
    SECTION test,ROM0[$036C]
    .loop
        ldh  a, [$FF85]
        and  a
        jr   z, .loop
        ",
    )
    .unwrap();

    #[rustfmt::skip]
    assert_eq!(bin, [
        0xf0, 0x85,
        0xa7,
        0x28, 0xfb,
    ]);
}
