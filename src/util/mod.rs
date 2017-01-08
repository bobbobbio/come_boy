use std::io::{self, Result};
use std::mem;

pub fn read_u16<T: io::Read>(
    mut stream: T) -> Result<u16>
{
    let narg : u16;
    let mut arg_buffer = [0; 2];
    try!(stream.read_exact(&mut arg_buffer));
    unsafe {
        narg = mem::transmute(arg_buffer);
    }
    Ok(u16::from_le(narg))
}

pub fn read_u8<T: io::Read>(
    mut stream: T) -> Result<u8>
{
    let mut arg_buffer = [0; 1];
    try!(stream.read_exact(&mut arg_buffer));
    Ok(arg_buffer[0])
}

#[test]
fn read_u8_test()
{
    assert_eq!(read_u8(&[0x0f][..]).unwrap(), 0x0f);
}

#[test]
fn read_u16_test()
{
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
fn twos_complement_u8()
{
    assert_eq!(0b00001010u8.twos_complement(), 0b11110110u8);
}

#[test]
fn twos_complement_u16()
{
    assert_eq!(0b0111000000001010u16.twos_complement(), 0b1000111111110110u16);
}
