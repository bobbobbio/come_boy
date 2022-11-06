// copyright 2022 Remi Bernotavicius

#![allow(dead_code)]

use crate::picosystem;
use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instant {
    us: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            us: unsafe { picosystem::now_us() },
        }
    }

    pub fn duration_since(&self, earlier: Self) -> Duration {
        Duration::from_micros(self.us - earlier.us)
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl come_boy::game_boy_emulator::perf::Instant for Instant {
    fn now() -> Self {
        Instant::now()
    }

    fn elapsed(&self) -> Duration {
        Instant::elapsed(self)
    }
}
