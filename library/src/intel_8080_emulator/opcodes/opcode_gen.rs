#![allow(dead_code)]
use crate::bytes::{LittleEndian, ReadBytesExt};
use crate::emulator_common::Intel8080Register;
use crate::intel_8080_emulator::opcodes::Intel8080InstructionPrinter;
use crate::io;
use serde_derive::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intel8080Instruction {
    AddImmediateToAccumulator {
        data1: u8,
    },
    AddImmediateToAccumulatorWithCarry {
        data1: u8,
    },
    AddToAccumulator {
        register1: Intel8080Register,
    },
    AddToAccumulatorWithCarry {
        register1: Intel8080Register,
    },
    AndImmediateWithAccumulator {
        data1: u8,
    },
    Call {
        address1: u16,
    },
    CallIfCarry {
        address1: u16,
    },
    CallIfMinus {
        address1: u16,
    },
    CallIfNoCarry {
        address1: u16,
    },
    CallIfNotZero {
        address1: u16,
    },
    CallIfParityEven {
        address1: u16,
    },
    CallIfParityOdd {
        address1: u16,
    },
    CallIfPlus {
        address1: u16,
    },
    CallIfZero {
        address1: u16,
    },
    CompareImmediateWithAccumulator {
        data1: u8,
    },
    CompareWithAccumulator {
        register1: Intel8080Register,
    },
    ComplementAccumulator,
    ComplementCarry,
    DecimalAdjustAccumulator,
    DecrementRegisterOrMemory {
        register1: Intel8080Register,
    },
    DecrementRegisterPair {
        register1: Intel8080Register,
    },
    DisableInterrupts,
    DoubleAdd {
        register1: Intel8080Register,
    },
    EnableInterrupts,
    ExchangeRegisters,
    ExchangeStack,
    ExclusiveOrImmediateWithAccumulator {
        data1: u8,
    },
    Halt,
    IncrementRegisterOrMemory {
        register1: Intel8080Register,
    },
    IncrementRegisterPair {
        register1: Intel8080Register,
    },
    Input {
        data1: u8,
    },
    Jump {
        address1: u16,
    },
    JumpIfCarry {
        address1: u16,
    },
    JumpIfMinus {
        address1: u16,
    },
    JumpIfNoCarry {
        address1: u16,
    },
    JumpIfNotZero {
        address1: u16,
    },
    JumpIfParityEven {
        address1: u16,
    },
    JumpIfParityOdd {
        address1: u16,
    },
    JumpIfPositive {
        address1: u16,
    },
    JumpIfZero {
        address1: u16,
    },
    LoadAccumulator {
        register1: Intel8080Register,
    },
    LoadAccumulatorDirect {
        address1: u16,
    },
    LoadHAndLDirect {
        address1: u16,
    },
    LoadProgramCounter,
    LoadRegisterPairImmediate {
        register1: Intel8080Register,
        data2: u16,
    },
    LoadSpFromHAndL,
    LogicalAndWithAccumulator {
        register1: Intel8080Register,
    },
    LogicalExclusiveOrWithAccumulator {
        register1: Intel8080Register,
    },
    LogicalOrWithAccumulator {
        register1: Intel8080Register,
    },
    MoveData {
        register1: Intel8080Register,
        register2: Intel8080Register,
    },
    MoveImmediateData {
        register1: Intel8080Register,
        data2: u8,
    },
    NoOperation,
    OrImmediateWithAccumulator {
        data1: u8,
    },
    Output {
        data1: u8,
    },
    PopDataOffStack {
        register1: Intel8080Register,
    },
    PushDataOntoStack {
        register1: Intel8080Register,
    },
    Restart {
        data1: u8,
    },
    ReturnIfCarry,
    ReturnIfMinus,
    ReturnIfNoCarry,
    ReturnIfNotZero,
    ReturnIfParityEven,
    ReturnIfParityOdd,
    ReturnIfPlus,
    ReturnIfZero,
    ReturnUnconditionally,
    Rim,
    RotateAccumulatorLeft,
    RotateAccumulatorLeftThroughCarry,
    RotateAccumulatorRight,
    RotateAccumulatorRightThroughCarry,
    SetCarry,
    Sim,
    StoreAccumulator {
        register1: Intel8080Register,
    },
    StoreAccumulatorDirect {
        address1: u16,
    },
    StoreHAndLDirect {
        address1: u16,
    },
    SubtractFromAccumulator {
        register1: Intel8080Register,
    },
    SubtractFromAccumulatorWithBorrow {
        register1: Intel8080Register,
    },
    SubtractImmediateFromAccumulator {
        data1: u8,
    },
    SubtractImmediateFromAccumulatorWithBorrow {
        data1: u8,
    },
}
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, enum_iterator :: IntoEnumIterator,
)]
pub enum Intel8080InstructionType {
    AddImmediateToAccumulator = 0isize,
    AddImmediateToAccumulatorWithCarry = 1isize,
    AddToAccumulator = 2isize,
    AddToAccumulatorWithCarry = 3isize,
    AndImmediateWithAccumulator = 4isize,
    Call = 5isize,
    CallIfCarry = 6isize,
    CallIfMinus = 7isize,
    CallIfNoCarry = 8isize,
    CallIfNotZero = 9isize,
    CallIfParityEven = 10isize,
    CallIfParityOdd = 11isize,
    CallIfPlus = 12isize,
    CallIfZero = 13isize,
    CompareImmediateWithAccumulator = 14isize,
    CompareWithAccumulator = 15isize,
    ComplementAccumulator = 16isize,
    ComplementCarry = 17isize,
    DecimalAdjustAccumulator = 18isize,
    DecrementRegisterOrMemory = 19isize,
    DecrementRegisterPair = 20isize,
    DisableInterrupts = 21isize,
    DoubleAdd = 22isize,
    EnableInterrupts = 23isize,
    ExchangeRegisters = 24isize,
    ExchangeStack = 25isize,
    ExclusiveOrImmediateWithAccumulator = 26isize,
    Halt = 27isize,
    IncrementRegisterOrMemory = 28isize,
    IncrementRegisterPair = 29isize,
    Input = 30isize,
    Jump = 31isize,
    JumpIfCarry = 32isize,
    JumpIfMinus = 33isize,
    JumpIfNoCarry = 34isize,
    JumpIfNotZero = 35isize,
    JumpIfParityEven = 36isize,
    JumpIfParityOdd = 37isize,
    JumpIfPositive = 38isize,
    JumpIfZero = 39isize,
    LoadAccumulator = 40isize,
    LoadAccumulatorDirect = 41isize,
    LoadHAndLDirect = 42isize,
    LoadProgramCounter = 43isize,
    LoadRegisterPairImmediate = 44isize,
    LoadSpFromHAndL = 45isize,
    LogicalAndWithAccumulator = 46isize,
    LogicalExclusiveOrWithAccumulator = 47isize,
    LogicalOrWithAccumulator = 48isize,
    MoveData = 49isize,
    MoveImmediateData = 50isize,
    NoOperation = 51isize,
    OrImmediateWithAccumulator = 52isize,
    Output = 53isize,
    PopDataOffStack = 54isize,
    PushDataOntoStack = 55isize,
    Restart = 56isize,
    ReturnIfCarry = 57isize,
    ReturnIfMinus = 58isize,
    ReturnIfNoCarry = 59isize,
    ReturnIfNotZero = 60isize,
    ReturnIfParityEven = 61isize,
    ReturnIfParityOdd = 62isize,
    ReturnIfPlus = 63isize,
    ReturnIfZero = 64isize,
    ReturnUnconditionally = 65isize,
    Rim = 66isize,
    RotateAccumulatorLeft = 67isize,
    RotateAccumulatorLeftThroughCarry = 68isize,
    RotateAccumulatorRight = 69isize,
    RotateAccumulatorRightThroughCarry = 70isize,
    SetCarry = 71isize,
    Sim = 72isize,
    StoreAccumulator = 73isize,
    StoreAccumulatorDirect = 74isize,
    StoreHAndLDirect = 75isize,
    SubtractFromAccumulator = 76isize,
    SubtractFromAccumulatorWithBorrow = 77isize,
    SubtractImmediateFromAccumulator = 78isize,
    SubtractImmediateFromAccumulatorWithBorrow = 79isize,
}
const NUM_INSTRUCTIONS: usize = 80usize;
impl Intel8080Instruction {
    #[allow(clippy::unnecessary_cast)]
    #[inline(always)]
    pub fn from_reader<R: io::Read>(mut stream: R) -> io::Result<Option<Self>> {
        let opcode = stream.read_u8()?;
        Ok(match opcode {
            0x00 => Some(Self::NoOperation),
            0x01 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::B,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x02 => Some(Self::StoreAccumulator {
                register1: Intel8080Register::B,
            }),
            0x03 => Some(Self::IncrementRegisterPair {
                register1: Intel8080Register::B,
            }),
            0x04 => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::B,
            }),
            0x05 => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::B,
            }),
            0x06 => Some(Self::MoveImmediateData {
                register1: Intel8080Register::B,
                data2: stream.read_u8().unwrap(),
            }),
            0x07 => Some(Self::RotateAccumulatorLeft),
            0x09 => Some(Self::DoubleAdd {
                register1: Intel8080Register::B,
            }),
            0x0A => Some(Self::LoadAccumulator {
                register1: Intel8080Register::B,
            }),
            0x0B => Some(Self::DecrementRegisterPair {
                register1: Intel8080Register::B,
            }),
            0x0C => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::C,
            }),
            0x0D => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::C,
            }),
            0x0E => Some(Self::MoveImmediateData {
                register1: Intel8080Register::C,
                data2: stream.read_u8().unwrap(),
            }),
            0x0F => Some(Self::RotateAccumulatorRight),
            0x11 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::D,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x12 => Some(Self::StoreAccumulator {
                register1: Intel8080Register::D,
            }),
            0x13 => Some(Self::IncrementRegisterPair {
                register1: Intel8080Register::D,
            }),
            0x14 => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::D,
            }),
            0x15 => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::D,
            }),
            0x16 => Some(Self::MoveImmediateData {
                register1: Intel8080Register::D,
                data2: stream.read_u8().unwrap(),
            }),
            0x17 => Some(Self::RotateAccumulatorLeftThroughCarry),
            0x19 => Some(Self::DoubleAdd {
                register1: Intel8080Register::D,
            }),
            0x1A => Some(Self::LoadAccumulator {
                register1: Intel8080Register::D,
            }),
            0x1B => Some(Self::DecrementRegisterPair {
                register1: Intel8080Register::D,
            }),
            0x1C => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::E,
            }),
            0x1D => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::E,
            }),
            0x1E => Some(Self::MoveImmediateData {
                register1: Intel8080Register::E,
                data2: stream.read_u8().unwrap(),
            }),
            0x1F => Some(Self::RotateAccumulatorRightThroughCarry),
            0x20 => Some(Self::Rim),
            0x21 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::H,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x22 => Some(Self::StoreHAndLDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x23 => Some(Self::IncrementRegisterPair {
                register1: Intel8080Register::H,
            }),
            0x24 => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::H,
            }),
            0x25 => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::H,
            }),
            0x26 => Some(Self::MoveImmediateData {
                register1: Intel8080Register::H,
                data2: stream.read_u8().unwrap(),
            }),
            0x27 => Some(Self::DecimalAdjustAccumulator),
            0x29 => Some(Self::DoubleAdd {
                register1: Intel8080Register::H,
            }),
            0x2A => Some(Self::LoadHAndLDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x2B => Some(Self::DecrementRegisterPair {
                register1: Intel8080Register::H,
            }),
            0x2C => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::L,
            }),
            0x2D => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::L,
            }),
            0x2E => Some(Self::MoveImmediateData {
                register1: Intel8080Register::L,
                data2: stream.read_u8().unwrap(),
            }),
            0x2F => Some(Self::ComplementAccumulator),
            0x30 => Some(Self::Sim),
            0x31 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::SP,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x32 => Some(Self::StoreAccumulatorDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x33 => Some(Self::IncrementRegisterPair {
                register1: Intel8080Register::SP,
            }),
            0x34 => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::M,
            }),
            0x35 => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::M,
            }),
            0x36 => Some(Self::MoveImmediateData {
                register1: Intel8080Register::M,
                data2: stream.read_u8().unwrap(),
            }),
            0x37 => Some(Self::SetCarry),
            0x39 => Some(Self::DoubleAdd {
                register1: Intel8080Register::SP,
            }),
            0x3A => Some(Self::LoadAccumulatorDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x3B => Some(Self::DecrementRegisterPair {
                register1: Intel8080Register::SP,
            }),
            0x3C => Some(Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::A,
            }),
            0x3D => Some(Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::A,
            }),
            0x3E => Some(Self::MoveImmediateData {
                register1: Intel8080Register::A,
                data2: stream.read_u8().unwrap(),
            }),
            0x3F => Some(Self::ComplementCarry),
            0x40 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::B,
            }),
            0x41 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::C,
            }),
            0x42 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::D,
            }),
            0x43 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::E,
            }),
            0x44 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::H,
            }),
            0x45 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::L,
            }),
            0x46 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::M,
            }),
            0x47 => Some(Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::A,
            }),
            0x48 => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::B,
            }),
            0x49 => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::C,
            }),
            0x4A => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::D,
            }),
            0x4B => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::E,
            }),
            0x4C => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::H,
            }),
            0x4D => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::L,
            }),
            0x4E => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::M,
            }),
            0x4F => Some(Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::A,
            }),
            0x50 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::B,
            }),
            0x51 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::C,
            }),
            0x52 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::D,
            }),
            0x53 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::E,
            }),
            0x54 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::H,
            }),
            0x55 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::L,
            }),
            0x56 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::M,
            }),
            0x57 => Some(Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::A,
            }),
            0x58 => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::B,
            }),
            0x59 => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::C,
            }),
            0x5A => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::D,
            }),
            0x5B => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::E,
            }),
            0x5C => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::H,
            }),
            0x5D => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::L,
            }),
            0x5E => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::M,
            }),
            0x5F => Some(Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::A,
            }),
            0x60 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::B,
            }),
            0x61 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::C,
            }),
            0x62 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::D,
            }),
            0x63 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::E,
            }),
            0x64 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::H,
            }),
            0x65 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::L,
            }),
            0x66 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::M,
            }),
            0x67 => Some(Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::A,
            }),
            0x68 => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::B,
            }),
            0x69 => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::C,
            }),
            0x6A => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::D,
            }),
            0x6B => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::E,
            }),
            0x6C => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::H,
            }),
            0x6D => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::L,
            }),
            0x6E => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::M,
            }),
            0x6F => Some(Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::A,
            }),
            0x70 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::B,
            }),
            0x71 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::C,
            }),
            0x72 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::D,
            }),
            0x73 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::E,
            }),
            0x74 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::H,
            }),
            0x75 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::L,
            }),
            0x76 => Some(Self::Halt),
            0x77 => Some(Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
            }),
            0x78 => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::B,
            }),
            0x79 => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::C,
            }),
            0x7A => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::D,
            }),
            0x7B => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::E,
            }),
            0x7C => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::H,
            }),
            0x7D => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::L,
            }),
            0x7E => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
            }),
            0x7F => Some(Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::A,
            }),
            0x80 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::B,
            }),
            0x81 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::C,
            }),
            0x82 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::D,
            }),
            0x83 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::E,
            }),
            0x84 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::H,
            }),
            0x85 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::L,
            }),
            0x86 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::M,
            }),
            0x87 => Some(Self::AddToAccumulator {
                register1: Intel8080Register::A,
            }),
            0x88 => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::B,
            }),
            0x89 => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::C,
            }),
            0x8A => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::D,
            }),
            0x8B => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::E,
            }),
            0x8C => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::H,
            }),
            0x8D => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::L,
            }),
            0x8E => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::M,
            }),
            0x8F => Some(Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::A,
            }),
            0x90 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::B,
            }),
            0x91 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::C,
            }),
            0x92 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::D,
            }),
            0x93 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::E,
            }),
            0x94 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::H,
            }),
            0x95 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::L,
            }),
            0x96 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::M,
            }),
            0x97 => Some(Self::SubtractFromAccumulator {
                register1: Intel8080Register::A,
            }),
            0x98 => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::B,
            }),
            0x99 => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::C,
            }),
            0x9A => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::D,
            }),
            0x9B => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::E,
            }),
            0x9C => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::H,
            }),
            0x9D => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::L,
            }),
            0x9E => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::M,
            }),
            0x9F => Some(Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::A,
            }),
            0xA0 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::B,
            }),
            0xA1 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::C,
            }),
            0xA2 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::D,
            }),
            0xA3 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::E,
            }),
            0xA4 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::H,
            }),
            0xA5 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::L,
            }),
            0xA6 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::M,
            }),
            0xA7 => Some(Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::A,
            }),
            0xA8 => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::B,
            }),
            0xA9 => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::C,
            }),
            0xAA => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::D,
            }),
            0xAB => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::E,
            }),
            0xAC => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::H,
            }),
            0xAD => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::L,
            }),
            0xAE => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::M,
            }),
            0xAF => Some(Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::A,
            }),
            0xB0 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::B,
            }),
            0xB1 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::C,
            }),
            0xB2 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::D,
            }),
            0xB3 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::E,
            }),
            0xB4 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::H,
            }),
            0xB5 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::L,
            }),
            0xB6 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::M,
            }),
            0xB7 => Some(Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::A,
            }),
            0xB8 => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::B,
            }),
            0xB9 => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::C,
            }),
            0xBA => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::D,
            }),
            0xBB => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::E,
            }),
            0xBC => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::H,
            }),
            0xBD => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::L,
            }),
            0xBE => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::M,
            }),
            0xBF => Some(Self::CompareWithAccumulator {
                register1: Intel8080Register::A,
            }),
            0xC0 => Some(Self::ReturnIfNotZero),
            0xC1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::B,
            }),
            0xC2 => Some(Self::JumpIfNotZero {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xC3 => Some(Self::Jump {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xC4 => Some(Self::CallIfNotZero {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xC5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::B,
            }),
            0xC6 => Some(Self::AddImmediateToAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xC7 => Some(Self::Restart { data1: 0u8 }),
            0xC8 => Some(Self::ReturnIfZero),
            0xC9 => Some(Self::ReturnUnconditionally),
            0xCA => Some(Self::JumpIfZero {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xCC => Some(Self::CallIfZero {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xCD => Some(Self::Call {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xCE => Some(Self::AddImmediateToAccumulatorWithCarry {
                data1: stream.read_u8().unwrap(),
            }),
            0xCF => Some(Self::Restart { data1: 1u8 }),
            0xD0 => Some(Self::ReturnIfNoCarry),
            0xD1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::D,
            }),
            0xD2 => Some(Self::JumpIfNoCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xD3 => Some(Self::Output {
                data1: stream.read_u8().unwrap(),
            }),
            0xD4 => Some(Self::CallIfNoCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xD5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::D,
            }),
            0xD6 => Some(Self::SubtractImmediateFromAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xD7 => Some(Self::Restart { data1: 2u8 }),
            0xD8 => Some(Self::ReturnIfCarry),
            0xDA => Some(Self::JumpIfCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xDB => Some(Self::Input {
                data1: stream.read_u8().unwrap(),
            }),
            0xDC => Some(Self::CallIfCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xDE => Some(Self::SubtractImmediateFromAccumulatorWithBorrow {
                data1: stream.read_u8().unwrap(),
            }),
            0xDF => Some(Self::Restart { data1: 3u8 }),
            0xE0 => Some(Self::ReturnIfParityOdd),
            0xE1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::H,
            }),
            0xE2 => Some(Self::JumpIfParityOdd {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xE3 => Some(Self::ExchangeStack),
            0xE4 => Some(Self::CallIfParityOdd {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xE5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::H,
            }),
            0xE6 => Some(Self::AndImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xE7 => Some(Self::Restart { data1: 4u8 }),
            0xE8 => Some(Self::ReturnIfParityEven),
            0xE9 => Some(Self::LoadProgramCounter),
            0xEA => Some(Self::JumpIfParityEven {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xEB => Some(Self::ExchangeRegisters),
            0xEC => Some(Self::CallIfParityEven {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xEE => Some(Self::ExclusiveOrImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xEF => Some(Self::Restart { data1: 5u8 }),
            0xF0 => Some(Self::ReturnIfPlus),
            0xF1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::PSW,
            }),
            0xF2 => Some(Self::JumpIfPositive {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xF3 => Some(Self::DisableInterrupts),
            0xF4 => Some(Self::CallIfPlus {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xF5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::PSW,
            }),
            0xF6 => Some(Self::OrImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xF7 => Some(Self::Restart { data1: 6u8 }),
            0xF8 => Some(Self::ReturnIfMinus),
            0xF9 => Some(Self::LoadSpFromHAndL),
            0xFA => Some(Self::JumpIfMinus {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xFB => Some(Self::EnableInterrupts),
            0xFC => Some(Self::CallIfMinus {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xFE => Some(Self::CompareImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xFF => Some(Self::Restart { data1: 7u8 }),
            _ => None,
        })
    }
    pub fn to_type(&self) -> Intel8080InstructionType {
        match self {
            Self::AddImmediateToAccumulator { .. } => {
                Intel8080InstructionType::AddImmediateToAccumulator
            }
            Self::AddImmediateToAccumulatorWithCarry { .. } => {
                Intel8080InstructionType::AddImmediateToAccumulatorWithCarry
            }
            Self::AddToAccumulator { .. } => Intel8080InstructionType::AddToAccumulator,
            Self::AddToAccumulatorWithCarry { .. } => {
                Intel8080InstructionType::AddToAccumulatorWithCarry
            }
            Self::AndImmediateWithAccumulator { .. } => {
                Intel8080InstructionType::AndImmediateWithAccumulator
            }
            Self::Call { .. } => Intel8080InstructionType::Call,
            Self::CallIfCarry { .. } => Intel8080InstructionType::CallIfCarry,
            Self::CallIfMinus { .. } => Intel8080InstructionType::CallIfMinus,
            Self::CallIfNoCarry { .. } => Intel8080InstructionType::CallIfNoCarry,
            Self::CallIfNotZero { .. } => Intel8080InstructionType::CallIfNotZero,
            Self::CallIfParityEven { .. } => Intel8080InstructionType::CallIfParityEven,
            Self::CallIfParityOdd { .. } => Intel8080InstructionType::CallIfParityOdd,
            Self::CallIfPlus { .. } => Intel8080InstructionType::CallIfPlus,
            Self::CallIfZero { .. } => Intel8080InstructionType::CallIfZero,
            Self::CompareImmediateWithAccumulator { .. } => {
                Intel8080InstructionType::CompareImmediateWithAccumulator
            }
            Self::CompareWithAccumulator { .. } => Intel8080InstructionType::CompareWithAccumulator,
            Self::ComplementAccumulator => Intel8080InstructionType::ComplementAccumulator,
            Self::ComplementCarry => Intel8080InstructionType::ComplementCarry,
            Self::DecimalAdjustAccumulator => Intel8080InstructionType::DecimalAdjustAccumulator,
            Self::DecrementRegisterOrMemory { .. } => {
                Intel8080InstructionType::DecrementRegisterOrMemory
            }
            Self::DecrementRegisterPair { .. } => Intel8080InstructionType::DecrementRegisterPair,
            Self::DisableInterrupts => Intel8080InstructionType::DisableInterrupts,
            Self::DoubleAdd { .. } => Intel8080InstructionType::DoubleAdd,
            Self::EnableInterrupts => Intel8080InstructionType::EnableInterrupts,
            Self::ExchangeRegisters => Intel8080InstructionType::ExchangeRegisters,
            Self::ExchangeStack => Intel8080InstructionType::ExchangeStack,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => {
                Intel8080InstructionType::ExclusiveOrImmediateWithAccumulator
            }
            Self::Halt => Intel8080InstructionType::Halt,
            Self::IncrementRegisterOrMemory { .. } => {
                Intel8080InstructionType::IncrementRegisterOrMemory
            }
            Self::IncrementRegisterPair { .. } => Intel8080InstructionType::IncrementRegisterPair,
            Self::Input { .. } => Intel8080InstructionType::Input,
            Self::Jump { .. } => Intel8080InstructionType::Jump,
            Self::JumpIfCarry { .. } => Intel8080InstructionType::JumpIfCarry,
            Self::JumpIfMinus { .. } => Intel8080InstructionType::JumpIfMinus,
            Self::JumpIfNoCarry { .. } => Intel8080InstructionType::JumpIfNoCarry,
            Self::JumpIfNotZero { .. } => Intel8080InstructionType::JumpIfNotZero,
            Self::JumpIfParityEven { .. } => Intel8080InstructionType::JumpIfParityEven,
            Self::JumpIfParityOdd { .. } => Intel8080InstructionType::JumpIfParityOdd,
            Self::JumpIfPositive { .. } => Intel8080InstructionType::JumpIfPositive,
            Self::JumpIfZero { .. } => Intel8080InstructionType::JumpIfZero,
            Self::LoadAccumulator { .. } => Intel8080InstructionType::LoadAccumulator,
            Self::LoadAccumulatorDirect { .. } => Intel8080InstructionType::LoadAccumulatorDirect,
            Self::LoadHAndLDirect { .. } => Intel8080InstructionType::LoadHAndLDirect,
            Self::LoadProgramCounter => Intel8080InstructionType::LoadProgramCounter,
            Self::LoadRegisterPairImmediate { .. } => {
                Intel8080InstructionType::LoadRegisterPairImmediate
            }
            Self::LoadSpFromHAndL => Intel8080InstructionType::LoadSpFromHAndL,
            Self::LogicalAndWithAccumulator { .. } => {
                Intel8080InstructionType::LogicalAndWithAccumulator
            }
            Self::LogicalExclusiveOrWithAccumulator { .. } => {
                Intel8080InstructionType::LogicalExclusiveOrWithAccumulator
            }
            Self::LogicalOrWithAccumulator { .. } => {
                Intel8080InstructionType::LogicalOrWithAccumulator
            }
            Self::MoveData { .. } => Intel8080InstructionType::MoveData,
            Self::MoveImmediateData { .. } => Intel8080InstructionType::MoveImmediateData,
            Self::NoOperation => Intel8080InstructionType::NoOperation,
            Self::OrImmediateWithAccumulator { .. } => {
                Intel8080InstructionType::OrImmediateWithAccumulator
            }
            Self::Output { .. } => Intel8080InstructionType::Output,
            Self::PopDataOffStack { .. } => Intel8080InstructionType::PopDataOffStack,
            Self::PushDataOntoStack { .. } => Intel8080InstructionType::PushDataOntoStack,
            Self::Restart { .. } => Intel8080InstructionType::Restart,
            Self::ReturnIfCarry => Intel8080InstructionType::ReturnIfCarry,
            Self::ReturnIfMinus => Intel8080InstructionType::ReturnIfMinus,
            Self::ReturnIfNoCarry => Intel8080InstructionType::ReturnIfNoCarry,
            Self::ReturnIfNotZero => Intel8080InstructionType::ReturnIfNotZero,
            Self::ReturnIfParityEven => Intel8080InstructionType::ReturnIfParityEven,
            Self::ReturnIfParityOdd => Intel8080InstructionType::ReturnIfParityOdd,
            Self::ReturnIfPlus => Intel8080InstructionType::ReturnIfPlus,
            Self::ReturnIfZero => Intel8080InstructionType::ReturnIfZero,
            Self::ReturnUnconditionally => Intel8080InstructionType::ReturnUnconditionally,
            Self::Rim => Intel8080InstructionType::Rim,
            Self::RotateAccumulatorLeft => Intel8080InstructionType::RotateAccumulatorLeft,
            Self::RotateAccumulatorLeftThroughCarry => {
                Intel8080InstructionType::RotateAccumulatorLeftThroughCarry
            }
            Self::RotateAccumulatorRight => Intel8080InstructionType::RotateAccumulatorRight,
            Self::RotateAccumulatorRightThroughCarry => {
                Intel8080InstructionType::RotateAccumulatorRightThroughCarry
            }
            Self::SetCarry => Intel8080InstructionType::SetCarry,
            Self::Sim => Intel8080InstructionType::Sim,
            Self::StoreAccumulator { .. } => Intel8080InstructionType::StoreAccumulator,
            Self::StoreAccumulatorDirect { .. } => Intel8080InstructionType::StoreAccumulatorDirect,
            Self::StoreHAndLDirect { .. } => Intel8080InstructionType::StoreHAndLDirect,
            Self::SubtractFromAccumulator { .. } => {
                Intel8080InstructionType::SubtractFromAccumulator
            }
            Self::SubtractFromAccumulatorWithBorrow { .. } => {
                Intel8080InstructionType::SubtractFromAccumulatorWithBorrow
            }
            Self::SubtractImmediateFromAccumulator { .. } => {
                Intel8080InstructionType::SubtractImmediateFromAccumulator
            }
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => {
                Intel8080InstructionType::SubtractImmediateFromAccumulatorWithBorrow
            }
        }
    }
}
impl Intel8080Instruction {
    pub fn size(&self) -> u8 {
        match self {
            Self::NoOperation { .. } => 1u8,
            Self::LoadRegisterPairImmediate { .. } => 3u8,
            Self::StoreAccumulator { .. } => 1u8,
            Self::IncrementRegisterPair { .. } => 1u8,
            Self::IncrementRegisterOrMemory { .. } => 1u8,
            Self::DecrementRegisterOrMemory { .. } => 1u8,
            Self::MoveImmediateData { .. } => 2u8,
            Self::RotateAccumulatorLeft { .. } => 1u8,
            Self::DoubleAdd { .. } => 1u8,
            Self::LoadAccumulator { .. } => 1u8,
            Self::DecrementRegisterPair { .. } => 1u8,
            Self::RotateAccumulatorRight { .. } => 1u8,
            Self::RotateAccumulatorLeftThroughCarry { .. } => 1u8,
            Self::RotateAccumulatorRightThroughCarry { .. } => 1u8,
            Self::Rim { .. } => 1u8,
            Self::StoreHAndLDirect { .. } => 3u8,
            Self::DecimalAdjustAccumulator { .. } => 1u8,
            Self::LoadHAndLDirect { .. } => 3u8,
            Self::ComplementAccumulator { .. } => 1u8,
            Self::Sim { .. } => 1u8,
            Self::StoreAccumulatorDirect { .. } => 3u8,
            Self::SetCarry { .. } => 1u8,
            Self::LoadAccumulatorDirect { .. } => 3u8,
            Self::ComplementCarry { .. } => 1u8,
            Self::MoveData { .. } => 1u8,
            Self::Halt { .. } => 1u8,
            Self::AddToAccumulator { .. } => 1u8,
            Self::AddToAccumulatorWithCarry { .. } => 1u8,
            Self::SubtractFromAccumulator { .. } => 1u8,
            Self::SubtractFromAccumulatorWithBorrow { .. } => 1u8,
            Self::LogicalAndWithAccumulator { .. } => 1u8,
            Self::LogicalExclusiveOrWithAccumulator { .. } => 1u8,
            Self::LogicalOrWithAccumulator { .. } => 1u8,
            Self::CompareWithAccumulator { .. } => 1u8,
            Self::ReturnIfNotZero { .. } => 1u8,
            Self::PopDataOffStack { .. } => 1u8,
            Self::JumpIfNotZero { .. } => 3u8,
            Self::Jump { .. } => 3u8,
            Self::CallIfNotZero { .. } => 3u8,
            Self::PushDataOntoStack { .. } => 1u8,
            Self::AddImmediateToAccumulator { .. } => 2u8,
            Self::Restart { .. } => 1u8,
            Self::ReturnIfZero { .. } => 1u8,
            Self::ReturnUnconditionally { .. } => 1u8,
            Self::JumpIfZero { .. } => 3u8,
            Self::CallIfZero { .. } => 3u8,
            Self::Call { .. } => 3u8,
            Self::AddImmediateToAccumulatorWithCarry { .. } => 2u8,
            Self::ReturnIfNoCarry { .. } => 1u8,
            Self::JumpIfNoCarry { .. } => 3u8,
            Self::Output { .. } => 2u8,
            Self::CallIfNoCarry { .. } => 3u8,
            Self::SubtractImmediateFromAccumulator { .. } => 2u8,
            Self::ReturnIfCarry { .. } => 1u8,
            Self::JumpIfCarry { .. } => 3u8,
            Self::Input { .. } => 2u8,
            Self::CallIfCarry { .. } => 3u8,
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => 2u8,
            Self::ReturnIfParityOdd { .. } => 1u8,
            Self::JumpIfParityOdd { .. } => 3u8,
            Self::ExchangeStack { .. } => 1u8,
            Self::CallIfParityOdd { .. } => 3u8,
            Self::AndImmediateWithAccumulator { .. } => 2u8,
            Self::ReturnIfParityEven { .. } => 1u8,
            Self::LoadProgramCounter { .. } => 1u8,
            Self::JumpIfParityEven { .. } => 3u8,
            Self::ExchangeRegisters { .. } => 1u8,
            Self::CallIfParityEven { .. } => 3u8,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => 2u8,
            Self::ReturnIfPlus { .. } => 1u8,
            Self::JumpIfPositive { .. } => 3u8,
            Self::DisableInterrupts { .. } => 1u8,
            Self::CallIfPlus { .. } => 3u8,
            Self::OrImmediateWithAccumulator { .. } => 2u8,
            Self::ReturnIfMinus { .. } => 1u8,
            Self::LoadSpFromHAndL { .. } => 1u8,
            Self::JumpIfMinus { .. } => 3u8,
            Self::EnableInterrupts { .. } => 1u8,
            Self::CallIfMinus { .. } => 3u8,
            Self::CompareImmediateWithAccumulator { .. } => 2u8,
        }
    }
}
impl Intel8080Instruction {
    pub fn duration(&self) -> u8 {
        match self {
            Self::NoOperation { .. } => 0u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::StoreAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::RotateAccumulatorLeft { .. } => 0u8,
            Self::DoubleAdd {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::LoadAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::RotateAccumulatorRight { .. } => 0u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::StoreAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::RotateAccumulatorLeftThroughCarry { .. } => 0u8,
            Self::DoubleAdd {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::LoadAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::RotateAccumulatorRightThroughCarry { .. } => 0u8,
            Self::Rim { .. } => 0u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::StoreHAndLDirect { .. } => 0u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::DecimalAdjustAccumulator { .. } => 0u8,
            Self::DoubleAdd {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::LoadHAndLDirect { .. } => 0u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::ComplementAccumulator { .. } => 0u8,
            Self::Sim { .. } => 0u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::SP,
                ..
            } => 0u8,
            Self::StoreAccumulatorDirect { .. } => 0u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::SP,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::SetCarry { .. } => 0u8,
            Self::DoubleAdd {
                register1: Intel8080Register::SP,
                ..
            } => 0u8,
            Self::LoadAccumulatorDirect { .. } => 0u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::SP,
                ..
            } => 0u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::ComplementCarry { .. } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::Halt { .. } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::B,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::C,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::D,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::E,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::H,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::L,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
                ..
            } => 0u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::A,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 0u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 0u8,
            Self::ReturnIfNotZero { .. } => 0u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::JumpIfNotZero { .. } => 0u8,
            Self::Jump { .. } => 0u8,
            Self::CallIfNotZero { .. } => 0u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::B,
                ..
            } => 0u8,
            Self::AddImmediateToAccumulator { .. } => 0u8,
            Self::Restart { data1: 0u8, .. } => 0u8,
            Self::ReturnIfZero { .. } => 0u8,
            Self::ReturnUnconditionally { .. } => 0u8,
            Self::JumpIfZero { .. } => 0u8,
            Self::CallIfZero { .. } => 0u8,
            Self::Call { .. } => 0u8,
            Self::AddImmediateToAccumulatorWithCarry { .. } => 0u8,
            Self::Restart { data1: 1u8, .. } => 0u8,
            Self::ReturnIfNoCarry { .. } => 0u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::JumpIfNoCarry { .. } => 0u8,
            Self::Output { .. } => 0u8,
            Self::CallIfNoCarry { .. } => 0u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::D,
                ..
            } => 0u8,
            Self::SubtractImmediateFromAccumulator { .. } => 0u8,
            Self::Restart { data1: 2u8, .. } => 0u8,
            Self::ReturnIfCarry { .. } => 0u8,
            Self::JumpIfCarry { .. } => 0u8,
            Self::Input { .. } => 0u8,
            Self::CallIfCarry { .. } => 0u8,
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => 0u8,
            Self::Restart { data1: 3u8, .. } => 0u8,
            Self::ReturnIfParityOdd { .. } => 0u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::JumpIfParityOdd { .. } => 0u8,
            Self::ExchangeStack { .. } => 0u8,
            Self::CallIfParityOdd { .. } => 0u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::H,
                ..
            } => 0u8,
            Self::AndImmediateWithAccumulator { .. } => 0u8,
            Self::Restart { data1: 4u8, .. } => 0u8,
            Self::ReturnIfParityEven { .. } => 0u8,
            Self::LoadProgramCounter { .. } => 0u8,
            Self::JumpIfParityEven { .. } => 0u8,
            Self::ExchangeRegisters { .. } => 0u8,
            Self::CallIfParityEven { .. } => 0u8,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => 0u8,
            Self::Restart { data1: 5u8, .. } => 0u8,
            Self::ReturnIfPlus { .. } => 0u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::PSW,
                ..
            } => 0u8,
            Self::JumpIfPositive { .. } => 0u8,
            Self::DisableInterrupts { .. } => 0u8,
            Self::CallIfPlus { .. } => 0u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::PSW,
                ..
            } => 0u8,
            Self::OrImmediateWithAccumulator { .. } => 0u8,
            Self::Restart { data1: 6u8, .. } => 0u8,
            Self::ReturnIfMinus { .. } => 0u8,
            Self::LoadSpFromHAndL { .. } => 0u8,
            Self::JumpIfMinus { .. } => 0u8,
            Self::EnableInterrupts { .. } => 0u8,
            Self::CallIfMinus { .. } => 0u8,
            Self::CompareImmediateWithAccumulator { .. } => 0u8,
            Self::Restart { data1: 7u8, .. } => 0u8,
            instr => panic!("invalid instruction {:?}", instr),
        }
    }
}
pub trait Intel8080InstructionSet {
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn call_if_carry(&mut self, address1: u16);
    fn call_if_minus(&mut self, address1: u16);
    fn call_if_no_carry(&mut self, address1: u16);
    fn call_if_not_zero(&mut self, address1: u16);
    fn call_if_parity_even(&mut self, address1: u16);
    fn call_if_parity_odd(&mut self, address1: u16);
    fn call_if_plus(&mut self, address1: u16);
    fn call_if_zero(&mut self, address1: u16);
    fn compare_immediate_with_accumulator(&mut self, data1: u8);
    fn compare_with_accumulator(&mut self, register1: Intel8080Register);
    fn complement_accumulator(&mut self);
    fn complement_carry(&mut self);
    fn decimal_adjust_accumulator(&mut self);
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register);
    fn decrement_register_pair(&mut self, register1: Intel8080Register);
    fn disable_interrupts(&mut self);
    fn double_add(&mut self, register1: Intel8080Register);
    fn enable_interrupts(&mut self);
    fn exchange_registers(&mut self);
    fn exchange_stack(&mut self);
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn halt(&mut self);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn input(&mut self, data1: u8);
    fn jump(&mut self, address1: u16);
    fn jump_if_carry(&mut self, address1: u16);
    fn jump_if_minus(&mut self, address1: u16);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_parity_even(&mut self, address1: u16);
    fn jump_if_parity_odd(&mut self, address1: u16);
    fn jump_if_positive(&mut self, address1: u16);
    fn jump_if_zero(&mut self, address1: u16);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn load_h_and_l_direct(&mut self, address1: u16);
    fn load_program_counter(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn no_operation(&mut self);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn output(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn restart(&mut self, data1: u8);
    fn return_if_carry(&mut self);
    fn return_if_minus(&mut self);
    fn return_if_no_carry(&mut self);
    fn return_if_not_zero(&mut self);
    fn return_if_parity_even(&mut self);
    fn return_if_parity_odd(&mut self);
    fn return_if_plus(&mut self);
    fn return_if_zero(&mut self);
    fn return_unconditionally(&mut self);
    fn rim(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn set_carry(&mut self);
    fn sim(&mut self);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn store_h_and_l_direct(&mut self, address1: u16);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
}
impl Intel8080Instruction {
    #[inline(always)]
    pub fn dispatch<I: Intel8080InstructionSet>(self, machine: &mut I) {
        match self {
            Self::NoOperation {} => machine.no_operation(),
            Self::LoadRegisterPairImmediate { register1, data2 } => {
                machine.load_register_pair_immediate(register1, data2)
            }
            Self::StoreAccumulator { register1 } => machine.store_accumulator(register1),
            Self::IncrementRegisterPair { register1 } => machine.increment_register_pair(register1),
            Self::IncrementRegisterOrMemory { register1 } => {
                machine.increment_register_or_memory(register1)
            }
            Self::DecrementRegisterOrMemory { register1 } => {
                machine.decrement_register_or_memory(register1)
            }
            Self::MoveImmediateData { register1, data2 } => {
                machine.move_immediate_data(register1, data2)
            }
            Self::RotateAccumulatorLeft {} => machine.rotate_accumulator_left(),
            Self::DoubleAdd { register1 } => machine.double_add(register1),
            Self::LoadAccumulator { register1 } => machine.load_accumulator(register1),
            Self::DecrementRegisterPair { register1 } => machine.decrement_register_pair(register1),
            Self::RotateAccumulatorRight {} => machine.rotate_accumulator_right(),
            Self::RotateAccumulatorLeftThroughCarry {} => {
                machine.rotate_accumulator_left_through_carry()
            }
            Self::RotateAccumulatorRightThroughCarry {} => {
                machine.rotate_accumulator_right_through_carry()
            }
            Self::Rim {} => machine.rim(),
            Self::StoreHAndLDirect { address1 } => machine.store_h_and_l_direct(address1),
            Self::DecimalAdjustAccumulator {} => machine.decimal_adjust_accumulator(),
            Self::LoadHAndLDirect { address1 } => machine.load_h_and_l_direct(address1),
            Self::ComplementAccumulator {} => machine.complement_accumulator(),
            Self::Sim {} => machine.sim(),
            Self::StoreAccumulatorDirect { address1 } => machine.store_accumulator_direct(address1),
            Self::SetCarry {} => machine.set_carry(),
            Self::LoadAccumulatorDirect { address1 } => machine.load_accumulator_direct(address1),
            Self::ComplementCarry {} => machine.complement_carry(),
            Self::MoveData {
                register1,
                register2,
            } => machine.move_data(register1, register2),
            Self::Halt {} => machine.halt(),
            Self::AddToAccumulator { register1 } => machine.add_to_accumulator(register1),
            Self::AddToAccumulatorWithCarry { register1 } => {
                machine.add_to_accumulator_with_carry(register1)
            }
            Self::SubtractFromAccumulator { register1 } => {
                machine.subtract_from_accumulator(register1)
            }
            Self::SubtractFromAccumulatorWithBorrow { register1 } => {
                machine.subtract_from_accumulator_with_borrow(register1)
            }
            Self::LogicalAndWithAccumulator { register1 } => {
                machine.logical_and_with_accumulator(register1)
            }
            Self::LogicalExclusiveOrWithAccumulator { register1 } => {
                machine.logical_exclusive_or_with_accumulator(register1)
            }
            Self::LogicalOrWithAccumulator { register1 } => {
                machine.logical_or_with_accumulator(register1)
            }
            Self::CompareWithAccumulator { register1 } => {
                machine.compare_with_accumulator(register1)
            }
            Self::ReturnIfNotZero {} => machine.return_if_not_zero(),
            Self::PopDataOffStack { register1 } => machine.pop_data_off_stack(register1),
            Self::JumpIfNotZero { address1 } => machine.jump_if_not_zero(address1),
            Self::Jump { address1 } => machine.jump(address1),
            Self::CallIfNotZero { address1 } => machine.call_if_not_zero(address1),
            Self::PushDataOntoStack { register1 } => machine.push_data_onto_stack(register1),
            Self::AddImmediateToAccumulator { data1 } => {
                machine.add_immediate_to_accumulator(data1)
            }
            Self::Restart { data1 } => machine.restart(data1),
            Self::ReturnIfZero {} => machine.return_if_zero(),
            Self::ReturnUnconditionally {} => machine.return_unconditionally(),
            Self::JumpIfZero { address1 } => machine.jump_if_zero(address1),
            Self::CallIfZero { address1 } => machine.call_if_zero(address1),
            Self::Call { address1 } => machine.call(address1),
            Self::AddImmediateToAccumulatorWithCarry { data1 } => {
                machine.add_immediate_to_accumulator_with_carry(data1)
            }
            Self::ReturnIfNoCarry {} => machine.return_if_no_carry(),
            Self::JumpIfNoCarry { address1 } => machine.jump_if_no_carry(address1),
            Self::Output { data1 } => machine.output(data1),
            Self::CallIfNoCarry { address1 } => machine.call_if_no_carry(address1),
            Self::SubtractImmediateFromAccumulator { data1 } => {
                machine.subtract_immediate_from_accumulator(data1)
            }
            Self::ReturnIfCarry {} => machine.return_if_carry(),
            Self::JumpIfCarry { address1 } => machine.jump_if_carry(address1),
            Self::Input { data1 } => machine.input(data1),
            Self::CallIfCarry { address1 } => machine.call_if_carry(address1),
            Self::SubtractImmediateFromAccumulatorWithBorrow { data1 } => {
                machine.subtract_immediate_from_accumulator_with_borrow(data1)
            }
            Self::ReturnIfParityOdd {} => machine.return_if_parity_odd(),
            Self::JumpIfParityOdd { address1 } => machine.jump_if_parity_odd(address1),
            Self::ExchangeStack {} => machine.exchange_stack(),
            Self::CallIfParityOdd { address1 } => machine.call_if_parity_odd(address1),
            Self::AndImmediateWithAccumulator { data1 } => {
                machine.and_immediate_with_accumulator(data1)
            }
            Self::ReturnIfParityEven {} => machine.return_if_parity_even(),
            Self::LoadProgramCounter {} => machine.load_program_counter(),
            Self::JumpIfParityEven { address1 } => machine.jump_if_parity_even(address1),
            Self::ExchangeRegisters {} => machine.exchange_registers(),
            Self::CallIfParityEven { address1 } => machine.call_if_parity_even(address1),
            Self::ExclusiveOrImmediateWithAccumulator { data1 } => {
                machine.exclusive_or_immediate_with_accumulator(data1)
            }
            Self::ReturnIfPlus {} => machine.return_if_plus(),
            Self::JumpIfPositive { address1 } => machine.jump_if_positive(address1),
            Self::DisableInterrupts {} => machine.disable_interrupts(),
            Self::CallIfPlus { address1 } => machine.call_if_plus(address1),
            Self::OrImmediateWithAccumulator { data1 } => {
                machine.or_immediate_with_accumulator(data1)
            }
            Self::ReturnIfMinus {} => machine.return_if_minus(),
            Self::LoadSpFromHAndL {} => machine.load_sp_from_h_and_l(),
            Self::JumpIfMinus { address1 } => machine.jump_if_minus(address1),
            Self::EnableInterrupts {} => machine.enable_interrupts(),
            Self::CallIfMinus { address1 } => machine.call_if_minus(address1),
            Self::CompareImmediateWithAccumulator { data1 } => {
                machine.compare_immediate_with_accumulator(data1)
            }
        }
    }
}
impl<'a> Intel8080InstructionSet for Intel8080InstructionPrinter<'a> {
    fn add_immediate_to_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ADI", data1);
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ACI", data1);
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADD", register1);
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ADC", register1);
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ANI", data1);
    }
    fn call(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CALL", address1);
    }
    fn call_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CC", address1);
    }
    fn call_if_minus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CM", address1);
    }
    fn call_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNC", address1);
    }
    fn call_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CNZ", address1);
    }
    fn call_if_parity_even(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPE", address1);
    }
    fn call_if_parity_odd(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CPO", address1);
    }
    fn call_if_plus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CP", address1);
    }
    fn call_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "CZ", address1);
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "CPI", data1);
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "CMP", register1);
    }
    fn complement_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CMA");
    }
    fn complement_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CMC");
    }
    fn decimal_adjust_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCR", register1);
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DCX", register1);
    }
    fn disable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DI");
    }
    fn double_add(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "DAD", register1);
    }
    fn enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "EI");
    }
    fn exchange_registers(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "XCHG");
    }
    fn exchange_stack(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "XTHL");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "XRI", data1);
    }
    fn halt(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INR", register1);
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "INX", register1);
    }
    fn input(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "IN", data1);
    }
    fn jump(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JMP", address1);
    }
    fn jump_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JC", address1);
    }
    fn jump_if_minus(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JM", address1);
    }
    fn jump_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNC", address1);
    }
    fn jump_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JNZ", address1);
    }
    fn jump_if_parity_even(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPE", address1);
    }
    fn jump_if_parity_odd(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JPO", address1);
    }
    fn jump_if_positive(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JP", address1);
    }
    fn jump_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "JZ", address1);
    }
    fn load_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "LDAX", register1);
    }
    fn load_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LDA", address1);
    }
    fn load_h_and_l_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "LHLD", address1);
    }
    fn load_program_counter(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "PCHL");
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} #${:02x}",
            "LXI", register1, data2
        );
    }
    fn load_sp_from_h_and_l(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SPHL");
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ANA", register1);
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "XRA", register1);
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "ORA", register1);
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} {:?}",
            "MOV", register1, register2
        );
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} {:?} #${:02x}",
            "MVI", register1, data2
        );
    }
    fn no_operation(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "NOP");
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "ORI", data1);
    }
    fn output(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "OUT", data1);
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "POP", register1);
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "PUSH", register1);
    }
    fn restart(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} {}", "RST", data1);
    }
    fn return_if_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn return_if_minus(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RM");
    }
    fn return_if_no_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn return_if_not_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn return_if_parity_even(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RPE");
    }
    fn return_if_parity_odd(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RPO");
    }
    fn return_if_plus(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RP");
    }
    fn return_if_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn return_unconditionally(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RET");
    }
    fn rim(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RIM");
    }
    fn rotate_accumulator_left(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RLC");
    }
    fn rotate_accumulator_left_through_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RAL");
    }
    fn rotate_accumulator_right(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RRC");
    }
    fn rotate_accumulator_right_through_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RAR");
    }
    fn set_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STC");
    }
    fn sim(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SIM");
    }
    fn store_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "STAX", register1);
    }
    fn store_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "STA", address1);
    }
    fn store_h_and_l_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "SHLD", address1);
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SUB", register1);
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {:?}", "SBB", register1);
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SUI", data1);
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${:02x}", "SBI", data1);
    }
}
