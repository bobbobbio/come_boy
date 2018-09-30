// Copyright 2018 Remi Bernotavicius

use emulator_common::disassembler::{MemoryAccessor, MemoryDescription};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Range;
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

pub struct GameBoyMemoryMap {
    memory_map: BTreeMap<u16, MemoryChunk>,
}

impl<'a> GameBoyMemoryMap {
    pub fn new() -> GameBoyMemoryMap {
        return GameBoyMemoryMap {
            memory_map: BTreeMap::new(),
        };
    }

    pub fn map_chunk(&mut self, address: u16, mut chunk: MemoryChunk) {
        // Assert the chunk we are mapping doesn't overlap with any existing chunks
        match self.memory_map.range(..address).last() {
            Some((&key, value)) => assert!(
                address < key || address >= key + value.len(),
                "Chunk overlaps existing chunk"
            ),
            None => (),
        }

        match self.memory_map.range(address..).next() {
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

        self.memory_map.insert(address, chunk.clone());
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    pub fn unmap_chunk(&mut self, address: u16) {
        self.memory_map.remove(&address);
    }

    pub fn get_chunk_for_address(&self, address: u16) -> Option<(&u16, &MemoryChunk)> {
        if address == 0xFFFF {
            self.memory_map.iter().last()
        } else {
            self.memory_map.range(..address + 1).last()
        }
    }

    pub fn get_chunk_for_address_mut(&mut self, address: u16) -> Option<(&u16, &mut MemoryChunk)> {
        if address == 0xFFFF {
            self.memory_map.iter_mut().last()
        } else {
            self.memory_map.range_mut(..address + 1).last()
        }
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    pub fn get_address_for_chunk(&self, c: &MemoryChunk) -> Option<u16> {
        for (&address, chunk) in &self.memory_map {
            if chunk.ptr_eq(c) {
                return Some(address);
            }
        }

        return None;
    }
}

impl<'a> MemoryAccessor for GameBoyMemoryMap {
    fn read_memory(&self, address: u16) -> u8 {
        match self.get_chunk_for_address(address) {
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
        match self.get_chunk_for_address_mut(address) {
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
    read_only: bool,
}

impl MemoryChunk {
    pub fn new(v: Vec<u8>) -> MemoryChunk {
        assert!(v.len() > 0);
        MemoryChunk {
            value: Rc::new(RefCell::new((false, v))),
            borrower: false,
            read_only: false,
        }
    }

    pub fn set_value(&mut self, address: u16, value: u8) {
        if self.read_only {
            return;
        }
        let (borrowed, ref mut data) = *self.value.borrow_mut();
        if !borrowed || self.borrower {
            data[address as usize] = value;
        }
    }

    pub fn read_value(&self, address: u16) -> u8 {
        let (borrowed, ref data) = *self.value.borrow();
        if borrowed && !self.borrower {
            0xFF
        } else {
            data[address as usize]
        }
    }

    pub fn clone(&mut self) -> MemoryChunk {
        MemoryChunk {
            value: self.value.clone(),
            borrower: false,
            read_only: self.read_only,
        }
    }

    pub fn clone_read_only(&mut self) -> MemoryChunk {
        MemoryChunk {
            value: self.value.clone(),
            borrower: false,
            read_only: true,
        }
    }

    pub fn len(&self) -> u16 {
        let (_, ref data) = *self.value.borrow_mut();
        (*data).len() as u16
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

    pub fn ptr_eq(&self, other: &MemoryChunk) -> bool {
        Rc::<RefCell<(bool, Vec<u8>)>>::ptr_eq(&self.value, &other.value)
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
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(13, MemoryChunk::new(vec![1]));
}

#[test]
#[should_panic]
fn overlapping_left_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(10, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn overlapping_right_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(14, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_left_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(3, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(0, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_right_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(15, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn mapping_past_end_of_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn accessing_unmapped_region() {
    let mm = GameBoyMemoryMap::new();
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn accessing_unmapped_region_with_region_mapped() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
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
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn accessing_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    assert_eq!(mm.read_memory(10), 9);
    assert_eq!(mm.read_memory(11), 8);
    assert_eq!(mm.read_memory(12), 7);
}

#[test]
fn accessing_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![9]));
    assert_eq!(mm.read_memory(0xFFFF), 9);
}

#[test]
fn setting_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9]);
    mm.map_chunk(0xFFFF, chunk.clone());
    mm.set_memory(0xFFFF, 99);
    assert_eq!(chunk.read_value(0), 99);
}

#[test]
fn setting_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9, 8, 7]);
    mm.map_chunk(10, chunk.clone());
    mm.set_memory(11, 88);
    assert_eq!(chunk.read_value(1), 88);
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}
