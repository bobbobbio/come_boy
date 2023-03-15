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
pub enum SimpleKeyword {
    Adc,
    And,
    Or,
    Sbc,
    Sub,
    Xor,
}

impl SimpleKeyword {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("adc")).map(|_| Self::Adc),
            attempt(string("and")).map(|_| Self::And),
            attempt(string("or")).map(|_| Self::Or),
            attempt(string("sbc")).map(|_| Self::Sbc),
            attempt(string("sub")).map(|_| Self::Sub),
            attempt(string("xor")).map(|_| Self::Xor),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add {
        destination: Option<RegisterOrPair>,
        source: LoadSource,
    },
    Dec {
        register: RegisterOrPair,
    },
    Inc {
        register: RegisterOrPair,
    },
    Simple {
        keyword: SimpleKeyword,
        register: Register,
        source: Option<Constant<u8>>,
    },
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

fn dec<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(string("dec").skip(spaces1())),
        RegisterOrPair::parser(),
    )
        .map(|(_, register)| Instruction::Dec { register })
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

fn simple<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(SimpleKeyword::parser().skip(spaces1())),
        Register::parser(),
        optional(attempt(
            (spaces(), char(','), spaces()).with(Constant::<u8>::parser()),
        )),
    )
        .map(|(keyword, register, source)| Instruction::Simple {
            keyword,
            register,
            source,
        })
}

impl Instruction {
    pub(super) fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((add(), dec(), inc(), simple()))
    }
}

impl Instruction {
    pub(super) fn into_lr35902_instruction(
        self,
        _current_address: u16,
        _label_table: &LabelTable,
    ) -> Result<LR35902Instruction> {
        match self {
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
            Self::Dec { register } => match register {
                RegisterOrPair::Register(register) => {
                    Ok(LR35902Instruction::DecrementRegisterOrMemory {
                        register1: register.value,
                    })
                }
                RegisterOrPair::RegisterPair(register) => {
                    Ok(LR35902Instruction::DecrementRegisterPair {
                        register1: register.value,
                    })
                }
            },
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
            Self::Simple {
                keyword,
                register,
                source,
            } => match keyword {
                SimpleKeyword::And => {
                    if let Some(constant) = source {
                        register.require_value(Intel8080Register::A)?;
                        Ok(LR35902Instruction::AndImmediateWithAccumulator {
                            data1: constant.value,
                        })
                    } else {
                        Ok(LR35902Instruction::LogicalAndWithAccumulator {
                            register1: register.value,
                        })
                    }
                }
                SimpleKeyword::Adc => {
                    if let Some(constant) = source {
                        register.require_value(Intel8080Register::A)?;
                        Ok(LR35902Instruction::AddImmediateToAccumulatorWithCarry {
                            data1: constant.value,
                        })
                    } else {
                        Ok(LR35902Instruction::AddToAccumulatorWithCarry {
                            register1: register.value,
                        })
                    }
                }
                SimpleKeyword::Sub => {
                    if let Some(constant) = source {
                        register.require_value(Intel8080Register::A)?;
                        Ok(LR35902Instruction::SubtractImmediateFromAccumulator {
                            data1: constant.value,
                        })
                    } else {
                        Ok(LR35902Instruction::SubtractFromAccumulator {
                            register1: register.value,
                        })
                    }
                }
                SimpleKeyword::Sbc => {
                    if let Some(constant) = source {
                        register.require_value(Intel8080Register::A)?;
                        Ok(
                            LR35902Instruction::SubtractImmediateFromAccumulatorWithBorrow {
                                data1: constant.value,
                            },
                        )
                    } else {
                        Ok(LR35902Instruction::SubtractFromAccumulatorWithBorrow {
                            register1: register.value,
                        })
                    }
                }
                SimpleKeyword::Or => {
                    if let Some(constant) = source {
                        register.require_value(Intel8080Register::A)?;
                        Ok(LR35902Instruction::OrImmediateWithAccumulator {
                            data1: constant.value,
                        })
                    } else {
                        Ok(LR35902Instruction::LogicalOrWithAccumulator {
                            register1: register.value,
                        })
                    }
                }
                SimpleKeyword::Xor => {
                    if let Some(constant) = source {
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
            },
        }
    }
}
