// Copyright 2023 Remi Bernotavicius

use crate::lr35902_emulator::LR35902Instruction;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, format, vec};
use combine::eof;
use combine::parser::char::spaces;
use combine::stream::position;
use combine::{choice, many, optional, EasyParser as _, Parser};
use section::Section;
use types::{spanned, Address, Error, Label, LabelOrAddress, Result, SourcePosition, Span};

mod bits;
mod jump;
mod load;
mod no_arg;
mod ret;
mod section;
mod types;

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

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Load(load::Instruction),
    Bits(bits::Instruction),
    Jump(jump::Instruction),
    NoArg(no_arg::Instruction),
    Ret(ret::Instruction),
}

impl Instruction {
    fn into_lr35902_instruction(
        self,
        current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Load(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Bits(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Jump(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::NoArg(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Ret(instr) => instr.into_lr35902_instruction(current_address, label_table),
        }
    }
}

impl Instruction {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            load::Instruction::parser().map(Self::Load),
            bits::Instruction::parser().map(Self::Bits),
            jump::Instruction::parser().map(Self::Jump),
            no_arg::Instruction::parser().map(Self::NoArg),
            ret::Instruction::parser().map(Self::Ret),
        ))
    }
}

#[test]
fn instruction_line() {
    use crate::emulator_common::Intel8080Register;
    use types::{LoadDestination, LoadSource, Register};

    let input = ".foo ldh  a, [$FF85]";
    let (instruction_line, _) = InstructionLine::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();
    assert_eq!(
        instruction_line,
        InstructionLine {
            label: Some(Label::new("foo", Span::from(((1, 1), (1, 5))))),
            instruction: Instruction::Load(load::Instruction {
                type_: load::LoadType::Ldh,
                destination: LoadDestination::Register(Register::new(
                    Intel8080Register::A,
                    Span::from(((1, 11), (1, 12)))
                )),
                source: LoadSource::Address(LabelOrAddress::Address(Address {
                    value: 0xFF85,
                    span: Span::from(((1, 15), (1, 20)))
                })),
            }),
            span: Span::from(((1, 6), (1, 21)))
        }
    );
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
            _type_: section::SectionType::Rom0,
            address: Some(Address::new(0xFF34, Span::from(((1, 18), (1, 23))))),
        })
    );
}

#[test]
fn assembly_line_instruction() {
    use crate::emulator_common::Intel8080Register;
    use types::{LoadDestination, LoadSource, Register};

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
            instruction: Instruction::Load(load::Instruction {
                type_: load::LoadType::Ldh,
                destination: LoadDestination::Register(Register::new(
                    Intel8080Register::A,
                    Span::from(((2, 14), (2, 15)))
                )),
                source: LoadSource::Address(LabelOrAddress::Address(Address::new(
                    0xFF85,
                    Span::from(((2, 18), (2, 23)))
                ))),
            }),
            span: Span::from(((2, 9), (2, 24)))
        })
    );
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

#[test]
fn two_functions() {
    let bin = assemble(
        "
    SECTION test,ROM0[$0166]
        ld   a,e
        add  [hl]
        daa
        ldi  [hl],a
        ld   a,d
        adc  [hl]
        daa
        ldi  [hl],a
        ld   a,$00
        adc  [hl]
        daa
        ld   [hl],a
        ld   a,$01
        ldh  [$FFE0],a
        ret  nc
        ld   a,$99
        ldd  [hl],a
        ldd  [hl],a
        ld   [hl],a
        ret
        ",
    )
    .unwrap();

    #[rustfmt::skip]
    assert_eq!(bin, [
        0x7b,
        0x86,
        0x27,
        0x22,
        0x7a,
        0x8e,
        0x27,
        0x22,
        0x3e, 0x00,
        0x8e,
        0x27,
        0x77,
        0x3e, 0x01,
        0xe0, 0xe0,
        0xd0,
        0x3e, 0x99,
        0x32,
        0x32,
        0x77,
        0xc9,
    ]);
}
