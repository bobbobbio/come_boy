// Copyright 2018 Remi Bernotavicius

use emulator_common::disassembler::{MemoryAccessor, MemoryDescription};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::Range;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct GameBoyRegister {
    pub chunk: MemoryChunk,
}

impl Default for GameBoyRegister {
    fn default() -> Self {
        GameBoyRegister::new()
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

pub struct GameBoyFlags<T> {
    pub chunk: MemoryChunk,
    phantom: PhantomData<T>,
}

impl<T> Default for GameBoyFlags<T> {
    fn default() -> Self {
        GameBoyFlags::new()
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

    pub fn set_value(&mut self, value: u8) {
        self.chunk.set_value(0, value)
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
        self.set_value((value & !mask) | v);
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

pub enum MappingType {
    Read,
    Write,
    ReadWrite,
}

pub trait MemoryMappedHardware {
    fn read_value(&self, address: u16) -> u8;
    fn set_value(&mut self, address: u16, value: u8);
    fn len(&self) -> u16;
}

pub struct GameBoyMemoryMap {
    read_map: BTreeMap<u16, Box<MemoryMappedHardware>>,
    write_map: BTreeMap<u16, Box<MemoryMappedHardware>>,
}

impl GameBoyMemoryMap {
    pub fn new() -> GameBoyMemoryMap {
        return GameBoyMemoryMap {
            read_map: BTreeMap::new(),
            write_map: BTreeMap::new(),
        };
    }

    fn map_chunk_internal<M: MemoryMappedHardware + 'static>(
        map: &mut BTreeMap<u16, Box<MemoryMappedHardware>>,
        address: u16,
        chunk: M,
    ) {
        // Assert the chunk we are mapping doesn't overlap with any existing chunks
        match map.range(..address).last() {
            Some((&key, value)) => assert!(
                address < key || address >= key + value.len(),
                "Chunk overlaps existing chunk"
            ),
            None => (),
        }

        match map.range(address..).next() {
            Some((&key, _)) => assert!(
                address + chunk.len() <= key,
                "Chunk overlaps existing chunk"
            ),
            None => (),
        }

        if address > 0 {
            assert!(
                chunk.len() <= (0xFFFF - address) + 1,
                "Chunk extends past end of address space"
            );
        }

        map.insert(address, Box::new(chunk));
    }

    pub fn map_chunk<M: MemoryMappedHardware + Clone + 'static>(
        &mut self,
        address: u16,
        chunk: M,
        t: MappingType,
    ) {
        match t {
            MappingType::Read => Self::map_chunk_internal(&mut self.read_map, address, chunk),
            MappingType::Write => Self::map_chunk_internal(&mut self.write_map, address, chunk),
            MappingType::ReadWrite => {
                Self::map_chunk_internal(&mut self.read_map, address, chunk.clone());
                Self::map_chunk_internal(&mut self.write_map, address, chunk);
            }
        }
    }

    fn get_chunk_for_address(
        &self,
        address: u16,
        t: MappingType,
    ) -> Option<(&u16, &(dyn MemoryMappedHardware + 'static))> {
        let map = match t {
            MappingType::Read => &self.read_map,
            MappingType::Write => &self.write_map,
            MappingType::ReadWrite => panic!(),
        };
        if address == 0xFFFF {
            map.iter().last()
        } else {
            map.range(..address + 1).last()
        }
        .map(|(a, v)| (a, v.deref()))
    }

    fn get_chunk_for_address_mut(
        &mut self,
        address: u16,
        t: MappingType,
    ) -> Option<(&u16, &mut (dyn MemoryMappedHardware + 'static))> {
        let map = match t {
            MappingType::Read => &mut self.read_map,
            MappingType::Write => &mut self.write_map,
            MappingType::ReadWrite => panic!(),
        };
        if address == 0xFFFF {
            map.iter_mut().last()
        } else {
            map.range_mut(..address + 1).last()
        }
        .map(|(a, v)| (a, v.deref_mut()))
    }
}

impl MemoryAccessor for GameBoyMemoryMap {
    fn read_memory(&self, address: u16) -> u8 {
        match self.get_chunk_for_address(address, MappingType::Read) {
            None => 0xFF,
            Some((key, ref chunk)) => {
                if address - key >= chunk.len() {
                    0xFF
                } else {
                    chunk.read_value(address - key)
                }
            }
        }
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        match self.get_chunk_for_address_mut(address, MappingType::Write) {
            None => {}
            Some((key, chunk)) => {
                if address - key < chunk.len() {
                    chunk.set_value(address - key, value);
                }
            }
        };
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        return MemoryDescription::Instruction;
    }
}

#[derive(Default)]
pub struct MemoryChunk {
    value: Rc<RefCell<(bool, Vec<u8>)>>,
    borrower: bool,
}

impl MemoryMappedHardware for MemoryChunk {
    fn set_value(&mut self, address: u16, value: u8) {
        let (borrowed, ref mut data) = *self.value.borrow_mut();
        if !borrowed || self.borrower {
            data[address as usize] = value;
        }
    }

    fn read_value(&self, address: u16) -> u8 {
        let (borrowed, ref data) = *self.value.borrow();
        if borrowed && !self.borrower {
            0xFF
        } else {
            data[address as usize]
        }
    }

    fn len(&self) -> u16 {
        let (_, ref data) = *self.value.borrow_mut();
        (*data).len() as u16
    }
}

impl Clone for MemoryChunk {
    fn clone(&self) -> Self {
        MemoryChunk {
            value: self.value.clone(),
            borrower: false,
        }
    }
}

impl MemoryChunk {
    pub fn new(v: Vec<u8>) -> MemoryChunk {
        assert!(v.len() > 0);
        MemoryChunk {
            value: Rc::new(RefCell::new((false, v))),
            borrower: false,
        }
    }

    pub fn from_range(range: Range<u16>) -> MemoryChunk {
        let mut v = Vec::<u8>::new();
        v.resize(range.len(), 0);
        return MemoryChunk::new(v);
    }

    pub fn clone_from_slice(&mut self, slice: &[u8]) {
        let (ref borrowed, ref mut data) = *self.value.borrow_mut();
        assert!(!borrowed);
        (*data).clone_from_slice(slice);
    }

    pub fn clone_range_from_slice(&mut self, range: Range<usize>, slice: &[u8]) {
        let (ref borrowed, ref mut data) = *self.value.borrow_mut();
        assert!(!borrowed);
        ((*data)[range]).clone_from_slice(slice);
    }

    pub fn borrow(&mut self) {
        if self.borrower {
            return;
        }

        let (ref mut borrowed, _) = *self.value.borrow_mut();
        assert!(!*borrowed);
        *borrowed = true;
        self.borrower = true;
    }

    pub fn release(&mut self) {
        if self.borrower == false {
            return;
        }
        self.borrower = false;

        let (ref mut borrowed, _) = *self.value.borrow_mut();
        assert!(*borrowed);
        *borrowed = false;
    }
}

pub struct MemoryChunkIterator<'a> {
    chunk: &'a MemoryChunk,
    current: u16,
}

impl<'a> Iterator for MemoryChunkIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current < self.chunk.len() {
            let mem = self.chunk.read_value(self.current);
            self.current += 1;
            return Some(mem);
        } else {
            return None;
        }
    }
}

impl<'a> MemoryChunkIterator<'a> {
    pub fn new(chunk: &'a MemoryChunk) -> MemoryChunkIterator {
        return MemoryChunkIterator {
            chunk: chunk,
            current: 0,
        };
    }
}

#[test]
#[should_panic]
fn overlapping_inside_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
    mm.map_chunk(13, MemoryChunk::new(vec![1]), MappingType::ReadWrite);
}

