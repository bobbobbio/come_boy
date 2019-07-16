// Copyright 2018 Remi Bernotavicius

pub use self::memory_map::{GameBoyMemoryMap, GameBoyMemoryMapMut};
pub use emulator_common::disassembler::{MemoryAccessor, MemoryDescription};
use std::marker::PhantomData;
use std::ops::Range;

#[macro_use]
pub mod memory_map;

pub struct GameBoyRegister {
    pub chunk: MemoryChunk,
}

impl Default for GameBoyRegister {
    fn default() -> Self {
        GameBoyRegister::new()
    }
}

impl MemoryMappedHardware for GameBoyRegister {
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.chunk.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        self.chunk.set_value(address, value)
    }
}

impl GameBoyRegister {
    pub fn new() -> GameBoyRegister {
        GameBoyRegister {
            chunk: MemoryChunk::from_range(0..1),
        }
    }

    pub fn read_value(&self) -> u8 {
        self.chunk.read_value(0)
    }

    pub fn set_value(&mut self, value: u8) {
        self.chunk.set_value(0, value)
    }

    pub fn add(&mut self, value: u8) {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_add(value));
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    pub fn subtract(&mut self, value: u8) {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_sub(value));
    }
}

pub trait FlagMask {
    fn mask() -> u8;
}

pub struct GameBoyFlags<T> {
    chunk: MemoryChunk,
    phantom: PhantomData<T>,
}

impl<T> Default for GameBoyFlags<T> {
    fn default() -> Self {
        GameBoyFlags::new()
    }
}

impl<T: FlagMask> MemoryMappedHardware for GameBoyFlags<T> {
    fn read_value(&self, address: u16) -> u8 {
        assert_eq!(address, 0);
        self.chunk.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        assert_eq!(address, 0);
        self.chunk
            .set_value(address, (value & T::mask()) | !T::mask())
    }
}

impl<T> GameBoyFlags<T> {
    pub fn new() -> Self {
        Self {
            chunk: MemoryChunk::from_range(0..1),
            phantom: Default::default(),
        }
    }

    pub fn read_value(&self) -> u8 {
        self.chunk.read_value(0)
    }
}

impl<T: FlagMask> GameBoyFlags<T> {
    pub fn set_value(&mut self, value: u8) {
        self.chunk.set_value(0, (value & T::mask()) | !T::mask())
    }
}

impl<T> GameBoyFlags<T>
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
        self.chunk.set_value(0, (value & !mask) | v);
    }
}

#[test]
fn set_flag_then_read_flag() {
    let mut f: GameBoyFlags<u8> = GameBoyFlags::new();
    f.set_flag(0b0010, true);
    assert!(f.read_flag(0b0010));

    f.set_flag(0b0010, false);
    assert!(!f.read_flag(0b0010));
}

#[test]
fn set_flag_value_then_read_flag_value() {
    let mut f: GameBoyFlags<u8> = GameBoyFlags::new();
    f.set_flag_value(0b001110, 0b101);
    assert_eq!(f.read_flag_value(0b001110), 0b101);
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

#[derive(Default)]
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

    #[cfg(test)]
    pub fn len(&self) -> u16 {
        self.value.len() as u16
    }

    pub fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }

    /*
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.value.as_mut_slice()
    }
    */
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}
