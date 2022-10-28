#![allow(dead_code)]
use crate::bytes::{LittleEndian, ReadBytesExt};
use crate::emulator_common::Intel8080Register;
use crate::io;
use crate::lr35902_emulator::opcodes::LR35902InstructionPrinter;
use serde_derive::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LR35902Instruction {
    AddImmediateToAccumulator {
        data1: u8,
    },
    AddImmediateToAccumulatorWithCarry {
        data1: u8,
    },
    AddImmediateToSp {
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
    CallIfNoCarry {
        address1: u16,
    },
    CallIfNotZero {
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
    ExclusiveOrImmediateWithAccumulator {
        data1: u8,
    },
    Halt,
    HaltUntilButtonPress,
    IncrementRegisterOrMemory {
        register1: Intel8080Register,
    },
    IncrementRegisterPair {
        register1: Intel8080Register,
    },
    Jump {
        address1: u16,
    },
    JumpIfCarry {
        address1: u16,
    },
    JumpIfNoCarry {
        address1: u16,
    },
    JumpIfNotZero {
        address1: u16,
    },
    JumpIfZero {
        address1: u16,
    },
    JumpRelative {
        data1: u8,
    },
    JumpRelativeIfCarry {
        data1: u8,
    },
    JumpRelativeIfNoCarry {
        data1: u8,
    },
    JumpRelativeIfNotZero {
        data1: u8,
    },
    JumpRelativeIfZero {
        data1: u8,
    },
    LoadAccumulator {
        register1: Intel8080Register,
    },
    LoadAccumulatorDirect {
        address1: u16,
    },
    LoadAccumulatorDirectOneByte {
        data1: u8,
    },
    LoadAccumulatorOneByte,
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
    MoveAndDecrementHl {
        register1: Intel8080Register,
        register2: Intel8080Register,
    },
    MoveAndIncrementHl {
        register1: Intel8080Register,
        register2: Intel8080Register,
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
    PopDataOffStack {
        register1: Intel8080Register,
    },
    PushDataOntoStack {
        register1: Intel8080Register,
    },
    ResetBit {
        data1: u8,
        register2: Intel8080Register,
    },
    Restart {
        data1: u8,
    },
    ReturnAndEnableInterrupts,
    ReturnIfCarry,
    ReturnIfNoCarry,
    ReturnIfNotZero,
    ReturnIfZero,
    ReturnUnconditionally,
    RotateAccumulatorLeft,
    RotateAccumulatorLeftThroughCarry,
    RotateAccumulatorRight,
    RotateAccumulatorRightThroughCarry,
    RotateRegisterLeft {
        register1: Intel8080Register,
    },
    RotateRegisterLeftThroughCarry {
        register1: Intel8080Register,
    },
    RotateRegisterRight {
        register1: Intel8080Register,
    },
    RotateRegisterRightThroughCarry {
        register1: Intel8080Register,
    },
    SetBit {
        data1: u8,
        register2: Intel8080Register,
    },
    SetCarry,
    ShiftRegisterLeft {
        register1: Intel8080Register,
    },
    ShiftRegisterRight {
        register1: Intel8080Register,
    },
    ShiftRegisterRightSigned {
        register1: Intel8080Register,
    },
    StoreAccumulator {
        register1: Intel8080Register,
    },
    StoreAccumulatorDirect {
        address1: u16,
    },
    StoreAccumulatorDirectOneByte {
        data1: u8,
    },
    StoreAccumulatorOneByte,
    StoreSpDirect {
        address1: u16,
    },
    StoreSpPlusImmediate {
        data1: u8,
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
    SwapRegister {
        register1: Intel8080Register,
    },
    TestBit {
        data1: u8,
        register2: Intel8080Register,
    },
}
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, enum_iterator :: IntoEnumIterator,
)]
pub enum LR35902InstructionType {
    AddImmediateToAccumulator = 0isize,
    AddImmediateToAccumulatorWithCarry = 1isize,
    AddImmediateToSp = 2isize,
    AddToAccumulator = 3isize,
    AddToAccumulatorWithCarry = 4isize,
    AndImmediateWithAccumulator = 5isize,
    Call = 6isize,
    CallIfCarry = 7isize,
    CallIfNoCarry = 8isize,
    CallIfNotZero = 9isize,
    CallIfZero = 10isize,
    CompareImmediateWithAccumulator = 11isize,
    CompareWithAccumulator = 12isize,
    ComplementAccumulator = 13isize,
    ComplementCarry = 14isize,
    DecimalAdjustAccumulator = 15isize,
    DecrementRegisterOrMemory = 16isize,
    DecrementRegisterPair = 17isize,
    DisableInterrupts = 18isize,
    DoubleAdd = 19isize,
    EnableInterrupts = 20isize,
    ExclusiveOrImmediateWithAccumulator = 21isize,
    Halt = 22isize,
    HaltUntilButtonPress = 23isize,
    IncrementRegisterOrMemory = 24isize,
    IncrementRegisterPair = 25isize,
    Jump = 26isize,
    JumpIfCarry = 27isize,
    JumpIfNoCarry = 28isize,
    JumpIfNotZero = 29isize,
    JumpIfZero = 30isize,
    JumpRelative = 31isize,
    JumpRelativeIfCarry = 32isize,
    JumpRelativeIfNoCarry = 33isize,
    JumpRelativeIfNotZero = 34isize,
    JumpRelativeIfZero = 35isize,
    LoadAccumulator = 36isize,
    LoadAccumulatorDirect = 37isize,
    LoadAccumulatorDirectOneByte = 38isize,
    LoadAccumulatorOneByte = 39isize,
    LoadProgramCounter = 40isize,
    LoadRegisterPairImmediate = 41isize,
    LoadSpFromHAndL = 42isize,
    LogicalAndWithAccumulator = 43isize,
    LogicalExclusiveOrWithAccumulator = 44isize,
    LogicalOrWithAccumulator = 45isize,
    MoveAndDecrementHl = 46isize,
    MoveAndIncrementHl = 47isize,
    MoveData = 48isize,
    MoveImmediateData = 49isize,
    NoOperation = 50isize,
    OrImmediateWithAccumulator = 51isize,
    PopDataOffStack = 52isize,
    PushDataOntoStack = 53isize,
    ResetBit = 54isize,
    Restart = 55isize,
    ReturnAndEnableInterrupts = 56isize,
    ReturnIfCarry = 57isize,
    ReturnIfNoCarry = 58isize,
    ReturnIfNotZero = 59isize,
    ReturnIfZero = 60isize,
    ReturnUnconditionally = 61isize,
    RotateAccumulatorLeft = 62isize,
    RotateAccumulatorLeftThroughCarry = 63isize,
    RotateAccumulatorRight = 64isize,
    RotateAccumulatorRightThroughCarry = 65isize,
    RotateRegisterLeft = 66isize,
    RotateRegisterLeftThroughCarry = 67isize,
    RotateRegisterRight = 68isize,
    RotateRegisterRightThroughCarry = 69isize,
    SetBit = 70isize,
    SetCarry = 71isize,
    ShiftRegisterLeft = 72isize,
    ShiftRegisterRight = 73isize,
    ShiftRegisterRightSigned = 74isize,
    StoreAccumulator = 75isize,
    StoreAccumulatorDirect = 76isize,
    StoreAccumulatorDirectOneByte = 77isize,
    StoreAccumulatorOneByte = 78isize,
    StoreSpDirect = 79isize,
    StoreSpPlusImmediate = 80isize,
    SubtractFromAccumulator = 81isize,
    SubtractFromAccumulatorWithBorrow = 82isize,
    SubtractImmediateFromAccumulator = 83isize,
    SubtractImmediateFromAccumulatorWithBorrow = 84isize,
    SwapRegister = 85isize,
    TestBit = 86isize,
}
const NUM_INSTRUCTIONS: usize = 87usize;
impl LR35902Instruction {
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
            0x08 => Some(Self::StoreSpDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
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
            0x10 => match (0x10 as u16) << 8 | stream.read_u8()? as u16 {
                0x1000 => Some(Self::HaltUntilButtonPress),
                _ => None,
            },
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
            0x18 => Some(Self::JumpRelative {
                data1: stream.read_u8().unwrap(),
            }),
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
            0x20 => Some(Self::JumpRelativeIfNotZero {
                data1: stream.read_u8().unwrap(),
            }),
            0x21 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::H,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x22 => Some(Self::MoveAndIncrementHl {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
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
            0x28 => Some(Self::JumpRelativeIfZero {
                data1: stream.read_u8().unwrap(),
            }),
            0x29 => Some(Self::DoubleAdd {
                register1: Intel8080Register::H,
            }),
            0x2A => Some(Self::MoveAndIncrementHl {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
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
            0x30 => Some(Self::JumpRelativeIfNoCarry {
                data1: stream.read_u8().unwrap(),
            }),
            0x31 => Some(Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::SP,
                data2: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0x32 => Some(Self::MoveAndDecrementHl {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
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
            0x38 => Some(Self::JumpRelativeIfCarry {
                data1: stream.read_u8().unwrap(),
            }),
            0x39 => Some(Self::DoubleAdd {
                register1: Intel8080Register::SP,
            }),
            0x3A => Some(Self::MoveAndDecrementHl {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
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
            0xCB => match (0xCB as u16) << 8 | stream.read_u8()? as u16 {
                0xCB00 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::B,
                }),
                0xCB01 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::C,
                }),
                0xCB02 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::D,
                }),
                0xCB03 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::E,
                }),
                0xCB04 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::H,
                }),
                0xCB05 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::L,
                }),
                0xCB06 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::M,
                }),
                0xCB07 => Some(Self::RotateRegisterLeft {
                    register1: Intel8080Register::A,
                }),
                0xCB08 => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::B,
                }),
                0xCB09 => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::C,
                }),
                0xCB0A => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::D,
                }),
                0xCB0B => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::E,
                }),
                0xCB0C => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::H,
                }),
                0xCB0D => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::L,
                }),
                0xCB0E => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::M,
                }),
                0xCB0F => Some(Self::RotateRegisterRight {
                    register1: Intel8080Register::A,
                }),
                0xCB10 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::B,
                }),
                0xCB11 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::C,
                }),
                0xCB12 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::D,
                }),
                0xCB13 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::E,
                }),
                0xCB14 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::H,
                }),
                0xCB15 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::L,
                }),
                0xCB16 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::M,
                }),
                0xCB17 => Some(Self::RotateRegisterLeftThroughCarry {
                    register1: Intel8080Register::A,
                }),
                0xCB18 => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::B,
                }),
                0xCB19 => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::C,
                }),
                0xCB1A => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::D,
                }),
                0xCB1B => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::E,
                }),
                0xCB1C => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::H,
                }),
                0xCB1D => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::L,
                }),
                0xCB1E => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::M,
                }),
                0xCB1F => Some(Self::RotateRegisterRightThroughCarry {
                    register1: Intel8080Register::A,
                }),
                0xCB20 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::B,
                }),
                0xCB21 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::C,
                }),
                0xCB22 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::D,
                }),
                0xCB23 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::E,
                }),
                0xCB24 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::H,
                }),
                0xCB25 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::L,
                }),
                0xCB26 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::M,
                }),
                0xCB27 => Some(Self::ShiftRegisterLeft {
                    register1: Intel8080Register::A,
                }),
                0xCB28 => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::B,
                }),
                0xCB29 => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::C,
                }),
                0xCB2A => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::D,
                }),
                0xCB2B => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::E,
                }),
                0xCB2C => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::H,
                }),
                0xCB2D => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::L,
                }),
                0xCB2E => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::M,
                }),
                0xCB2F => Some(Self::ShiftRegisterRightSigned {
                    register1: Intel8080Register::A,
                }),
                0xCB30 => Some(Self::SwapRegister {
                    register1: Intel8080Register::B,
                }),
                0xCB31 => Some(Self::SwapRegister {
                    register1: Intel8080Register::C,
                }),
                0xCB32 => Some(Self::SwapRegister {
                    register1: Intel8080Register::D,
                }),
                0xCB33 => Some(Self::SwapRegister {
                    register1: Intel8080Register::E,
                }),
                0xCB34 => Some(Self::SwapRegister {
                    register1: Intel8080Register::H,
                }),
                0xCB35 => Some(Self::SwapRegister {
                    register1: Intel8080Register::L,
                }),
                0xCB36 => Some(Self::SwapRegister {
                    register1: Intel8080Register::M,
                }),
                0xCB37 => Some(Self::SwapRegister {
                    register1: Intel8080Register::A,
                }),
                0xCB38 => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::B,
                }),
                0xCB39 => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::C,
                }),
                0xCB3A => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::D,
                }),
                0xCB3B => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::E,
                }),
                0xCB3C => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::H,
                }),
                0xCB3D => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::L,
                }),
                0xCB3E => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::M,
                }),
                0xCB3F => Some(Self::ShiftRegisterRight {
                    register1: Intel8080Register::A,
                }),
                0xCB40 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::B,
                }),
                0xCB41 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::C,
                }),
                0xCB42 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::D,
                }),
                0xCB43 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::E,
                }),
                0xCB44 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::H,
                }),
                0xCB45 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::L,
                }),
                0xCB46 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::M,
                }),
                0xCB47 => Some(Self::TestBit {
                    data1: 0u8,
                    register2: Intel8080Register::A,
                }),
                0xCB48 => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::B,
                }),
                0xCB49 => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::C,
                }),
                0xCB4A => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::D,
                }),
                0xCB4B => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::E,
                }),
                0xCB4C => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::H,
                }),
                0xCB4D => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::L,
                }),
                0xCB4E => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::M,
                }),
                0xCB4F => Some(Self::TestBit {
                    data1: 1u8,
                    register2: Intel8080Register::A,
                }),
                0xCB50 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::B,
                }),
                0xCB51 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::C,
                }),
                0xCB52 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::D,
                }),
                0xCB53 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::E,
                }),
                0xCB54 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::H,
                }),
                0xCB55 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::L,
                }),
                0xCB56 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::M,
                }),
                0xCB57 => Some(Self::TestBit {
                    data1: 2u8,
                    register2: Intel8080Register::A,
                }),
                0xCB58 => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::B,
                }),
                0xCB59 => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::C,
                }),
                0xCB5A => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::D,
                }),
                0xCB5B => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::E,
                }),
                0xCB5C => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::H,
                }),
                0xCB5D => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::L,
                }),
                0xCB5E => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::M,
                }),
                0xCB5F => Some(Self::TestBit {
                    data1: 3u8,
                    register2: Intel8080Register::A,
                }),
                0xCB60 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::B,
                }),
                0xCB61 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::C,
                }),
                0xCB62 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::D,
                }),
                0xCB63 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::E,
                }),
                0xCB64 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::H,
                }),
                0xCB65 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::L,
                }),
                0xCB66 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::M,
                }),
                0xCB67 => Some(Self::TestBit {
                    data1: 4u8,
                    register2: Intel8080Register::A,
                }),
                0xCB68 => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::B,
                }),
                0xCB69 => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::C,
                }),
                0xCB6A => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::D,
                }),
                0xCB6B => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::E,
                }),
                0xCB6C => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::H,
                }),
                0xCB6D => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::L,
                }),
                0xCB6E => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::M,
                }),
                0xCB6F => Some(Self::TestBit {
                    data1: 5u8,
                    register2: Intel8080Register::A,
                }),
                0xCB70 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::B,
                }),
                0xCB71 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::C,
                }),
                0xCB72 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::D,
                }),
                0xCB73 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::E,
                }),
                0xCB74 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::H,
                }),
                0xCB75 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::L,
                }),
                0xCB76 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::M,
                }),
                0xCB77 => Some(Self::TestBit {
                    data1: 6u8,
                    register2: Intel8080Register::A,
                }),
                0xCB78 => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::B,
                }),
                0xCB79 => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::C,
                }),
                0xCB7A => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::D,
                }),
                0xCB7B => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::E,
                }),
                0xCB7C => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::H,
                }),
                0xCB7D => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::L,
                }),
                0xCB7E => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::M,
                }),
                0xCB7F => Some(Self::TestBit {
                    data1: 7u8,
                    register2: Intel8080Register::A,
                }),
                0xCB80 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::B,
                }),
                0xCB81 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::C,
                }),
                0xCB82 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::D,
                }),
                0xCB83 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::E,
                }),
                0xCB84 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::H,
                }),
                0xCB85 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::L,
                }),
                0xCB86 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::M,
                }),
                0xCB87 => Some(Self::ResetBit {
                    data1: 0u8,
                    register2: Intel8080Register::A,
                }),
                0xCB88 => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::B,
                }),
                0xCB89 => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::C,
                }),
                0xCB8A => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::D,
                }),
                0xCB8B => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::E,
                }),
                0xCB8C => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::H,
                }),
                0xCB8D => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::L,
                }),
                0xCB8E => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::M,
                }),
                0xCB8F => Some(Self::ResetBit {
                    data1: 1u8,
                    register2: Intel8080Register::A,
                }),
                0xCB90 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::B,
                }),
                0xCB91 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::C,
                }),
                0xCB92 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::D,
                }),
                0xCB93 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::E,
                }),
                0xCB94 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::H,
                }),
                0xCB95 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::L,
                }),
                0xCB96 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::M,
                }),
                0xCB97 => Some(Self::ResetBit {
                    data1: 2u8,
                    register2: Intel8080Register::A,
                }),
                0xCB98 => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::B,
                }),
                0xCB99 => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::C,
                }),
                0xCB9A => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::D,
                }),
                0xCB9B => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::E,
                }),
                0xCB9C => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::H,
                }),
                0xCB9D => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::L,
                }),
                0xCB9E => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::M,
                }),
                0xCB9F => Some(Self::ResetBit {
                    data1: 3u8,
                    register2: Intel8080Register::A,
                }),
                0xCBA0 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::B,
                }),
                0xCBA1 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::C,
                }),
                0xCBA2 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::D,
                }),
                0xCBA3 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::E,
                }),
                0xCBA4 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::H,
                }),
                0xCBA5 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::L,
                }),
                0xCBA6 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::M,
                }),
                0xCBA7 => Some(Self::ResetBit {
                    data1: 4u8,
                    register2: Intel8080Register::A,
                }),
                0xCBA8 => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::B,
                }),
                0xCBA9 => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::C,
                }),
                0xCBAA => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::D,
                }),
                0xCBAB => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::E,
                }),
                0xCBAC => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::H,
                }),
                0xCBAD => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::L,
                }),
                0xCBAE => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::M,
                }),
                0xCBAF => Some(Self::ResetBit {
                    data1: 5u8,
                    register2: Intel8080Register::A,
                }),
                0xCBB0 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::B,
                }),
                0xCBB1 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::C,
                }),
                0xCBB2 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::D,
                }),
                0xCBB3 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::E,
                }),
                0xCBB4 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::H,
                }),
                0xCBB5 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::L,
                }),
                0xCBB6 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::M,
                }),
                0xCBB7 => Some(Self::ResetBit {
                    data1: 6u8,
                    register2: Intel8080Register::A,
                }),
                0xCBB8 => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::B,
                }),
                0xCBB9 => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::C,
                }),
                0xCBBA => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::D,
                }),
                0xCBBB => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::E,
                }),
                0xCBBC => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::H,
                }),
                0xCBBD => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::L,
                }),
                0xCBBE => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::M,
                }),
                0xCBBF => Some(Self::ResetBit {
                    data1: 7u8,
                    register2: Intel8080Register::A,
                }),
                0xCBC0 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::B,
                }),
                0xCBC1 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::C,
                }),
                0xCBC2 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::D,
                }),
                0xCBC3 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::E,
                }),
                0xCBC4 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::H,
                }),
                0xCBC5 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::L,
                }),
                0xCBC6 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::M,
                }),
                0xCBC7 => Some(Self::SetBit {
                    data1: 0u8,
                    register2: Intel8080Register::A,
                }),
                0xCBC8 => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::B,
                }),
                0xCBC9 => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::C,
                }),
                0xCBCA => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::D,
                }),
                0xCBCB => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::E,
                }),
                0xCBCC => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::H,
                }),
                0xCBCD => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::L,
                }),
                0xCBCE => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::M,
                }),
                0xCBCF => Some(Self::SetBit {
                    data1: 1u8,
                    register2: Intel8080Register::A,
                }),
                0xCBD0 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::B,
                }),
                0xCBD1 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::C,
                }),
                0xCBD2 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::D,
                }),
                0xCBD3 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::E,
                }),
                0xCBD4 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::H,
                }),
                0xCBD5 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::L,
                }),
                0xCBD6 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::M,
                }),
                0xCBD7 => Some(Self::SetBit {
                    data1: 2u8,
                    register2: Intel8080Register::A,
                }),
                0xCBD8 => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::B,
                }),
                0xCBD9 => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::C,
                }),
                0xCBDA => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::D,
                }),
                0xCBDB => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::E,
                }),
                0xCBDC => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::H,
                }),
                0xCBDD => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::L,
                }),
                0xCBDE => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::M,
                }),
                0xCBDF => Some(Self::SetBit {
                    data1: 3u8,
                    register2: Intel8080Register::A,
                }),
                0xCBE0 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::B,
                }),
                0xCBE1 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::C,
                }),
                0xCBE2 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::D,
                }),
                0xCBE3 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::E,
                }),
                0xCBE4 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::H,
                }),
                0xCBE5 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::L,
                }),
                0xCBE6 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::M,
                }),
                0xCBE7 => Some(Self::SetBit {
                    data1: 4u8,
                    register2: Intel8080Register::A,
                }),
                0xCBE8 => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::B,
                }),
                0xCBE9 => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::C,
                }),
                0xCBEA => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::D,
                }),
                0xCBEB => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::E,
                }),
                0xCBEC => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::H,
                }),
                0xCBED => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::L,
                }),
                0xCBEE => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::M,
                }),
                0xCBEF => Some(Self::SetBit {
                    data1: 5u8,
                    register2: Intel8080Register::A,
                }),
                0xCBF0 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::B,
                }),
                0xCBF1 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::C,
                }),
                0xCBF2 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::D,
                }),
                0xCBF3 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::E,
                }),
                0xCBF4 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::H,
                }),
                0xCBF5 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::L,
                }),
                0xCBF6 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::M,
                }),
                0xCBF7 => Some(Self::SetBit {
                    data1: 6u8,
                    register2: Intel8080Register::A,
                }),
                0xCBF8 => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::B,
                }),
                0xCBF9 => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::C,
                }),
                0xCBFA => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::D,
                }),
                0xCBFB => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::E,
                }),
                0xCBFC => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::H,
                }),
                0xCBFD => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::L,
                }),
                0xCBFE => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::M,
                }),
                0xCBFF => Some(Self::SetBit {
                    data1: 7u8,
                    register2: Intel8080Register::A,
                }),
                _ => None,
            },
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
            0xD9 => Some(Self::ReturnAndEnableInterrupts),
            0xDA => Some(Self::JumpIfCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xDC => Some(Self::CallIfCarry {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xDE => Some(Self::SubtractImmediateFromAccumulatorWithBorrow {
                data1: stream.read_u8().unwrap(),
            }),
            0xDF => Some(Self::Restart { data1: 3u8 }),
            0xE0 => Some(Self::StoreAccumulatorDirectOneByte {
                data1: stream.read_u8().unwrap(),
            }),
            0xE1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::H,
            }),
            0xE2 => Some(Self::StoreAccumulatorOneByte),
            0xE5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::H,
            }),
            0xE6 => Some(Self::AndImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xE7 => Some(Self::Restart { data1: 4u8 }),
            0xE8 => Some(Self::AddImmediateToSp {
                data1: stream.read_u8().unwrap(),
            }),
            0xE9 => Some(Self::LoadProgramCounter),
            0xEA => Some(Self::StoreAccumulatorDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xEE => Some(Self::ExclusiveOrImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xEF => Some(Self::Restart { data1: 5u8 }),
            0xF0 => Some(Self::LoadAccumulatorDirectOneByte {
                data1: stream.read_u8().unwrap(),
            }),
            0xF1 => Some(Self::PopDataOffStack {
                register1: Intel8080Register::PSW,
            }),
            0xF2 => Some(Self::LoadAccumulatorOneByte),
            0xF3 => Some(Self::DisableInterrupts),
            0xF5 => Some(Self::PushDataOntoStack {
                register1: Intel8080Register::PSW,
            }),
            0xF6 => Some(Self::OrImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xF7 => Some(Self::Restart { data1: 6u8 }),
            0xF8 => Some(Self::StoreSpPlusImmediate {
                data1: stream.read_u8().unwrap(),
            }),
            0xF9 => Some(Self::LoadSpFromHAndL),
            0xFA => Some(Self::LoadAccumulatorDirect {
                address1: stream.read_u16::<LittleEndian>().unwrap(),
            }),
            0xFB => Some(Self::EnableInterrupts),
            0xFE => Some(Self::CompareImmediateWithAccumulator {
                data1: stream.read_u8().unwrap(),
            }),
            0xFF => Some(Self::Restart { data1: 7u8 }),
            _ => None,
        })
    }
    pub fn to_type(&self) -> LR35902InstructionType {
        match self {
            Self::AddImmediateToAccumulator { .. } => {
                LR35902InstructionType::AddImmediateToAccumulator
            }
            Self::AddImmediateToAccumulatorWithCarry { .. } => {
                LR35902InstructionType::AddImmediateToAccumulatorWithCarry
            }
            Self::AddImmediateToSp { .. } => LR35902InstructionType::AddImmediateToSp,
            Self::AddToAccumulator { .. } => LR35902InstructionType::AddToAccumulator,
            Self::AddToAccumulatorWithCarry { .. } => {
                LR35902InstructionType::AddToAccumulatorWithCarry
            }
            Self::AndImmediateWithAccumulator { .. } => {
                LR35902InstructionType::AndImmediateWithAccumulator
            }
            Self::Call { .. } => LR35902InstructionType::Call,
            Self::CallIfCarry { .. } => LR35902InstructionType::CallIfCarry,
            Self::CallIfNoCarry { .. } => LR35902InstructionType::CallIfNoCarry,
            Self::CallIfNotZero { .. } => LR35902InstructionType::CallIfNotZero,
            Self::CallIfZero { .. } => LR35902InstructionType::CallIfZero,
            Self::CompareImmediateWithAccumulator { .. } => {
                LR35902InstructionType::CompareImmediateWithAccumulator
            }
            Self::CompareWithAccumulator { .. } => LR35902InstructionType::CompareWithAccumulator,
            Self::ComplementAccumulator => LR35902InstructionType::ComplementAccumulator,
            Self::ComplementCarry => LR35902InstructionType::ComplementCarry,
            Self::DecimalAdjustAccumulator => LR35902InstructionType::DecimalAdjustAccumulator,
            Self::DecrementRegisterOrMemory { .. } => {
                LR35902InstructionType::DecrementRegisterOrMemory
            }
            Self::DecrementRegisterPair { .. } => LR35902InstructionType::DecrementRegisterPair,
            Self::DisableInterrupts => LR35902InstructionType::DisableInterrupts,
            Self::DoubleAdd { .. } => LR35902InstructionType::DoubleAdd,
            Self::EnableInterrupts => LR35902InstructionType::EnableInterrupts,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => {
                LR35902InstructionType::ExclusiveOrImmediateWithAccumulator
            }
            Self::Halt => LR35902InstructionType::Halt,
            Self::HaltUntilButtonPress => LR35902InstructionType::HaltUntilButtonPress,
            Self::IncrementRegisterOrMemory { .. } => {
                LR35902InstructionType::IncrementRegisterOrMemory
            }
            Self::IncrementRegisterPair { .. } => LR35902InstructionType::IncrementRegisterPair,
            Self::Jump { .. } => LR35902InstructionType::Jump,
            Self::JumpIfCarry { .. } => LR35902InstructionType::JumpIfCarry,
            Self::JumpIfNoCarry { .. } => LR35902InstructionType::JumpIfNoCarry,
            Self::JumpIfNotZero { .. } => LR35902InstructionType::JumpIfNotZero,
            Self::JumpIfZero { .. } => LR35902InstructionType::JumpIfZero,
            Self::JumpRelative { .. } => LR35902InstructionType::JumpRelative,
            Self::JumpRelativeIfCarry { .. } => LR35902InstructionType::JumpRelativeIfCarry,
            Self::JumpRelativeIfNoCarry { .. } => LR35902InstructionType::JumpRelativeIfNoCarry,
            Self::JumpRelativeIfNotZero { .. } => LR35902InstructionType::JumpRelativeIfNotZero,
            Self::JumpRelativeIfZero { .. } => LR35902InstructionType::JumpRelativeIfZero,
            Self::LoadAccumulator { .. } => LR35902InstructionType::LoadAccumulator,
            Self::LoadAccumulatorDirect { .. } => LR35902InstructionType::LoadAccumulatorDirect,
            Self::LoadAccumulatorDirectOneByte { .. } => {
                LR35902InstructionType::LoadAccumulatorDirectOneByte
            }
            Self::LoadAccumulatorOneByte => LR35902InstructionType::LoadAccumulatorOneByte,
            Self::LoadProgramCounter => LR35902InstructionType::LoadProgramCounter,
            Self::LoadRegisterPairImmediate { .. } => {
                LR35902InstructionType::LoadRegisterPairImmediate
            }
            Self::LoadSpFromHAndL => LR35902InstructionType::LoadSpFromHAndL,
            Self::LogicalAndWithAccumulator { .. } => {
                LR35902InstructionType::LogicalAndWithAccumulator
            }
            Self::LogicalExclusiveOrWithAccumulator { .. } => {
                LR35902InstructionType::LogicalExclusiveOrWithAccumulator
            }
            Self::LogicalOrWithAccumulator { .. } => {
                LR35902InstructionType::LogicalOrWithAccumulator
            }
            Self::MoveAndDecrementHl { .. } => LR35902InstructionType::MoveAndDecrementHl,
            Self::MoveAndIncrementHl { .. } => LR35902InstructionType::MoveAndIncrementHl,
            Self::MoveData { .. } => LR35902InstructionType::MoveData,
            Self::MoveImmediateData { .. } => LR35902InstructionType::MoveImmediateData,
            Self::NoOperation => LR35902InstructionType::NoOperation,
            Self::OrImmediateWithAccumulator { .. } => {
                LR35902InstructionType::OrImmediateWithAccumulator
            }
            Self::PopDataOffStack { .. } => LR35902InstructionType::PopDataOffStack,
            Self::PushDataOntoStack { .. } => LR35902InstructionType::PushDataOntoStack,
            Self::ResetBit { .. } => LR35902InstructionType::ResetBit,
            Self::Restart { .. } => LR35902InstructionType::Restart,
            Self::ReturnAndEnableInterrupts => LR35902InstructionType::ReturnAndEnableInterrupts,
            Self::ReturnIfCarry => LR35902InstructionType::ReturnIfCarry,
            Self::ReturnIfNoCarry => LR35902InstructionType::ReturnIfNoCarry,
            Self::ReturnIfNotZero => LR35902InstructionType::ReturnIfNotZero,
            Self::ReturnIfZero => LR35902InstructionType::ReturnIfZero,
            Self::ReturnUnconditionally => LR35902InstructionType::ReturnUnconditionally,
            Self::RotateAccumulatorLeft => LR35902InstructionType::RotateAccumulatorLeft,
            Self::RotateAccumulatorLeftThroughCarry => {
                LR35902InstructionType::RotateAccumulatorLeftThroughCarry
            }
            Self::RotateAccumulatorRight => LR35902InstructionType::RotateAccumulatorRight,
            Self::RotateAccumulatorRightThroughCarry => {
                LR35902InstructionType::RotateAccumulatorRightThroughCarry
            }
            Self::RotateRegisterLeft { .. } => LR35902InstructionType::RotateRegisterLeft,
            Self::RotateRegisterLeftThroughCarry { .. } => {
                LR35902InstructionType::RotateRegisterLeftThroughCarry
            }
            Self::RotateRegisterRight { .. } => LR35902InstructionType::RotateRegisterRight,
            Self::RotateRegisterRightThroughCarry { .. } => {
                LR35902InstructionType::RotateRegisterRightThroughCarry
            }
            Self::SetBit { .. } => LR35902InstructionType::SetBit,
            Self::SetCarry => LR35902InstructionType::SetCarry,
            Self::ShiftRegisterLeft { .. } => LR35902InstructionType::ShiftRegisterLeft,
            Self::ShiftRegisterRight { .. } => LR35902InstructionType::ShiftRegisterRight,
            Self::ShiftRegisterRightSigned { .. } => {
                LR35902InstructionType::ShiftRegisterRightSigned
            }
            Self::StoreAccumulator { .. } => LR35902InstructionType::StoreAccumulator,
            Self::StoreAccumulatorDirect { .. } => LR35902InstructionType::StoreAccumulatorDirect,
            Self::StoreAccumulatorDirectOneByte { .. } => {
                LR35902InstructionType::StoreAccumulatorDirectOneByte
            }
            Self::StoreAccumulatorOneByte => LR35902InstructionType::StoreAccumulatorOneByte,
            Self::StoreSpDirect { .. } => LR35902InstructionType::StoreSpDirect,
            Self::StoreSpPlusImmediate { .. } => LR35902InstructionType::StoreSpPlusImmediate,
            Self::SubtractFromAccumulator { .. } => LR35902InstructionType::SubtractFromAccumulator,
            Self::SubtractFromAccumulatorWithBorrow { .. } => {
                LR35902InstructionType::SubtractFromAccumulatorWithBorrow
            }
            Self::SubtractImmediateFromAccumulator { .. } => {
                LR35902InstructionType::SubtractImmediateFromAccumulator
            }
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => {
                LR35902InstructionType::SubtractImmediateFromAccumulatorWithBorrow
            }
            Self::SwapRegister { .. } => LR35902InstructionType::SwapRegister,
            Self::TestBit { .. } => LR35902InstructionType::TestBit,
        }
    }
}
impl LR35902Instruction {
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
            Self::StoreSpDirect { .. } => 3u8,
            Self::DoubleAdd { .. } => 1u8,
            Self::LoadAccumulator { .. } => 1u8,
            Self::DecrementRegisterPair { .. } => 1u8,
            Self::RotateAccumulatorRight { .. } => 1u8,
            Self::HaltUntilButtonPress { .. } => 2u8,
            Self::RotateAccumulatorLeftThroughCarry { .. } => 1u8,
            Self::JumpRelative { .. } => 2u8,
            Self::RotateAccumulatorRightThroughCarry { .. } => 1u8,
            Self::JumpRelativeIfNotZero { .. } => 2u8,
            Self::MoveAndIncrementHl { .. } => 1u8,
            Self::DecimalAdjustAccumulator { .. } => 1u8,
            Self::JumpRelativeIfZero { .. } => 2u8,
            Self::ComplementAccumulator { .. } => 1u8,
            Self::JumpRelativeIfNoCarry { .. } => 2u8,
            Self::MoveAndDecrementHl { .. } => 1u8,
            Self::SetCarry { .. } => 1u8,
            Self::JumpRelativeIfCarry { .. } => 2u8,
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
            Self::RotateRegisterLeft { .. } => 2u8,
            Self::RotateRegisterRight { .. } => 2u8,
            Self::RotateRegisterLeftThroughCarry { .. } => 2u8,
            Self::RotateRegisterRightThroughCarry { .. } => 2u8,
            Self::ShiftRegisterLeft { .. } => 2u8,
            Self::ShiftRegisterRightSigned { .. } => 2u8,
            Self::SwapRegister { .. } => 2u8,
            Self::ShiftRegisterRight { .. } => 2u8,
            Self::TestBit { .. } => 2u8,
            Self::ResetBit { .. } => 2u8,
            Self::SetBit { .. } => 2u8,
            Self::CallIfZero { .. } => 3u8,
            Self::Call { .. } => 3u8,
            Self::AddImmediateToAccumulatorWithCarry { .. } => 2u8,
            Self::ReturnIfNoCarry { .. } => 1u8,
            Self::JumpIfNoCarry { .. } => 3u8,
            Self::CallIfNoCarry { .. } => 3u8,
            Self::SubtractImmediateFromAccumulator { .. } => 2u8,
            Self::ReturnIfCarry { .. } => 1u8,
            Self::ReturnAndEnableInterrupts { .. } => 1u8,
            Self::JumpIfCarry { .. } => 3u8,
            Self::CallIfCarry { .. } => 3u8,
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => 2u8,
            Self::StoreAccumulatorDirectOneByte { .. } => 2u8,
            Self::StoreAccumulatorOneByte { .. } => 1u8,
            Self::AndImmediateWithAccumulator { .. } => 2u8,
            Self::AddImmediateToSp { .. } => 2u8,
            Self::LoadProgramCounter { .. } => 1u8,
            Self::StoreAccumulatorDirect { .. } => 3u8,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => 2u8,
            Self::LoadAccumulatorDirectOneByte { .. } => 2u8,
            Self::LoadAccumulatorOneByte { .. } => 1u8,
            Self::DisableInterrupts { .. } => 1u8,
            Self::OrImmediateWithAccumulator { .. } => 2u8,
            Self::StoreSpPlusImmediate { .. } => 2u8,
            Self::LoadSpFromHAndL { .. } => 1u8,
            Self::LoadAccumulatorDirect { .. } => 3u8,
            Self::EnableInterrupts { .. } => 1u8,
            Self::CompareImmediateWithAccumulator { .. } => 2u8,
        }
    }
}
impl LR35902Instruction {
    pub fn duration(&self) -> u8 {
        match self {
            Self::NoOperation { .. } => 4u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::B,
                ..
            } => 12u8,
            Self::StoreAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::RotateAccumulatorLeft { .. } => 4u8,
            Self::StoreSpDirect { .. } => 20u8,
            Self::DoubleAdd {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::LoadAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::RotateAccumulatorRight { .. } => 4u8,
            Self::HaltUntilButtonPress { .. } => 4u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::D,
                ..
            } => 12u8,
            Self::StoreAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::RotateAccumulatorLeftThroughCarry { .. } => 4u8,
            Self::JumpRelative { .. } => 12u8,
            Self::DoubleAdd {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::LoadAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::RotateAccumulatorRightThroughCarry { .. } => 4u8,
            Self::JumpRelativeIfNotZero { .. } => 8u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::H,
                ..
            } => 12u8,
            Self::MoveAndIncrementHl {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::DecimalAdjustAccumulator { .. } => 4u8,
            Self::JumpRelativeIfZero { .. } => 8u8,
            Self::DoubleAdd {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::MoveAndIncrementHl {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ComplementAccumulator { .. } => 4u8,
            Self::JumpRelativeIfNoCarry { .. } => 8u8,
            Self::LoadRegisterPairImmediate {
                register1: Intel8080Register::SP,
                ..
            } => 12u8,
            Self::MoveAndDecrementHl {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::IncrementRegisterPair {
                register1: Intel8080Register::SP,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::M,
                ..
            } => 12u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::M,
                ..
            } => 12u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::M,
                ..
            } => 12u8,
            Self::SetCarry { .. } => 4u8,
            Self::JumpRelativeIfCarry { .. } => 8u8,
            Self::DoubleAdd {
                register1: Intel8080Register::SP,
                ..
            } => 8u8,
            Self::MoveAndDecrementHl {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::DecrementRegisterPair {
                register1: Intel8080Register::SP,
                ..
            } => 8u8,
            Self::IncrementRegisterOrMemory {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::DecrementRegisterOrMemory {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveImmediateData {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ComplementCarry { .. } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::B,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::C,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::D,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::E,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::H,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::L,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::Halt { .. } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::M,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::B,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::C,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::D,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::E,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::H,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::L,
                ..
            } => 4u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::M,
                ..
            } => 8u8,
            Self::MoveData {
                register1: Intel8080Register::A,
                register2: Intel8080Register::A,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::AddToAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::AddToAccumulatorWithCarry {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::SubtractFromAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::SubtractFromAccumulatorWithBorrow {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::LogicalAndWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::LogicalExclusiveOrWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::LogicalOrWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::B,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::C,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::D,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::E,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::H,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::L,
                ..
            } => 4u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::M,
                ..
            } => 8u8,
            Self::CompareWithAccumulator {
                register1: Intel8080Register::A,
                ..
            } => 4u8,
            Self::ReturnIfNotZero { .. } => 8u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::B,
                ..
            } => 12u8,
            Self::JumpIfNotZero { .. } => 12u8,
            Self::Jump { .. } => 16u8,
            Self::CallIfNotZero { .. } => 12u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::B,
                ..
            } => 16u8,
            Self::AddImmediateToAccumulator { .. } => 8u8,
            Self::Restart { data1: 0u8, .. } => 16u8,
            Self::ReturnIfZero { .. } => 8u8,
            Self::ReturnUnconditionally { .. } => 16u8,
            Self::JumpIfZero { .. } => 12u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::RotateRegisterLeft {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::RotateRegisterRight {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::RotateRegisterLeftThroughCarry {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::RotateRegisterRightThroughCarry {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ShiftRegisterLeft {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ShiftRegisterRightSigned {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SwapRegister {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SwapRegister {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ShiftRegisterRight {
                register1: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 0u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 1u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 2u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 3u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 4u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 5u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 6u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::M,
                ..
            } => 12u8,
            Self::TestBit {
                data1: 7u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 0u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 1u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 2u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 3u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 4u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 5u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 6u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::ResetBit {
                data1: 7u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 0u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 1u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 2u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 3u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 4u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 5u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 6u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::B,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::C,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::D,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::E,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::H,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::L,
                ..
            } => 8u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::M,
                ..
            } => 16u8,
            Self::SetBit {
                data1: 7u8,
                register2: Intel8080Register::A,
                ..
            } => 8u8,
            Self::CallIfZero { .. } => 12u8,
            Self::Call { .. } => 24u8,
            Self::AddImmediateToAccumulatorWithCarry { .. } => 8u8,
            Self::Restart { data1: 1u8, .. } => 16u8,
            Self::ReturnIfNoCarry { .. } => 8u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::D,
                ..
            } => 12u8,
            Self::JumpIfNoCarry { .. } => 12u8,
            Self::CallIfNoCarry { .. } => 12u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::D,
                ..
            } => 16u8,
            Self::SubtractImmediateFromAccumulator { .. } => 8u8,
            Self::Restart { data1: 2u8, .. } => 16u8,
            Self::ReturnIfCarry { .. } => 8u8,
            Self::ReturnAndEnableInterrupts { .. } => 16u8,
            Self::JumpIfCarry { .. } => 12u8,
            Self::CallIfCarry { .. } => 12u8,
            Self::SubtractImmediateFromAccumulatorWithBorrow { .. } => 8u8,
            Self::Restart { data1: 3u8, .. } => 16u8,
            Self::StoreAccumulatorDirectOneByte { .. } => 12u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::H,
                ..
            } => 12u8,
            Self::StoreAccumulatorOneByte { .. } => 8u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::H,
                ..
            } => 16u8,
            Self::AndImmediateWithAccumulator { .. } => 8u8,
            Self::Restart { data1: 4u8, .. } => 16u8,
            Self::AddImmediateToSp { .. } => 16u8,
            Self::LoadProgramCounter { .. } => 4u8,
            Self::StoreAccumulatorDirect { .. } => 16u8,
            Self::ExclusiveOrImmediateWithAccumulator { .. } => 8u8,
            Self::Restart { data1: 5u8, .. } => 16u8,
            Self::LoadAccumulatorDirectOneByte { .. } => 12u8,
            Self::PopDataOffStack {
                register1: Intel8080Register::PSW,
                ..
            } => 12u8,
            Self::LoadAccumulatorOneByte { .. } => 8u8,
            Self::DisableInterrupts { .. } => 4u8,
            Self::PushDataOntoStack {
                register1: Intel8080Register::PSW,
                ..
            } => 16u8,
            Self::OrImmediateWithAccumulator { .. } => 8u8,
            Self::Restart { data1: 6u8, .. } => 16u8,
            Self::StoreSpPlusImmediate { .. } => 12u8,
            Self::LoadSpFromHAndL { .. } => 8u8,
            Self::LoadAccumulatorDirect { .. } => 16u8,
            Self::EnableInterrupts { .. } => 4u8,
            Self::CompareImmediateWithAccumulator { .. } => 8u8,
            Self::Restart { data1: 7u8, .. } => 16u8,
            instr => panic!("invalid instruction {:?}", instr),
        }
    }
}
pub trait LR35902InstructionSet {
    fn add_immediate_to_accumulator(&mut self, data1: u8);
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8);
    fn add_immediate_to_sp(&mut self, data1: u8);
    fn add_to_accumulator(&mut self, register1: Intel8080Register);
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register);
    fn and_immediate_with_accumulator(&mut self, data1: u8);
    fn call(&mut self, address1: u16);
    fn call_if_carry(&mut self, address1: u16);
    fn call_if_no_carry(&mut self, address1: u16);
    fn call_if_not_zero(&mut self, address1: u16);
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
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8);
    fn halt(&mut self);
    fn halt_until_button_press(&mut self);
    fn increment_register_or_memory(&mut self, register1: Intel8080Register);
    fn increment_register_pair(&mut self, register1: Intel8080Register);
    fn jump(&mut self, address1: u16);
    fn jump_if_carry(&mut self, address1: u16);
    fn jump_if_no_carry(&mut self, address1: u16);
    fn jump_if_not_zero(&mut self, address1: u16);
    fn jump_if_zero(&mut self, address1: u16);
    fn jump_relative(&mut self, data1: u8);
    fn jump_relative_if_carry(&mut self, data1: u8);
    fn jump_relative_if_no_carry(&mut self, data1: u8);
    fn jump_relative_if_not_zero(&mut self, data1: u8);
    fn jump_relative_if_zero(&mut self, data1: u8);
    fn load_accumulator(&mut self, register1: Intel8080Register);
    fn load_accumulator_direct(&mut self, address1: u16);
    fn load_accumulator_direct_one_byte(&mut self, data1: u8);
    fn load_accumulator_one_byte(&mut self);
    fn load_program_counter(&mut self);
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16);
    fn load_sp_from_h_and_l(&mut self);
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register);
    fn move_and_decrement_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_and_increment_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register);
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8);
    fn no_operation(&mut self);
    fn or_immediate_with_accumulator(&mut self, data1: u8);
    fn pop_data_off_stack(&mut self, register1: Intel8080Register);
    fn push_data_onto_stack(&mut self, register1: Intel8080Register);
    fn reset_bit(&mut self, data1: u8, register2: Intel8080Register);
    fn restart(&mut self, data1: u8);
    fn return_and_enable_interrupts(&mut self);
    fn return_if_carry(&mut self);
    fn return_if_no_carry(&mut self);
    fn return_if_not_zero(&mut self);
    fn return_if_zero(&mut self);
    fn return_unconditionally(&mut self);
    fn rotate_accumulator_left(&mut self);
    fn rotate_accumulator_left_through_carry(&mut self);
    fn rotate_accumulator_right(&mut self);
    fn rotate_accumulator_right_through_carry(&mut self);
    fn rotate_register_left(&mut self, register1: Intel8080Register);
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register);
    fn rotate_register_right(&mut self, register1: Intel8080Register);
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register);
    fn set_bit(&mut self, data1: u8, register2: Intel8080Register);
    fn set_carry(&mut self);
    fn shift_register_left(&mut self, register1: Intel8080Register);
    fn shift_register_right(&mut self, register1: Intel8080Register);
    fn shift_register_right_signed(&mut self, register1: Intel8080Register);
    fn store_accumulator(&mut self, register1: Intel8080Register);
    fn store_accumulator_direct(&mut self, address1: u16);
    fn store_accumulator_direct_one_byte(&mut self, data1: u8);
    fn store_accumulator_one_byte(&mut self);
    fn store_sp_direct(&mut self, address1: u16);
    fn store_sp_plus_immediate(&mut self, data1: u8);
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register);
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register);
    fn subtract_immediate_from_accumulator(&mut self, data1: u8);
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8);
    fn swap_register(&mut self, register1: Intel8080Register);
    fn test_bit(&mut self, data1: u8, register2: Intel8080Register);
}
impl LR35902Instruction {
    #[inline(always)]
    pub fn dispatch<I: LR35902InstructionSet>(self, machine: &mut I) {
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
            Self::StoreSpDirect { address1 } => machine.store_sp_direct(address1),
            Self::DoubleAdd { register1 } => machine.double_add(register1),
            Self::LoadAccumulator { register1 } => machine.load_accumulator(register1),
            Self::DecrementRegisterPair { register1 } => machine.decrement_register_pair(register1),
            Self::RotateAccumulatorRight {} => machine.rotate_accumulator_right(),
            Self::HaltUntilButtonPress {} => machine.halt_until_button_press(),
            Self::RotateAccumulatorLeftThroughCarry {} => {
                machine.rotate_accumulator_left_through_carry()
            }
            Self::JumpRelative { data1 } => machine.jump_relative(data1),
            Self::RotateAccumulatorRightThroughCarry {} => {
                machine.rotate_accumulator_right_through_carry()
            }
            Self::JumpRelativeIfNotZero { data1 } => machine.jump_relative_if_not_zero(data1),
            Self::MoveAndIncrementHl {
                register1,
                register2,
            } => machine.move_and_increment_hl(register1, register2),
            Self::DecimalAdjustAccumulator {} => machine.decimal_adjust_accumulator(),
            Self::JumpRelativeIfZero { data1 } => machine.jump_relative_if_zero(data1),
            Self::ComplementAccumulator {} => machine.complement_accumulator(),
            Self::JumpRelativeIfNoCarry { data1 } => machine.jump_relative_if_no_carry(data1),
            Self::MoveAndDecrementHl {
                register1,
                register2,
            } => machine.move_and_decrement_hl(register1, register2),
            Self::SetCarry {} => machine.set_carry(),
            Self::JumpRelativeIfCarry { data1 } => machine.jump_relative_if_carry(data1),
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
            Self::RotateRegisterLeft { register1 } => machine.rotate_register_left(register1),
            Self::RotateRegisterRight { register1 } => machine.rotate_register_right(register1),
            Self::RotateRegisterLeftThroughCarry { register1 } => {
                machine.rotate_register_left_through_carry(register1)
            }
            Self::RotateRegisterRightThroughCarry { register1 } => {
                machine.rotate_register_right_through_carry(register1)
            }
            Self::ShiftRegisterLeft { register1 } => machine.shift_register_left(register1),
            Self::ShiftRegisterRightSigned { register1 } => {
                machine.shift_register_right_signed(register1)
            }
            Self::SwapRegister { register1 } => machine.swap_register(register1),
            Self::ShiftRegisterRight { register1 } => machine.shift_register_right(register1),
            Self::TestBit { data1, register2 } => machine.test_bit(data1, register2),
            Self::ResetBit { data1, register2 } => machine.reset_bit(data1, register2),
            Self::SetBit { data1, register2 } => machine.set_bit(data1, register2),
            Self::CallIfZero { address1 } => machine.call_if_zero(address1),
            Self::Call { address1 } => machine.call(address1),
            Self::AddImmediateToAccumulatorWithCarry { data1 } => {
                machine.add_immediate_to_accumulator_with_carry(data1)
            }
            Self::ReturnIfNoCarry {} => machine.return_if_no_carry(),
            Self::JumpIfNoCarry { address1 } => machine.jump_if_no_carry(address1),
            Self::CallIfNoCarry { address1 } => machine.call_if_no_carry(address1),
            Self::SubtractImmediateFromAccumulator { data1 } => {
                machine.subtract_immediate_from_accumulator(data1)
            }
            Self::ReturnIfCarry {} => machine.return_if_carry(),
            Self::ReturnAndEnableInterrupts {} => machine.return_and_enable_interrupts(),
            Self::JumpIfCarry { address1 } => machine.jump_if_carry(address1),
            Self::CallIfCarry { address1 } => machine.call_if_carry(address1),
            Self::SubtractImmediateFromAccumulatorWithBorrow { data1 } => {
                machine.subtract_immediate_from_accumulator_with_borrow(data1)
            }
            Self::StoreAccumulatorDirectOneByte { data1 } => {
                machine.store_accumulator_direct_one_byte(data1)
            }
            Self::StoreAccumulatorOneByte {} => machine.store_accumulator_one_byte(),
            Self::AndImmediateWithAccumulator { data1 } => {
                machine.and_immediate_with_accumulator(data1)
            }
            Self::AddImmediateToSp { data1 } => machine.add_immediate_to_sp(data1),
            Self::LoadProgramCounter {} => machine.load_program_counter(),
            Self::StoreAccumulatorDirect { address1 } => machine.store_accumulator_direct(address1),
            Self::ExclusiveOrImmediateWithAccumulator { data1 } => {
                machine.exclusive_or_immediate_with_accumulator(data1)
            }
            Self::LoadAccumulatorDirectOneByte { data1 } => {
                machine.load_accumulator_direct_one_byte(data1)
            }
            Self::LoadAccumulatorOneByte {} => machine.load_accumulator_one_byte(),
            Self::DisableInterrupts {} => machine.disable_interrupts(),
            Self::OrImmediateWithAccumulator { data1 } => {
                machine.or_immediate_with_accumulator(data1)
            }
            Self::StoreSpPlusImmediate { data1 } => machine.store_sp_plus_immediate(data1),
            Self::LoadSpFromHAndL {} => machine.load_sp_from_h_and_l(),
            Self::LoadAccumulatorDirect { address1 } => machine.load_accumulator_direct(address1),
            Self::EnableInterrupts {} => machine.enable_interrupts(),
            Self::CompareImmediateWithAccumulator { data1 } => {
                machine.compare_immediate_with_accumulator(data1)
            }
        }
    }
}
impl<'a> LR35902InstructionSet for LR35902InstructionPrinter<'a> {
    fn add_immediate_to_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "ADI");
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "ACI");
    }
    fn add_immediate_to_sp(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "ADDS");
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "ADD");
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "ADC");
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "ANI");
    }
    fn call(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "CALL");
    }
    fn call_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "CC");
    }
    fn call_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "CNC");
    }
    fn call_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "CNZ");
    }
    fn call_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "CZ");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "CPI");
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "CMP");
    }
    fn complement_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CPL");
    }
    fn complement_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "CCF");
    }
    fn decimal_adjust_accumulator(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DAA");
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "DCR");
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "DCX");
    }
    fn disable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "DI");
    }
    fn double_add(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "DAD");
    }
    fn enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "EI");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "XRI");
    }
    fn halt(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "HLT");
    }
    fn halt_until_button_press(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STOP");
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "INR");
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "INX");
    }
    fn jump(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "JMP");
    }
    fn jump_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "JC");
    }
    fn jump_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "JNC");
    }
    fn jump_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "JNZ");
    }
    fn jump_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "JZ");
    }
    fn jump_relative(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "JR");
    }
    fn jump_relative_if_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "JRC");
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "JRNC");
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "JRNZ");
    }
    fn jump_relative_if_zero(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "JRZ");
    }
    fn load_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "LDAX");
    }
    fn load_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "LDAD");
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "LDAB");
    }
    fn load_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "LDAC");
    }
    fn load_program_counter(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "PCHL");
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16) {
        self.error = write!(self.stream_out, "{:04} {register1:?} #${data2:02x}", "LXI");
    }
    fn load_sp_from_h_and_l(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SPHL");
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "ANA");
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "XRA");
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "ORA");
    }
    fn move_and_decrement_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(self.stream_out, "{:04} {register1:?} {register2:?}", "MVM-");
    }
    fn move_and_increment_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(self.stream_out, "{:04} {register1:?} {register2:?}", "MVM+");
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?} {register2:?}", "MOV");
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8) {
        self.error = write!(self.stream_out, "{:04} {register1:?} #${data2:02x}", "MVI");
    }
    fn no_operation(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "NOP");
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "ORI");
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "POP");
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "PUSH");
    }
    fn reset_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {data1} {register2:?}", "RES");
    }
    fn restart(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} {data1}", "RST");
    }
    fn return_and_enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RETI");
    }
    fn return_if_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RC");
    }
    fn return_if_no_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNC");
    }
    fn return_if_not_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RNZ");
    }
    fn return_if_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RZ");
    }
    fn return_unconditionally(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "RET");
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
    fn rotate_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "RLC");
    }
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "RL");
    }
    fn rotate_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "RRC");
    }
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "RR");
    }
    fn set_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {data1} {register2:?}", "SET");
    }
    fn set_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "SCF");
    }
    fn shift_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SLA");
    }
    fn shift_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SRL");
    }
    fn shift_register_right_signed(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SRA");
    }
    fn store_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "STAX");
    }
    fn store_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "STA");
    }
    fn store_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "STAB");
    }
    fn store_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "STAC");
    }
    fn store_sp_direct(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:02x}", "SSPD");
    }
    fn store_sp_plus_immediate(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "STSP");
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SUB");
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SBB");
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "SUI");
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} #${data1:02x}", "SBI");
    }
    fn swap_register(&mut self, register1: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {register1:?}", "SWAP");
    }
    fn test_bit(&mut self, data1: u8, register2: Intel8080Register) {
        self.error = write!(self.stream_out, "{:04} {data1} {register2:?}", "BIT");
    }
}
