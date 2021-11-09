// copyright 2021 Remi Bernotavicius

#![cfg_attr(feature = "std", allow(dead_code))]

pub mod codec;
pub mod io;

pub struct Instant;

impl Instant {
    pub fn now() -> Self {
        unimplemented!()
    }

    pub fn elapsed(&self) -> core::time::Duration {
        unimplemented!()
    }
}

pub fn sleep(_duration: core::time::Duration) {
    unimplemented!()
}
