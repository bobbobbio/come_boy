// Copyright 2022 Remi Bernotavicius

use alloc::{collections::BTreeMap, vec::Vec};
use core::fmt;
use core::time::Duration;

pub trait PerfObserver {
    fn start_observation(&mut self, tag: &'static str);
    fn end_observation(&mut self, tag: &'static str);
    fn tick_observed(&mut self);
}

pub struct NullPerfObserver;

impl PerfObserver for NullPerfObserver {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn start_observation(&mut self, _tag: &'static str) {}

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn end_observation(&mut self, _tag: &'static str) {}

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn tick_observed(&mut self) {}
}

#[cfg_attr(not(debug_assertions), inline(always))]
pub fn observe<R>(p: &mut impl PerfObserver, tag: &'static str, body: impl FnOnce() -> R) -> R {
    p.start_observation(tag);
    let ret = body();
    p.end_observation(tag);
    ret
}

trait Instant: Sized {
    fn now() -> Self;

    fn elapsed(&self) -> Duration;
}

impl Instant for crate::Instant {
    fn now() -> Self {
        crate::Instant::now()
    }

    fn elapsed(&self) -> Duration {
        crate::Instant::elapsed(self)
    }
}

pub struct PerfStats<InstantT = crate::Instant> {
    in_flight: BTreeMap<&'static str, InstantT>,
    stats: BTreeMap<&'static str, (Duration, u32)>,
    num_ticks: u32,
}

impl<InstantT> Default for PerfStats<InstantT> {
    fn default() -> Self {
        Self {
            in_flight: Default::default(),
            stats: Default::default(),
            num_ticks: Default::default(),
        }
    }
}

impl PerfStats<crate::Instant> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<InstantT: Instant> PerfObserver for PerfStats<InstantT> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn start_observation(&mut self, tag: &'static str) {
        let existing = self.in_flight.insert(tag, InstantT::now()).is_some();
        assert!(!existing, "{}", "unfinished tag {tag}");
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn end_observation(&mut self, tag: &'static str) {
        let start = self
            .in_flight
            .remove(tag)
            .unwrap_or_else(|| panic!("unexpected tag {}", tag));
        let entry = self.stats.entry(tag).or_insert((Duration::ZERO, 0));
        entry.0 += start.elapsed();
        entry.1 += 1;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn tick_observed(&mut self) {
        self.num_ticks += 1;
    }
}

impl<InstantT> fmt::Display for PerfStats<InstantT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ticks = self.num_ticks;
        assert!(ticks != 0);
        let mut sorted_stats: Vec<_> = self
            .stats
            .iter()
            .map(|(t, &(d, n))| {
                assert!(n != 0);
                if ticks > n {
                    (t, d / n / (ticks / n))
                } else {
                    (t, d / n * (n / ticks))
                }
            })
            .collect();
        sorted_stats.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

        writeln!(f)?;
        for (tag, duration) in &sorted_stats {
            write!(
                f,
                "{tag:<30}: {:>10}ns amortized per tick",
                duration.as_nanos()
            )?;

            let &(dur, samples) = self.stats.get(*tag).unwrap();
            let avg = (dur / samples).as_nanos();
            writeln!(
                f,
                " ({avg:>10}ns average per call, {samples:>10} sample(s))",
            )?;
        }
        write!(f, "{ticks} sample(s)")?;

        Ok(())
    }
}
