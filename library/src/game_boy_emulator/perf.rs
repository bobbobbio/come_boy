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

pub trait Instant: Sized {
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

impl<InstantT> PerfStats<InstantT> {
    fn get_avg(&self, tag: &str) -> Option<Duration> {
        self.stats.get(tag).map(|&(d, n)| d / n)
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

fn stat_sorter(
    (&tag, &(mut dur, n)): (&&'static str, &(Duration, u32)),
    nothing: Duration,
    ticks: u32,
) -> (&'static str, Duration) {
    assert!(n != 0);
    dur = dur.saturating_sub(nothing);

    if ticks > n {
        (tag, dur / n / (ticks / n))
    } else {
        (tag, dur / n * (n / ticks))
    }
}

impl<InstantT> fmt::Display for PerfStats<InstantT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ticks = self.num_ticks;
        assert!(ticks != 0);

        let nothing = self.get_avg("nothing").unwrap_or(Duration::ZERO);
        let mut sorted_stats: Vec<_> = self
            .stats
            .iter()
            .filter(|e| e.0 != &"nothing")
            .map(|e| stat_sorter(e, nothing, ticks))
            .collect();
        sorted_stats.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));

        writeln!(f)?;
        let total_duration: Duration = sorted_stats.iter().map(|(_, d)| d).sum();
        writeln!(
            f,
            "{:<20}: {:>7}ns apt",
            "sum of all",
            total_duration.as_nanos()
        )?;

        for (tag, duration) in &sorted_stats {
            if tag == &"nothing" {
                continue;
            }

            let pct = duration.as_nanos() * 100 / total_duration.as_nanos();
            write!(f, "{tag:<20}: {:>7}ns apt ({pct:>2}%)", duration.as_nanos())?;

            if f.alternate() {
                let &(dur, samples) = self.stats.get(*tag).unwrap();
                let avg = (dur / samples).as_nanos();
                write!(
                    f,
                    " ({avg:>10}ns average per call, {samples:>10} sample(s))",
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "{ticks} sample(s)")?;

        Ok(())
    }
}
