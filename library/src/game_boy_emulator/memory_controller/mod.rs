// Copyright 2018 Remi Bernotavicius

use super::game_pak::GamePak;
use super::joypad::JoyPad;
use super::Bridge;
pub use crate::emulator_common::disassembler::{MemoryAccessor, MemoryDescription};
use crate::storage::PersistentStorage;
use alloc::{boxed::Box, format, vec, vec::Vec};
use core::fmt;
use core::marker::PhantomData;
use core::ops::Range;
use enum_iterator::IntoEnumIterator;
use serde_derive::{Deserialize, Serialize};

mod memory_map;
mod memory_map_mut;

pub(crate) struct GameBoyMemoryMap<'a, Storage: PersistentStorage> {
    pub game_pak: Option<&'a GamePak<Storage>>,
    pub joypad: Option<&'a dyn JoyPad>,
    pub bridge: &'a Bridge,
}

pub(crate) struct GameBoyMemoryMapMut<'a, Storage: PersistentStorage> {
    pub game_pak: Option<&'a mut GamePak<Storage>>,
    pub joypad: Option<&'a mut dyn JoyPad>,
    pub bridge: &'a mut Bridge,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GameBoyRegister {
    value: u8,
}

impl fmt::Debug for GameBoyRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02x}", self.value)
    }
}

impl MemoryMappedHardware for GameBoyRegister {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        self.value = value;
    }
}

impl GameBoyRegister {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_value(&self) -> u8 {
        self.value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GameBoyRegister16 {
    value: u16,
}

impl fmt::Debug for GameBoyRegister16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04x}", self.value)
    }
}

impl MemoryMappedHardware for GameBoyRegister16 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        match address {
            0 => self.value as u8,
            1 => (self.value >> 8) as u8,
            a => panic!("address = {}", a),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        self.value = match address {
            0 => (self.value & 0xFF00) | value as u16,
            1 => (self.value & 0x00FF) | ((value as u16) << 8),
            a => panic!("address = {}", a),
        };
    }
}

impl GameBoyRegister16 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_value(&self) -> u16 {
        self.value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_value(&mut self, value: u16) {
        self.value = value;
    }
}

#[test]
fn gameboy_register_16_read_write() {
    let mut register = GameBoyRegister16::default();
    register.set_value(0xabcd);
    assert_eq!(register.read_value(), 0xabcd);

    assert_eq!(MemoryMappedHardware::read_value(&register, 0), 0xcd);
    assert_eq!(MemoryMappedHardware::read_value(&register, 1), 0xab);

    MemoryMappedHardware::set_value(&mut register, 0, 0x12);
    assert_eq!(MemoryMappedHardware::read_value(&register, 0), 0x12);

    MemoryMappedHardware::set_value(&mut register, 1, 0x34);
    assert_eq!(MemoryMappedHardware::read_value(&register, 1), 0x34);
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
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        Self::read_value(self)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
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
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_value(&mut self, value: u8) {
        self.value = (value & T::write_mask()) | (self.value & !T::write_mask());
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_value(&self) -> u8 {
        self.value | !T::read_mask()
    }
}

impl<T> GameBoyFlags<T>
where
    u8: From<T>,
{
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_flag(&self, f: T) -> bool {
        self.read_flag_value(f) != 0
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_flag_value(&self, f: T) -> u8 {
        let mask = u8::from(f);
        (self.value & mask) >> mask.trailing_zeros()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_flag(&mut self, f: T, v: bool) {
        self.set_flag_value(f, v.into())
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_flag_value(&mut self, f: T, v: u8) {
        let mask = u8::from(f);
        let v = (v << mask.trailing_zeros()) & mask;
        self.value = (self.value & !mask) | v;
    }
}

impl<T> fmt::Debug for GameBoyFlags<T>
where
    T: IntoEnumIterator + fmt::Debug + Clone,
    u8: From<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut extra = vec![];
        for flag in T::into_enum_iter() {
            let value = self.read_flag_value(flag.clone());
            let is_flag = u8::from(flag.clone()).count_ones() == 1;

            if is_flag && value == 1 {
                extra.push(format!("{flag:?}"));
            } else if !is_flag {
                extra.push(format!("{flag:?} = {value}"));
            }
        }
        write!(f, "0x{:02x}: [{}]", self.value, extra.join(", "))?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::enum_variant_names)]
#[derive(num_enum::IntoPrimitive, IntoEnumIterator, Debug, Clone)]
#[repr(u8)]
enum TestMaskedValue {
    ReadWriteValue = 0b00000011,
    ReadWriteFlag = 0b00000100,
    ReadOnlyValue = 0b00011000,
    ReadOnlyFlag = 0b00100000,
}

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
fn flags_debug_fmt() {
    let mut f: GameBoyFlags<TestMaskedValue> = GameBoyFlags::new();
    f.set_flag(TestMaskedValue::ReadWriteFlag, true);
    f.set_flag_value(TestMaskedValue::ReadWriteValue, 2);
    assert_eq!(
        format!("{f:?}"),
        "0x06: [ReadWriteValue = 2, ReadWriteFlag, ReadOnlyValue = 0]"
    )
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
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        match self {
            Some(v) => v.read_value(address),
            None => 0xFF,
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        if let Some(v) = self {
            v.set_value(address, value);
        };
    }
}

impl<T: MemoryMappedHardware + ?Sized> MemoryMappedHardware for Box<T> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        MemoryMappedHardware::read_value(&**self, address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        MemoryMappedHardware::set_value(&mut **self, address, value)
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemoryChunk {
    value: Vec<u8>,
    borrowed: u32,
}

impl fmt::Debug for MemoryChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemoryChunk")
    }
}

impl MemoryMappedHardware for MemoryChunk {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        if self.borrowed == 0 {
            self.value[address as usize] = value;
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        if self.borrowed != 0 {
            0xFF
        } else {
            self.value[address as usize]
        }
    }
}

impl MemoryChunk {
    pub fn new(value: Vec<u8>) -> MemoryChunk {
        assert!(!value.is_empty());
        MemoryChunk { value, borrowed: 0 }
    }

    pub fn from_range(range: Range<u16>) -> MemoryChunk {
        let mut v = Vec::<u8>::new();
        v.resize(range.len(), 0);
        MemoryChunk::new(v)
    }

    pub fn clone_from_slice(&mut self, slice: &[u8]) {
        self.value.clone_from_slice(slice);
    }

    pub fn clone_range_from_slice(&mut self, range: Range<usize>, slice: &[u8]) {
        self.value[range].clone_from_slice(slice);
    }

    pub fn borrow(&mut self) {
        self.borrowed += 1;
    }

    pub fn release(&mut self) {
        self.borrowed -= 1;
    }

    #[cfg(test)]
    pub fn release_all(&mut self) {
        self.borrowed = 0;
    }

    pub fn len(&self) -> u16 {
        self.value.len() as u16
    }

    pub fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.value.as_mut_slice()
    }
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}
