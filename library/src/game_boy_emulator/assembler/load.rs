// Copyright 2023 Remi Bernotavicius

use super::types::{spaces1, LoadDestination, LoadSource, Result, SourcePosition};
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
            source: LoadSource::Address(LabelOrAddress::Address(Address::new(
                0xFF85,
                Span::from(((1, 10), (1, 15)))
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
                (LoadDestination::Register(destination), LoadSource::Address(source)) => {
                    destination.require_value(Intel8080Register::A)?;
                    let data1 = label_table.resolve(source)?.shorten()?;
                    Ok(LR35902Instruction::LoadAccumulatorDirectOneByte { data1 })
                }
                (LoadDestination::Address(destination), LoadSource::Register(source)) => {
                    source.require_value(Intel8080Register::A)?;
                    let data1 = label_table.resolve(destination)?.shorten()?;
                    Ok(LR35902Instruction::StoreAccumulatorDirectOneByte { data1 })
                }
                v => unimplemented!("{v:?}"),
            },
            LoadType::Ld => {
                match (self.destination, self.source) {
                    (LoadDestination::Register(destination), LoadSource::Register(source)) => {
                        Ok(LR35902Instruction::MoveData {
                            register1: destination.value,
                            register2: source.value,
                        })
                    }
                    (LoadDestination::Register(destination), LoadSource::Address(address)) => {
                        destination.require_value(Intel8080Register::A)?;
                        let address = label_table.resolve(address)?;
                        Ok(LR35902Instruction::LoadAccumulatorDirect {
                            address1: address.value,
                        })
                    }
                    (LoadDestination::Register(register), LoadSource::ConstantU8(constant)) => {
                        Ok(LR35902Instruction::MoveImmediateData {
                            register1: register.value,
                            data2: constant.value,
                        })
                    }
                    (LoadDestination::RegisterPair(pair), LoadSource::ConstantU16(constant)) => {
                        Ok(LR35902Instruction::LoadRegisterPairImmediate {
                            register1: pair.value,
                            data2: constant.value,
                        })
                    }
                    (LoadDestination::Address(address), LoadSource::Register(register)) => {
                        register.require_value(Intel8080Register::A)?;
                        let address = label_table.resolve(address)?;
                        Ok(LR35902Instruction::StoreAccumulatorDirect {
                            address1: address.value,
                        })
                    }
                    (LoadDestination::Address(address), LoadSource::RegisterPair(pair)) => {
                        pair.require_value(Intel8080Register::SP)?;
                        let address = label_table.resolve(address)?;
                        Ok(LR35902Instruction::StoreSpDirect {
                            address1: address.value,
                        })
                    }
                    (LoadDestination::RegisterPair(pair1), LoadSource::RegisterPair(pair2)) => {
                        pair1.require_value(Intel8080Register::SP)?;
                        pair2.require_value(Intel8080Register::H)?;
                        Ok(LR35902Instruction::LoadSpFromHAndL)
                    }
                    v => unimplemented!("{v:?}"),
                }
                // load_accumulator ld a,[<rp>]
                // load_accumulator_one_byte ld a,[$FF00+c]
                // store_accumulator_one_byte ld [$FF00+c],a
                // store_accumulator ld [<rp>],a
                // store_sp_plus_immediate ld hl,[sp+$XXXX]
            }
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
