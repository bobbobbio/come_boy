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
pub enum NoArgKeyword {
    Ccf,
    Cpl,
    Daa,
    Rla,
    Rlca,
    Rra,
    Rrca,
    Scf,
}

impl NoArgKeyword {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("ccf")).map(|_| Self::Ccf),
            attempt(string("cpl")).map(|_| Self::Cpl),
            attempt(string("daa")).map(|_| Self::Daa),
            attempt(string("rlca")).map(|_| Self::Rlca),
            attempt(string("rra")).map(|_| Self::Rra),
            attempt(string("rrca")).map(|_| Self::Rrca),
            attempt(string("rla")).map(|_| Self::Rla),
            attempt(string("scf")).map(|_| Self::Scf),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SingleArgKeyword {
    Dec,
    Inc,
    Rcr,
    Rl,
    Rlc,
    Rr,
    Sla,
    Sra,
    Srl,
    Swap,
}

impl SingleArgKeyword {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: combine::Stream<Token = char>,
        Input::Position: Into<SourcePosition>,
    {
        choice((
            attempt(string("dec")).map(|_| Self::Dec),
            attempt(string("inc")).map(|_| Self::Inc),
            attempt(string("rcr")).map(|_| Self::Rcr),
            attempt(string("rl")).map(|_| Self::Rl),
            attempt(string("rlc")).map(|_| Self::Rlc),
            attempt(string("rr")).map(|_| Self::Rr),
            attempt(string("sla")).map(|_| Self::Sla),
            attempt(string("sra")).map(|_| Self::Sra),
            attempt(string("srl")).map(|_| Self::Srl),
            attempt(string("swap")).map(|_| Self::Swap),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TwoArgsKeyword {
    Adc,
    And,
    Or,
    Sbc,
    Sub,
    Xor,
}

impl TwoArgsKeyword {
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
    NoArg {
        keyword: NoArgKeyword,
    },
    SingleArg {
        keyword: SingleArgKeyword,
        register: RegisterOrPair,
    },
    TwoArgs {
        keyword: TwoArgsKeyword,
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

fn no_arg<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    attempt(NoArgKeyword::parser()).map(|keyword| Instruction::NoArg { keyword })
}

fn single_arg<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(SingleArgKeyword::parser().skip(spaces1())),
        RegisterOrPair::parser(),
    )
        .map(|(keyword, register)| Instruction::SingleArg { keyword, register })
}

fn two_args<Input>() -> impl Parser<Input, Output = Instruction>
where
    Input: combine::Stream<Token = char>,
    Input::Position: Into<SourcePosition>,
{
    (
        attempt(TwoArgsKeyword::parser().skip(spaces1())),
        Register::parser(),
        optional(attempt(
            (spaces(), char(','), spaces()).with(Constant::<u8>::parser()),
        )),
    )
        .map(|(keyword, register, source)| Instruction::TwoArgs {
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
        choice((add(), no_arg(), single_arg(), two_args()))
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
            Self::NoArg { keyword } => match keyword {
                NoArgKeyword::Ccf => Ok(LR35902Instruction::ComplementCarry),
                NoArgKeyword::Cpl => Ok(LR35902Instruction::ComplementAccumulator),
                NoArgKeyword::Daa => Ok(LR35902Instruction::DecimalAdjustAccumulator),
                NoArgKeyword::Rla => Ok(LR35902Instruction::RotateAccumulatorLeftThroughCarry),
                NoArgKeyword::Rlca => Ok(LR35902Instruction::RotateAccumulatorLeft),
                NoArgKeyword::Rra => Ok(LR35902Instruction::RotateAccumulatorRightThroughCarry),
                NoArgKeyword::Rrca => Ok(LR35902Instruction::RotateAccumulatorRight),
                NoArgKeyword::Scf => Ok(LR35902Instruction::SetCarry),
            },
            Self::SingleArg { keyword, register } => match keyword {
                SingleArgKeyword::Dec => match register {
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
                SingleArgKeyword::Inc => match register {
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
                SingleArgKeyword::Rcr => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::RotateRegisterRightThroughCarry {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Rl => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::RotateRegisterLeft {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Rlc => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::RotateRegisterLeftThroughCarry {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Rr => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::RotateRegisterRight {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Sla => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::ShiftRegisterLeft {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Sra => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::ShiftRegisterRightSigned {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Srl => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::ShiftRegisterRight {
                        register1: register.value,
                    })
                }
                SingleArgKeyword::Swap => {
                    let register = register.require_register()?;
                    Ok(LR35902Instruction::SwapRegister {
                        register1: register.value,
                    })
                }
            },
            Self::TwoArgs {
                keyword,
                register,
                source,
            } => match keyword {
                TwoArgsKeyword::And => {
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
                TwoArgsKeyword::Adc => {
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
                TwoArgsKeyword::Sub => {
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
                TwoArgsKeyword::Sbc => {
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
                TwoArgsKeyword::Or => {
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
                TwoArgsKeyword::Xor => {
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