#[test]
fn overlapping_chunk_different_mapping() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]), MappingType::Read);
    mm.map_chunk(13, MemoryChunk::new(vec![1]), MappingType::Write);
}

#[test]
#[should_panic]
fn overlapping_left_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
    mm.map_chunk(10, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
}

#[test]
#[should_panic]
fn overlapping_right_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
    mm.map_chunk(14, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
}

#[test]
fn non_overlapping_left_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(3, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
    mm.map_chunk(0, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
}

#[test]
fn non_overlapping_right_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
    mm.map_chunk(15, MemoryChunk::new(vec![1, 2, 3]), MappingType::ReadWrite);
}

#[test]
#[should_panic]
fn mapping_past_end_of_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(
        0xFFFF,
        MemoryChunk::new(vec![1, 2, 3]),
        MappingType::ReadWrite,
    );
}

#[test]
fn accessing_unmapped_region() {
    let mm = GameBoyMemoryMap::new();
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn accessing_unmapped_region_with_region_mapped() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]), MappingType::ReadWrite);
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn setting_unmapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn setting_unmapped_region_with_region_mapped() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]), MappingType::ReadWrite);
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn accessing_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]), MappingType::ReadWrite);
    assert_eq!(mm.read_memory(10), 9);
    assert_eq!(mm.read_memory(11), 8);
    assert_eq!(mm.read_memory(12), 7);
}

#[test]
fn accessing_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![9]), MappingType::ReadWrite);
    assert_eq!(mm.read_memory(0xFFFF), 9);
}

#[test]
fn setting_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    let chunk = MemoryChunk::new(vec![9]);
    mm.map_chunk(0xFFFF, chunk.clone(), MappingType::ReadWrite);
    mm.set_memory(0xFFFF, 99);
    assert_eq!(chunk.read_value(0), 99);
}

#[test]
fn setting_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    let chunk = MemoryChunk::new(vec![9, 8, 7]);
    mm.map_chunk(10, chunk.clone(), MappingType::ReadWrite);
    mm.set_memory(11, 88);
    assert_eq!(chunk.read_value(1), 88);
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}
