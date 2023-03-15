// Copyright 2023 Remi Bernotavicius

use crate::lr35902_emulator::LR35902Instruction;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, format, vec};
use combine::eof;
use combine::parser::char::{char, spaces};
use combine::stream::position;
use combine::{attempt, choice, many, optional, satisfy, EasyParser as _, Parser};
use section::Section;
use types::{spanned, Address, Error, Label, LabelOrAddress, Result, SourcePosition, Span};

mod bits;
mod call;
mod compare;
mod jump;
mod load;
mod no_arg;
mod ret;
mod section;
mod stack;
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
    Bits(bits::Instruction),
    Call(call::Instruction),
    Compare(compare::Instruction),
    Jump(jump::Instruction),
    Load(load::Instruction),
    NoArg(no_arg::Instruction),
    Ret(ret::Instruction),
    Stack(stack::Instruction),
}

impl Instruction {
    fn into_lr35902_instruction(
        self,
        current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::Bits(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Call(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Compare(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Jump(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Load(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::NoArg(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Ret(instr) => instr.into_lr35902_instruction(current_address, label_table),
            Self::Stack(instr) => instr.into_lr35902_instruction(current_address, label_table),
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
            bits::Instruction::parser().map(Self::Bits),
            call::Instruction::parser().map(Self::Call),
            compare::Instruction::parser().map(Self::Compare),
            jump::Instruction::parser().map(Self::Jump),
            load::Instruction::parser().map(Self::Load),
            no_arg::Instruction::parser().map(Self::NoArg),
            ret::Instruction::parser().map(Self::Ret),
            stack::Instruction::parser().map(Self::Stack),
        ))
    }
}

#[test]
fn instruction_line() {
    use crate::emulator_common::Intel8080Register;
    use types::{AddressExpression, AddressSource, LoadDestination, LoadSource, Register};

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
                source: LoadSource::Address(AddressExpression::Identity(AddressSource::Address(
                    LabelOrAddress::Address(Address {
                        value: 0xFF85,
                        span: Span::from(((1, 15), (1, 20)))
                    })
                ))),
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
        let comment = attempt((
            spaces(),
            char(';'),
            many::<String, _, _>(satisfy(|c| c != '\n')),
        ));
        choice((
            Section::parser().map(Self::Section),
            InstructionLine::parser().map(Self::Instruction),
        ))
        .skip(optional(comment))
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
    use types::{AddressExpression, AddressSource, LoadDestination, LoadSource, Register};

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
                source: LoadSource::Address(AddressExpression::Identity(AddressSource::Address(
                    LabelOrAddress::Address(Address::new(0xFF85, Span::from(((2, 18), (2, 23)))))
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

#[test]
fn another_function() {
    let bin = assemble(
        "
    SECTION test,ROM0[$025a]
        ld   a,[$C0CE]
        and  a
        jr   z,$027A
        ldh  a,[$FF98]
        cp   a,$03
        jr   nz,$027A
        ld   hl,$986D
        call $249B
        ld   a,$01
        ldh  [$FFE0],a
        ld   hl,$9C6D
        call $249B
        xor  a
        ld   [$C0CE],a
        ld   hl,$FFE2
        inc  [hl]
        xor  a
        ldh  [$FF43],a ; SCX
        ldh  [$FF42],a ; SCY
        inc  a
        ldh  [$FF85],a
        pop  hl
        pop  de
        pop  bc
        pop  af
        reti
        ",
    )
    .unwrap();

    #[rustfmt::skip]
    assert_eq!(bin, [
        0xfa, 0xce, 0xc0,
        0xa7,
        0x28, 0x1a,
        0xf0, 0x98,
        0xfe, 0x03,
        0x20, 0x14,
        0x21, 0x6d, 0x98,
        0xcd, 0x9b, 0x24,
        0x3e, 0x01,
        0xe0, 0xe0,
        0x21, 0x6d, 0x9c,
        0xcd, 0x9b, 0x24,
        0xaf,
        0xea, 0xce, 0xc0,
        0x21, 0xe2, 0xff,
        0x34,
        0xaf,
        0xe0, 0x43,
        0xe0, 0x42,
        0x3c,
        0xe0, 0x85,
        0xe1,
        0xd1,
        0xc1,
        0xf1,
        0xd9,
    ]);
}

#[test]
fn yet_another_function() {
    let bin = assemble(
        "
    SECTION test,ROM0[$3085]
    ldh  [$FFE1],a
    rst  $38
    ld   hl,$DE32
    rst  $18
    ldh  [$FFE1],a
    rst  $38
    ld   hl,$DE32
    ld   [$FF00+c],a
    ldh  [$FFE4],a
    rst  $38
    ld   hl,$DC32
    xor  a,$E0
        ",
    )
    .unwrap();

    #[rustfmt::skip]
    assert_eq!(bin, [
        0xe0, 0xe1,
        0xff,
        0x21, 0x32, 0xde,
        0xdf,
        0xe0, 0xe1,
        0xff,
        0x21, 0x32, 0xde,
        0xe2,
        0xe0, 0xe4,
        0xff,
        0x21, 0x32, 0xdc,
        0xee, 0xe0,
    ]);
}

#[test]
fn yet_another_another_function() {
    let bin = assemble(
        "
    SECTION test,ROM0[$0383]
        dec  e
        add  c
        dec  e
        add  hl,de
        inc  b
        and  a,$04
        xor  b
        inc  d
        ldh  a,[$FF14] ; Channel 1 Frequency (high)
        ld   l,e
        ld   a,[de]
        dec  de
        ld   e,$71
        rra
        ld   a,d
        rra
        adc  c
        dec  d
        inc  d
        dec  d
        rst  $18
        dec  d
        inc  hl
        ld   d,$8D
        ld   d,$DE
        ld   d,$4F
        rla
        ld   [hl],a
        add  hl,de
        ",
    )
    .unwrap();

    #[rustfmt::skip]
    assert_eq!(bin, [
        0x1d,
        0x81,
        0x1d,
        0x19,
        0x04,
        0xe6, 0x04,
        0xa8,
        0x14,
        0xf0, 0x14,
        0x6b,
        0x1a,
        0x1b,
        0x1e, 0x71,
        0x1f,
        0x7a,
        0x1f,
        0x89,
        0x15,
        0x14,
        0x15,
        0xdf,
        0x15,
        0x23,
        0x16, 0x8d,
        0x16, 0xde,
        0x16, 0x4f,
        0x17,
        0x77,
        0x19,
    ]);
}
