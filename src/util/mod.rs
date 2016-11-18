use std::cell::UnsafeCell;
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

// This function adds mut to any reference.  Basically we are telling the compiler to trust us.
pub unsafe fn add_mut<T>(r: &T) -> &mut T
{
    return *mem::transmute::<*mut &T, *mut &mut T>(UnsafeCell::new(r).get());
}

#[test]
fn add_mut_test()
{
    let v: u8 = 123;
    let g: &mut u8;
    unsafe {
        g = add_mut(&v);
    }

    assert_eq!(*g, v);
}
