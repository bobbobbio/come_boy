// copyright 2021 Remi Bernotavicius
use super::io::{self, Result};
pub use byteorder::*;

pub trait ReadBytesExt: io::Read {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(T::read_u16(&buf))
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}
