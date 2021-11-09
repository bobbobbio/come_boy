// copyright 2021 Remi Bernotavicius

#![cfg_attr(feature = "std", allow(dead_code))]

//! This module is here so no_std will compile.
//! I need some serialization format that supports no_std

use super::io::{self, Result};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct Error;

pub fn serialize_into<W, T: ?Sized>(_writer: W, _value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    unimplemented!()
}

pub fn deserialize_from<R, T>(_reader: R) -> Result<T>
where
    R: io::Read,
    T: DeserializeOwned,
{
    unimplemented!()
}
