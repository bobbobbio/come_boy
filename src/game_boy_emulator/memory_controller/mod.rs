// Copyright 2018 Remi Bernotavicius

pub use self::memory_map::{GameBoyMemoryMap, GameBoyMemoryMapMut};
pub use crate::emulator_common::disassembler::{MemoryAccessor, MemoryDescription};
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::marker::PhantomData;
use std::ops::Range;

#[macro_use]
pub mod memory_map;

#[derive(Default, Serialize, Deserialize)]
pub struct GameBoyRegister {
    value: u8,
}

impl MemoryMappedHardware for GameBoyRegister {
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.value
    }

    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        self.value = value;
    }
}

impl GameBoyRegister {
    pub fn read_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn add(&mut self, value: u8) {
        self.value = self.value.wrapping_add(value);
    }
}

#[test]
fn gameboy_register_read_write() {
    let mut register = GameBoyRegister::default();
    register.set_value(12);
    assert_eq!(register.read_value(), 12);
}

pub trait FlagMask {
    fn read_mask() -> u8;
    fn write_mask() -> u8;
}

#[derive(Serialize, Deserialize)]
pub struct GameBoyFlags<T> {
    value: u8,
    phantom: PhantomData<T>,
}

impl<T> Default for GameBoyFlags<T> {
    fn default() -> Self {
        Self {
            value: 0,
            phantom: PhantomData,
        }
    }
}

impl<T: FlagMask> MemoryMappedHardware for GameBoyFlags<T> {
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        Self::read_value(self)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        Self::set_value(self, value);
    }
}

impl<T> GameBoyFlags<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: FlagMask> GameBoyFlags<T> {
    pub fn set_value(&mut self, value: u8) {
        self.value = (value & T::write_mask()) | (self.value & !T::write_mask());
    }

    pub fn read_value(&self) -> u8 {
        self.value | !T::read_mask()
    }
}

impl<T: FlagMask> GameBoyFlags<T>
where
    u8: From<T>,
{
    pub fn read_flag(&self, f: T) -> bool {
        self.read_flag_value(f) != 0
    }

    pub fn read_flag_value(&self, f: T) -> u8 {
        let value = self.read_value();
        let mask = u8::from(f);
        (value & mask) >> mask.trailing_zeros()
    }

    pub fn set_flag(&mut self, f: T, v: bool) {
        self.set_flag_value(f, if v { 1 } else { 0 });
    }

    pub fn set_flag_value(&mut self, f: T, v: u8) {
        let value = self.read_value();
        let mask = u8::from(f);
        let v = (v << mask.trailing_zeros()) & mask;
        self.value = (value & !mask) | v;
    }
}

#[cfg(test)]
enum TestMaskedValue {
    ReadWriteValue = 0b00000011,
    ReadWriteFlag = 0b00000100,
    ReadOnlyValue = 0b00011000,
    ReadOnlyFlag = 0b00100000,
}

#[cfg(test)]
from_u8!(TestMaskedValue);

#[cfg(test)]
impl FlagMask for TestMaskedValue {
    fn read_mask() -> u8 {
        Self::ReadWriteValue as u8
            | Self::ReadWriteFlag as u8
            | Self::ReadOnlyValue as u8
            | Self::ReadOnlyFlag as u8
    }

    fn write_mask() -> u8 {
        Self::ReadWriteValue as u8 | Self::ReadWriteFlag as u8
    }
}

#[test]
fn set_flag_then_read_flag() {
    let mut f: GameBoyFlags<TestMaskedValue> = GameBoyFlags::new();
    f.set_flag(TestMaskedValue::ReadWriteFlag, true);
    assert!(f.read_flag(TestMaskedValue::ReadWriteFlag));

    f.set_flag(TestMaskedValue::ReadWriteFlag, false);
    assert!(!f.read_flag(TestMaskedValue::ReadWriteFlag));
}

#[test]
fn set_flag_value_then_read_flag_value() {
    let mut f: GameBoyFlags<TestMaskedValue> = GameBoyFlags::new();

    for i in 0..3 {
        f.set_flag_value(TestMaskedValue::ReadWriteValue, i);
        assert_eq!(f.read_flag_value(TestMaskedValue::ReadWriteValue), i);
    }

    // When we set a value too large, we only keep the lower bits that fit
    f.set_flag_value(TestMaskedValue::ReadWriteValue, 0b1101);
    assert_eq!(f.read_flag_value(TestMaskedValue::ReadWriteValue), 0b01);
}

