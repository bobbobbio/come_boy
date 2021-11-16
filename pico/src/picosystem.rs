// copyright 2021 Remi Bernotavicius

#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type c_char = i8;
type c_uchar = u8;
type c_schar = i8;
type c_int = i32;
type c_uint = u32;
type c_ushort = u16;
type c_short = i16;
type c_long = i64;
type c_longlong = i64;
type c_ulong = u64;
type c_ulonglong = u64;
type c_void = core::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
