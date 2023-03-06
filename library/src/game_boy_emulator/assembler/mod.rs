// Copyright 2023 Remi Bernotavicius

use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::LR35902Instruction;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, format, vec};
use combine::eof;
use combine::parser::char::spaces;
use combine::stream::position;
use combine::{choice, many, optional, EasyParser as _, Parser};
use jump::Condition;
use section::Section;
use types::{
    spanned, Address, Error, Label, LabelOrAddress, Register, Result, SourcePosition, Span,
};

mod bits;
mod jump;
mod load;
mod section;
mod types;

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

impl Instruction {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((load::ldh(), bits::and(), jump::jr()))
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