#[test]
fn flags_read_mask() {
    let mut f: GameBoyFlags<TestMaskedValue> = GameBoyFlags::new();

    // The values outside the read mask should be 1
    assert_eq!(f.read_value(), 0b11000000);

    // Even if we try to set it to 0, we can't read the outside the read-mask
    f.set_value(0);
    assert_eq!(f.read_value(), 0b11000000);
}

#[test]
fn flags_write_mask() {
    let mut f: GameBoyFlags<TestMaskedValue> = GameBoyFlags::new();

    f.set_flag_value(TestMaskedValue::ReadOnlyValue, 2);
    assert_eq!(f.read_flag_value(TestMaskedValue::ReadOnlyValue), 2);

    f.set_flag(TestMaskedValue::ReadOnlyFlag, true);
    assert!(f.read_flag(TestMaskedValue::ReadOnlyFlag));

    // No matter what value we set, the read-only value should stay the same
    f.set_value(0);
    assert_eq!(f.read_flag_value(TestMaskedValue::ReadOnlyValue), 2);
    assert!(f.read_flag(TestMaskedValue::ReadOnlyFlag));

    f.set_flag(TestMaskedValue::ReadOnlyFlag, false);

    // If we set read-only and read-write bits, only the read-write bits are written
    f.set_value(TestMaskedValue::ReadWriteFlag as u8 | TestMaskedValue::ReadOnlyFlag as u8);
    assert!(!f.read_flag(TestMaskedValue::ReadOnlyFlag));
    assert!(f.read_flag(TestMaskedValue::ReadWriteFlag));
}

pub trait MemoryMappedHardware {
    fn read_value(&self, address: u16) -> u8;
    fn set_value(&mut self, address: u16, value: u8);
}

impl<T: MemoryMappedHardware> MemoryMappedHardware for Option<T> {
    fn read_value(&self, address: u16) -> u8 {
        match self {
            Some(v) => v.read_value(address),
            None => 0xFF,
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if let Some(v) = self {
            v.set_value(address, value);
        };
    }
}

impl<T: MemoryMappedHardware + ?Sized> MemoryMappedHardware for Box<T> {
    fn read_value(&self, address: u16) -> u8 {
        MemoryMappedHardware::read_value(&**self, address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        MemoryMappedHardware::set_value(&mut **self, address, value)
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemoryChunk {
    value: Vec<u8>,
    borrowed: bool,
}

impl MemoryMappedHardware for MemoryChunk {
    fn set_value(&mut self, address: u16, value: u8) {
        if !self.borrowed {
            self.value[address as usize] = value;
        }
    }

    fn read_value(&self, address: u16) -> u8 {
        if self.borrowed {
            0xFF
        } else {
            self.value[address as usize]
        }
    }
}

impl MemoryChunk {
    pub fn new(value: Vec<u8>) -> MemoryChunk {
        assert!(value.len() > 0);
        MemoryChunk {
            value,
            borrowed: false,
        }
    }

    pub fn from_range(range: Range<u16>) -> MemoryChunk {
        let mut v = Vec::<u8>::new();
        v.resize(range.len(), 0);
        MemoryChunk::new(v)
    }

    pub fn from_reader<R: io::Read>(mut r: R, len: usize) -> io::Result<Self> {
        let mut v = Vec::<u8>::new();
        v.resize(len, 0);
        r.read_exact(&mut v)?;
        Ok(MemoryChunk::new(v))
    }

    pub fn clone_from_slice(&mut self, slice: &[u8]) {
        self.value.clone_from_slice(slice);
    }

    pub fn clone_range_from_slice(&mut self, range: Range<usize>, slice: &[u8]) {
        self.value[range].clone_from_slice(slice);
    }

    pub fn borrow(&mut self) {
        self.borrowed = true;
    }

    pub fn release(&mut self) {
        self.borrowed = false;
    }

    pub fn len(&self) -> u16 {
        self.value.len() as u16
    }

    pub fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}
