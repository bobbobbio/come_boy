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
    let data = [0x0f];
    assert_eq!(read_u8(data.as_ref()).unwrap(), 0x0f);
}

#[test]
fn read_u16_test()
{
    let data = [0x0f, 0x08];
    assert_eq!(read_u16(data.as_ref()).unwrap(), 0x080f);
}
