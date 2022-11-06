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
