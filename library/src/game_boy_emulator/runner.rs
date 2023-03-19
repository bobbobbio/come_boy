// copyright 2021 Remi Bernotavicius
use super::*;

impl GameBoyEmulator {
    fn run_inner<RendererT: Renderer, SoundStreamT: SoundStream, StorageT: PersistentStorage>(
        &mut self,
        ops: &mut GameBoyOps<RendererT, SoundStreamT, StorageT>,
        visitor: &mut impl FnMut(&Self, &mut GameBoyOps<RendererT, SoundStreamT, StorageT>),
        observer: &mut impl PerfObserver,
    ) -> core::result::Result<(), UserControl> {
        let mut underclocker = Underclocker::new(self.cpu.elapsed_cycles, ops.clock_speed_hz);
        let mut sometimes = ModuloCounter::new(SLEEP_INPUT_TICKS);

        visitor(self, ops);

        while self.crashed().is_none() {
            // We can't do this every tick because it is too slow. So instead so only every so
            // often.
            if sometimes.incr() {
                self.tick_with_observer(ops, observer);
                underclocker.underclock(self.elapsed_cycles());
                self.read_key_events(ops)?;
            } else {
                self.tick(ops);
            }

            visitor(self, ops);
        }

        if self.cpu.crashed() {
            log::info!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
        Ok(())
    }

    pub(crate) fn run_with_options<
        RendererT: Renderer,
        SoundStreamT: SoundStream,
        StorageT: PersistentStorage,
    >(
        &mut self,
        ops: &mut GameBoyOps<RendererT, SoundStreamT, StorageT>,
        mut visitor: impl FnMut(&Self, &mut GameBoyOps<RendererT, SoundStreamT, StorageT>),
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
        self.run_with_options(ops, |_, _| {}, &mut NullPerfObserver)
    }
}
