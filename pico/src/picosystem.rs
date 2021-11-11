// copyright 2021 Remi Bernotavicius

#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type c_uchar = u8;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
