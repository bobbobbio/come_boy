// Copyright 2026 Remi Bernotavicius

use super::{Channel, Frequency, MAX_VOLUME};
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, MemoryAccessor, MemoryMappedHardware,
};
use crate::game_boy_emulator::{default_clock_speed_hz, GameBoyScheduler};
use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use serde_derive::{Deserialize, Serialize};

mod memory_map_mut;

#[derive(Debug, Default, Serialize, Deserialize)]
struct SquareWave {
    duty_timer: u8,
    duty: u8,
}

impl SquareWave {
    fn tick(&mut self) {
        if self.duty == 3 {
            return;
        }

        self.duty_timer = (self.duty_timer + 1) % 8;
        if self.duty_timer == 0 {
            self.duty += 1;
        }
    }

    fn waveform(&self) -> u8 {
        match self.duty {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => unreachable!(),
        }
    }

    fn restart(&mut self) {
        self.duty = 0;
        self.duty_timer = 0;
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize)]
pub enum Channel2Event {
    FrequencyTick,
    LengthTick,
    VolumeEnvelopeTick,
}

impl Channel2Event {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub(crate) fn deliver(
        self,
        channel: &mut Channel2,
        freq: &mut Frequency,
        using_length: bool,
        scheduler: &mut GameBoyScheduler,
        time: u64,
    ) {
        match self {
            Channel2Event::FrequencyTick => channel.freq_tick(freq, scheduler, time),
            Channel2Event::LengthTick => channel.length_tick(using_length, scheduler, time),
            Channel2Event::VolumeEnvelopeTick => channel.volume_envelope_tick(scheduler, time),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
enum LengthAndWaveDuty {
    WavePatternDuty = 0b11000000,
    SoundLength = 0b00111111,
}

impl FlagMask for LengthAndWaveDuty {
    fn read_mask() -> u8 {
        Self::WavePatternDuty as u8
    }

    fn write_mask() -> u8 {
        Self::WavePatternDuty as u8 | Self::SoundLength as u8
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LengthAndWave {
    length: u8,
    waveform: SquareWave,
}

impl LengthAndWave {
    pub fn set_value(&mut self, value: u8) {
        MemoryMappedHardware::set_value(self, 0, value);
    }

    fn restart(&mut self) {
        if self.length == 0 {
            self.length = 64;
        }
        self.waveform.restart();
    }

    fn length_tick(&mut self, using_length: bool, channel_enabled: &mut bool) {
        if using_length && self.length > 0 {
            self.length -= 1;
            if self.length == 0 {
                *channel_enabled = false;
            }
        }
    }
}

impl MemoryMappedHardware for LengthAndWave {
    fn read_value(&self, address: u16) -> u8 {
        let mut flags = GameBoyFlags::<LengthAndWaveDuty>::new();
        flags.set_flag_value(LengthAndWaveDuty::WavePatternDuty, self.waveform.duty);
        MemoryMappedHardware::read_value(&flags, address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let mut flags = GameBoyFlags::<LengthAndWaveDuty>::new();
        MemoryMappedHardware::set_value(&mut flags, address, value);
        self.waveform.duty = flags.read_flag_value(LengthAndWaveDuty::WavePatternDuty);
        let length = flags.read_flag_value(LengthAndWaveDuty::SoundLength);
        self.length = 64 - length;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, IntoEnumIterator)]
#[repr(u8)]
pub(crate) enum VolumeEnvelopeFlag {
    InitialVolume = 0b11110000,
    Direction = 0b00001000,
    SweepPace = 0b00000111,
}

impl FlagMask for VolumeEnvelopeFlag {
    fn read_mask() -> u8 {
        Self::InitialVolume as u8 | Self::Direction as u8 | Self::SweepPace as u8
    }

    fn write_mask() -> u8 {
        Self::InitialVolume as u8 | Self::Direction as u8 | Self::SweepPace as u8
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel2 {
    pub length_and_wave: LengthAndWave,
    pub volume_envelope: GameBoyFlags<VolumeEnvelopeFlag>,
    pub enabled: bool,
    pub volume_envelope_timer: u8,
    pub volume: u8,
}

impl Channel2 {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn freq_tick(&mut self, freq: &mut Frequency, scheduler: &mut GameBoyScheduler, now: u64) {
        self.length_and_wave.waveform.tick();

        let freq = freq.read_value();
        let period = (2048 - freq as u64) * 4;
        assert!(period > 0);
        scheduler.schedule(now + period, Channel2Event::FrequencyTick);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn length_tick(&mut self, using_length: bool, scheduler: &mut GameBoyScheduler, now: u64) {
        self.length_and_wave
            .length_tick(using_length, &mut self.enabled);

        let period = default_clock_speed_hz() / 256;
        scheduler.schedule(now + period as u64, Channel2Event::LengthTick);
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn volume_envelope_tick(&mut self, scheduler: &mut GameBoyScheduler, now: u64) {
        let period = self
            .volume_envelope
            .read_flag_value(VolumeEnvelopeFlag::SweepPace);

        if period > 0 {
            self.volume_envelope_timer = self.volume_envelope_timer.saturating_sub(1);
            if self.volume_envelope_timer == 0 {
                self.volume_envelope_timer = period;

                let direction = self
                    .volume_envelope
                    .read_flag(VolumeEnvelopeFlag::Direction);
                if direction {
                    if self.volume < MAX_VOLUME {
                        self.volume += 1;
                    }
                } else {
                    self.volume = self.volume.saturating_sub(1);
                }
            }
        }

        let period_ticks = default_clock_speed_hz() / 64;
        scheduler.schedule(now + period_ticks as u64, Channel2Event::VolumeEnvelopeTick);
    }

    pub fn waveform(&self) -> u8 {
        if self.enabled {
            self.length_and_wave.waveform.waveform()
        } else {
            0x00
        }
    }

    pub(crate) fn schedule_initial_events(&mut self, scheduler: &mut GameBoyScheduler, now: u64) {
        scheduler.schedule(now, Channel2Event::FrequencyTick);
        scheduler.schedule(now, Channel2Event::LengthTick);
        scheduler.schedule(now, Channel2Event::VolumeEnvelopeTick);
    }
}

impl Channel for Channel2 {
    const FREQUENCY_ADDRESS: u16 = 0xFF18;

    fn restart(&mut self, _freq: &mut Frequency) {
        self.enabled = true;
        self.length_and_wave.restart();

        self.volume = self
            .volume_envelope
            .read_flag_value(VolumeEnvelopeFlag::InitialVolume);
        let period = self
            .volume_envelope
            .read_flag_value(VolumeEnvelopeFlag::SweepPace);
        self.volume_envelope_timer = period;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn enable(&mut self) {
        self.enabled = true;
    }
}

impl MemoryMappedHardware for Channel2 {
    fn read_value(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }
}
