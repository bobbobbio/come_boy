// copyright 2021 Remi Bernotavicius
use core::time::Duration;
use std::time::Instant;

pub struct Underclocker {
    start_cycles: u64,
    start_instant: Instant,
    speed: u32,
}

impl Underclocker {
    pub fn new(now: u64, speed: u32) -> Self {
        Self {
            start_cycles: now,
            start_instant: Instant::now(),
            speed,
        }
    }

    pub fn underclock(&mut self, now: u64) {
        let elapsed_cycles = now - self.start_cycles;

        let delay = Duration::from_secs(1) / self.speed;
        let expected_elapsed = (elapsed_cycles as u32) * delay;

        if let Some(sleep_time) = expected_elapsed.checked_sub(self.start_instant.elapsed()) {
            std::thread::sleep(sleep_time);
        }
    }
}
