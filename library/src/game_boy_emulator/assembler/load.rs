// Copyright 2023 Remi Bernotavicius

use super::types::{
    spaces1, AddressAugend, AddressExpression, AddressSource, Error, LoadDestination, LoadSource,
    Result, SourcePosition,
};
use super::LabelTable;
use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::LR35902Instruction;
use combine::parser::char::{char, spaces, string};
use combine::{attempt, choice, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadType {
    /// Regular load
    Ld,
    /// Load memory
    Ldh,
    /// Load and increment
    Ldi,
    /// Load and decrement
    Ldd,
}

impl LoadType {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("ldh")).map(|_| Self::Ldh),
            attempt(string("ldi")).map(|_| Self::Ldi),
            attempt(string("ldd")).map(|_| Self::Ldd),
            string("ld").map(|_| Self::Ld),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub type_: LoadType,
    pub destination: LoadDestination,
    pub source: LoadSource,
}

impl Instruction {
    pub(crate) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        (
            LoadType::parser().skip(spaces1()),
            LoadDestination::parser(),
            (spaces(), char(','), spaces()),
            LoadSource::parser(),
        )
            .map(|(type_, destination, _, source)| Self {
                type_,
                destination,
                source,
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

#[test]
fn parse_ldh() {
    use super::types::{Address, LabelOrAddress, Register, Span};
    use combine::eof;
    use combine::{stream::position, EasyParser as _};

    let input = "ldh  a, [$FF85]";
    let (instr, _) = Instruction::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();

    assert_eq!(
        instr,
        Instruction {
            type_: LoadType::Ldh,
            destination: LoadDestination::Register(Register::new(
                Intel8080Register::A,
                Span::from(((1, 6), (1, 7)))
            )),
            source: LoadSource::Address(AddressExpression::Identity(AddressSource::Address(
                LabelOrAddress::Address(Address::new(0xFF85, Span::from(((1, 10), (1, 15)))))
            )))
        }
    );
}

#[test]
fn parse_ld() {
    use super::types::{LoadDestination, RegisterPair, Span};
    use combine::eof;
    use combine::{stream::position, EasyParser as _};

    let input = "ld  sp,hl";
    let (instr, _) = Instruction::parser()
        .skip(eof())
        .easy_parse(position::Stream::new(input))
        .unwrap();

    assert_eq!(
        instr,
        Instruction {
            type_: LoadType::Ld,
            destination: LoadDestination::RegisterPair(RegisterPair::new(
                Intel8080Register::SP,
                Span::from(((1, 5), (1, 7)))
            )),
            source: LoadSource::RegisterPair(RegisterPair::new(
                Intel8080Register::H,
                Span::from(((1, 8), (1, 10)))
            )),
        }
    );
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self.type_ {
            LoadType::Ldh => match (self.destination, self.source) {
                (
                    LoadDestination::Register(destination),
                    LoadSource::Address(AddressExpression::Identity(AddressSource::Address(
                        source,
                    ))),
                ) => {
                    destination.require_value(Intel8080Register::A)?;
                    let data1 = label_table.resolve(source)?.shorten()?;
                    Ok(LR35902Instruction::LoadAccumulatorDirectOneByte { data1 })
                }
                (
                    LoadDestination::Address(AddressExpression::Identity(AddressSource::Address(
                        destination,
                    ))),
                    LoadSource::Register(source),
                ) => {
                    source.require_value(Intel8080Register::A)?;
                    let data1 = label_table.resolve(destination)?.shorten()?;
                    Ok(LR35902Instruction::StoreAccumulatorDirectOneByte { data1 })
                }
                (destination, source) => Err(Error::new(
                    "invalid arguments destination: {destination:?}, source: {source:?}",
                    destination.into_span() + source.into_span(),
                )),
            },
            LoadType::Ld => match (self.destination, self.source) {
                (LoadDestination::Register(destination), LoadSource::Register(source)) => {
                    Ok(LR35902Instruction::MoveData {
                        register1: destination.value,
                        register2: source.value,
                    })
                }
                (
                    LoadDestination::Register(destination),
                    LoadSource::Address(AddressExpression::Identity(AddressSource::Address(
                        source,
                    ))),
                ) => {
                    destination.require_value(Intel8080Register::A)?;
                    let source = label_table.resolve(source)?;
                    Ok(LR35902Instruction::LoadAccumulatorDirect {
                        address1: source.value,
                    })
                }
                (
                    LoadDestination::Register(destination),
                    LoadSource::Address(AddressExpression::Identity(AddressSource::RegisterPair(
                        source,
                    ))),
                ) => {
                    destination.require_value(Intel8080Register::A)?;
                    Ok(LR35902Instruction::LoadAccumulator {
                        register1: source.value,
                    })
                }
                (
                    LoadDestination::Register(destination),
                    LoadSource::Address(AddressExpression::Addition(
                        AddressSource::Address(source),
                        AddressAugend::Register(augend),
                    )),
                ) => {
                    destination.require_value(Intel8080Register::A)?;
                    let source = label_table.resolve(source)?;
                    source.require_value(0xFF00)?;
                    augend.require_value(Intel8080Register::C)?;
                    Ok(LR35902Instruction::LoadAccumulatorOneByte)
                }
                (LoadDestination::Register(destination), LoadSource::Constant(source)) => {
                    Ok(LR35902Instruction::MoveImmediateData {
                        register1: destination.value,
                        data2: source.require_u8()?,
                    })
                }
                (LoadDestination::RegisterPair(destination), LoadSource::Constant(source)) => {
                    Ok(LR35902Instruction::LoadRegisterPairImmediate {
                        register1: destination.value,
                        data2: source.require_u16()?,
                    })
                }
                (
                    LoadDestination::Address(AddressExpression::Identity(AddressSource::Address(
                        destination,
                    ))),
                    LoadSource::Register(source),
                ) => {
                    source.require_value(Intel8080Register::A)?;
                    let destination = label_table.resolve(destination)?;
                    Ok(LR35902Instruction::StoreAccumulatorDirect {
                        address1: destination.value,
                    })
                }
                (
                    LoadDestination::Address(AddressExpression::Identity(
                        AddressSource::RegisterPair(destination),
                    )),
                    LoadSource::Register(source),
                ) => {
                    source.require_value(Intel8080Register::A)?;
                    Ok(LR35902Instruction::StoreAccumulator {
                        register1: destination.value,
                    })
                }
                (
                    LoadDestination::Address(AddressExpression::Addition(
                        AddressSource::Address(destination),
                        AddressAugend::Register(augend),
                    )),
                    LoadSource::Register(source),
                ) => {
                    source.require_value(Intel8080Register::A)?;
                    let destination = label_table.resolve(destination)?;
                    destination.require_value(0xFF00)?;
                    augend.require_value(Intel8080Register::C)?;
                    Ok(LR35902Instruction::StoreAccumulatorOneByte)
                }
                (
                    LoadDestination::Address(AddressExpression::Identity(AddressSource::Address(
                        destination,
                    ))),
                    LoadSource::RegisterPair(source),
                ) => {
                    source.require_value(Intel8080Register::SP)?;
                    let destination = label_table.resolve(destination)?;
                    Ok(LR35902Instruction::StoreSpDirect {
                        address1: destination.value,
                    })
                }
                (LoadDestination::RegisterPair(destination), LoadSource::RegisterPair(source)) => {
                    destination.require_value(Intel8080Register::SP)?;
                    source.require_value(Intel8080Register::H)?;
                    Ok(LR35902Instruction::LoadSpFromHAndL)
                }
                (
                    LoadDestination::RegisterPair(destination),
                    LoadSource::Address(AddressExpression::Addition(
                        AddressSource::RegisterPair(base),
                        AddressAugend::Constant(augend),
                    )),
                ) => {
                    destination.require_value(Intel8080Register::H)?;
                    base.require_value(Intel8080Register::SP)?;
                    Ok(LR35902Instruction::StoreSpPlusImmediate {
                        data1: augend.require_u8()?,
                    })
                }
                (destination, source) => Err(Error::new(
                    "invalid arguments destination: {destination:?}, source: {source:?}",
                    destination.into_span() + source.into_span(),
                )),
            },
            LoadType::Ldi => {
                let destination = self.destination.require_register()?;
                let source = self.source.require_register()?;
                Ok(LR35902Instruction::MoveAndIncrementHl {
                    register1: destination.value,
                    register2: source.value,
                })
            }
            LoadType::Ldd => {
                let destination = self.destination.require_register()?;
                let source = self.source.require_register()?;
                Ok(LR35902Instruction::MoveAndDecrementHl {
                    register1: destination.value,
                    register2: source.value,
                })
            }
        }
    }
}
