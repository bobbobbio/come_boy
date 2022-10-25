// copyright 2021 Remi Bernotavicius
use super::*;

impl GameBoyEmulator {
    fn run_inner(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        visitor: &mut impl FnMut(&Self),
        observer: &mut impl PerfObserver,
    ) -> core::result::Result<(), UserControl> {
        let mut underclocker = Underclocker::new(self.cpu.elapsed_cycles, ops.clock_speed_hz);
        let mut sometimes = ModuloCounter::new(SLEEP_INPUT_TICKS);

        visitor(self);

        while self.crashed().is_none() {
            self.tick_with_observer(ops, observer);

            visitor(self);

            // We can't do this every tick because it is too slow. So instead so only every so
            // often.
            if sometimes.incr() {
                underclocker.underclock(self.elapsed_cycles());
                self.read_key_events(ops)?;
            }
        }

        if self.cpu.crashed() {
            log::info!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
        Ok(())
    }

    pub(crate) fn run_with_options(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
        mut visitor: impl FnMut(&Self),
        observer: &mut impl PerfObserver,
    ) {
        loop {
            let res = self.run_inner(ops, &mut visitor, observer);
            match res {
                Err(UserControl::SaveStateLoaded) => {
                    if let Err(e) = self.load_state_from_storage(ops) {
                        log::info!("Failed to load state {:?}", e);
                    }
                }
                Err(UserControl::SpeedChange) => {}
                _ => break,
            }
        }
    }

    pub(crate) fn run(
        &mut self,
        ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ) {
        self.run_with_options(ops, |_| {}, &mut NullPerfObserver)
    }
}
