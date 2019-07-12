// Copyright 2017 Remi Bernotavicius

use std::collections::BTreeMap;
use std::io::{self, Result};
use std::{mem, slice};

macro_rules! from_u8 {
    ($($cname:ident),*) => ($(
        impl From<$cname> for u8 {
            fn from(f: $cname) -> u8 {
                f as u8
            }
        }
    )*)
}

pub fn read_u16<T: io::Read>(mut stream: T) -> Result<u16> {
    let mut arg_buffer = [0; 2];
    stream.read_exact(&mut arg_buffer)?;
    let narg: u16 = unsafe { mem::transmute(arg_buffer) };
    Ok(u16::from_le(narg))
}

pub fn read_u8<T: io::Read>(mut stream: T) -> Result<u8> {
    let mut arg_buffer = [0; 1];
    stream.read_exact(&mut arg_buffer)?;
    Ok(arg_buffer[0])
}

#[test]
fn read_u8_test() {
    assert_eq!(read_u8(&[0x0f][..]).unwrap(), 0x0f);
}

#[test]
fn read_u16_test() {
    assert_eq!(read_u16(&[0x0f, 0x08][..]).unwrap(), 0x080f);
}

pub trait TwosComplement<T> {
    fn twos_complement(self) -> T;
}

impl TwosComplement<u8> for u8 {
    fn twos_complement(self) -> u8 {
        (!self).wrapping_add(1)
    }
}

impl TwosComplement<u16> for u16 {
    fn twos_complement(self) -> u16 {
        (!self).wrapping_add(1)
    }
}

#[test]
fn twos_complement_u8() {
    assert_eq!(0b00001010u8.twos_complement(), 0b11110110u8);
}

#[test]
fn twos_complement_u16() {
    assert_eq!(
        0b0111000000001010u16.twos_complement(),
        0b1000111111110110u16
    );
}

fn get_16_bits(data: &[u8], index: usize) -> u16 {
    (data[index + 1] as u16) << 8 | data[index] as u16
}

// This was taken from http://www.azillionmonkeys.com/qed/hash.html. The code
// for this function (and only this function) below is LGPL 2.1. I have ported it to rust, but
// kept the same behavior.
fn super_fast_hash_iter<T: Sized>(data_in: &T, mut hash: u32) -> u32 {
    let mut len = mem::size_of_val(data_in);
    let data: &[u8] = unsafe { mem::transmute(slice::from_raw_parts(data_in, len)) };

    let mut tmp: u32;
    let mut i = 0;

    let rem = (len & 3) as u32;
    len >>= 2;

    /* Main loop */
    for _ in 0..len {
        hash = hash.wrapping_add(get_16_bits(data, i) as u32);
        tmp = ((get_16_bits(data, i + 2) as u32) << 11) ^ hash;
        hash = (hash << 16) ^ tmp;
        i += 4;
        hash = hash.wrapping_add(hash >> 11);
    }

    /* Handle end cases */
    match rem {
        3 => {
            hash = hash.wrapping_add(get_16_bits(data, i) as u32);
            hash ^= hash << 16;
            hash ^= ((data[i + 2] as i8) as u32) << 18;
            hash = hash.wrapping_add(hash >> 11);
        }

        2 => {
            hash = hash.wrapping_add(get_16_bits(data, i) as u32);
            hash ^= hash << 11;
            hash = hash.wrapping_add(hash >> 17);
        }

        1 => {
            hash = hash.wrapping_add((data[i] as i8) as u32);
            hash ^= hash << 10;
            hash = hash.wrapping_add(hash >> 1);
        }

        _ => {}
    }

    /* Force "avalanching" of final 127 bits */
    hash ^= hash << 3;
    hash = hash.wrapping_add(hash >> 5);
    hash ^= hash << 4;
    hash = hash.wrapping_add(hash >> 17);
    hash ^= hash << 25;
    hash = hash.wrapping_add(hash >> 6);

    return hash;
}

pub fn super_fast_hash<T: Sized>(data_in: &T) -> u32 {
    let len = mem::size_of_val(data_in);
    super_fast_hash_iter(data_in, len as u32)
}

// These values were taken by running the original C version.
#[test]
fn super_fast_hash_example_1() {
    let v = [0x88u8, 0x99u8, 0x10u8, 0x11u8, 0x09u8];
    assert_eq!(super_fast_hash(&v), 284656667);
}

#[test]
fn super_fast_hash_example_2() {
    let v = [0x77u8, 0x01u8, 0x12u8, 0x24u8];
    assert_eq!(super_fast_hash(&v), 700799581);
}

#[test]
fn super_fast_hash_example_3() {
    let v = [0x91u8, 0x00u8, 0x84u8];
    assert_eq!(super_fast_hash(&v), 505819445);
}

#[test]
fn super_fast_hash_example_4() {
    let v = [0x11u8, 0x05u8];
    assert_eq!(super_fast_hash(&v), 3238191665);
}

#[test]
fn super_fast_hash_example_5() {
    let v = [0x45u8];
    assert_eq!(super_fast_hash(&v), 3114100952);
}

/*  ____       _              _       _
 * / ___|  ___| |__   ___  __| |_   _| | ___ _ __
 * \___ \ / __| '_ \ / _ \/ _` | | | | |/ _ \ '__|
 *  ___) | (__| | | |  __/ (_| | |_| | |  __/ |
 * |____/ \___|_| |_|\___|\__,_|\__,_|_|\___|_|
 */

#[derive(Default)]
pub struct Scheduler<T> {
    timeline: BTreeMap<u64, Vec<for<'r> fn(&'r mut T, u64)>>,
}

impl<T> Scheduler<T> {
    pub fn new() -> Self {
        Scheduler {
            timeline: BTreeMap::new(),
        }
    }

    pub fn schedule(&mut self, time: u64, event: for<'r> fn(&'r mut T, u64)) {
        if !self.timeline.contains_key(&time) {
            self.timeline.insert(time, Vec::new());
        }
        self.timeline.get_mut(&time).unwrap().push(event);
    }

    pub fn poll(&mut self, current_time: u64) -> Option<(u64, for<'r> fn(&'r mut T, u64))> {
        let entry = self.timeline.range_mut(..=current_time).next();
        if let Some((k, v)) = entry {
            let key = *k;
            let entry = (key, v.pop().unwrap());
            if v.len() == 0 {
                self.timeline.remove(&key);
            }
            Some(entry)
        } else {
            None
        }
    }

    pub fn drop_events(&mut self) {
        self.timeline = BTreeMap::new();
    }
}
