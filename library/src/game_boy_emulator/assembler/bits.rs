// Copyright 2023 Remi Bernotavicius

use super::types::{
    spaces1, Constant, Error, LoadSource, Register, RegisterOrPair, Result, SourcePosition,
};
use super::LabelTable;
use crate::emulator_common::Intel8080Register;
use crate::lr35902_emulator::LR35902Instruction;
use alloc::format;
use combine::parser::char::{char, spaces, string};
use combine::{attempt, choice, optional, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    And {
        register: Register,
    },
    Add {
        destination: Option<RegisterOrPair>,
        source: LoadSource,
    },
    Adc {
        source: LoadSource,
    },
    Xor {
        register: Register,
        constant: Option<Constant<u8>>,
    },
    Inc {
        register: RegisterOrPair,
    },
}

fn and<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(string("and").skip(spaces1()))
        .with(Register::parser())
        .map(|register| Instruction::And { register })
}

fn add<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(string("add").skip(spaces1())).with(choice((
        attempt((
            RegisterOrPair::parser(),
            (spaces(), char(','), spaces()),
            LoadSource::parser(),
        ))
        .map(|(destination, _, source)| Instruction::Add {
            destination: Some(destination),
            source,
        }),
        LoadSource::parser().map(|source| Instruction::Add {
            destination: None,
            source,
        }),
    )))
}

fn adc<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(string("adc").skip(spaces1())).with(choice((
        LoadSource::parser().map(|source| Instruction::Adc { source }),
    )))
}

fn xor<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(string("xor").skip(spaces1())).with(Register::parser()),
        optional(attempt(
            (spaces(), char(','), spaces()).with(Constant::<u8>::parser()),
        )),
    )
        .map(|(register, constant)| Instruction::Xor { register, constant })
}

fn inc<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(string("inc").skip(spaces1()))
        .with(RegisterOrPair::parser())
        .map(|register| Instruction::Inc { register })
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((attempt(and()), attempt(add()), adc(), xor(), inc()))
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
            Self::And { register } => Ok(LR35902Instruction::LogicalAndWithAccumulator {
                register1: register.value,
            }),
            Self::Add {
                destination,
                source,
            } => match destination {
                Some(RegisterOrPair::RegisterPair(pair)) => match pair.value {
                    Intel8080Register::H => {
                        let pair = source.require_register_pair()?;
                        Ok(LR35902Instruction::DoubleAdd {
                            register1: pair.value,
                        })
                    }
                    Intel8080Register::SP => {
                        let constant = source.require_constant_u8()?;
                        Ok(LR35902Instruction::AddImmediateToSp {
                            data1: constant.value,
                        })
                    }
                    v => Err(Error::new(format!("unsupported register {v:?}"), pair.span)),
                },
                Some(RegisterOrPair::Register(reg)) => {
                    reg.require_value(Intel8080Register::A)?;
                    let constant = source.require_constant_u8()?;
                    Ok(LR35902Instruction::AddImmediateToAccumulator {
                        data1: constant.value,
                    })
                }
                None => {
                    let register = source.require_register()?;
                    Ok(LR35902Instruction::AddToAccumulator {
                        register1: register.value,
                    })
                }
            },
            Self::Adc { source } => {
                let source = source.require_register()?;
                Ok(LR35902Instruction::AddToAccumulatorWithCarry {
                    register1: source.value,
                })
            }
            Self::Xor { register, constant } => {
                if let Some(constant) = constant {
                    register.require_value(Intel8080Register::A)?;
                    Ok(LR35902Instruction::ExclusiveOrImmediateWithAccumulator {
                        data1: constant.value,
                    })
                } else {
                    Ok(LR35902Instruction::LogicalExclusiveOrWithAccumulator {
                        register1: register.value,
                    })
                }
            }
            Self::Inc { register } => match register {
                RegisterOrPair::Register(register) => {
                    Ok(LR35902Instruction::IncrementRegisterOrMemory {
                        register1: register.value,
                    })
                }
                RegisterOrPair::RegisterPair(register) => {
                    Ok(LR35902Instruction::IncrementRegisterPair {
                        register1: register.value,
                    })
                }
            },
        }
    }
}
