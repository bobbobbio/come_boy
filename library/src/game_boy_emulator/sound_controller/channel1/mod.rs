// Copyright 2021 Remi Bernotavicius

use self::memory_map::{Channel1MemoryMap, Channel1MemoryMapMut};
use super::{Channel, Frequency};
use crate::game_boy_emulator::default_clock_speed_hz;
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryAccessor, MemoryMappedHardware,
};
use crate::sound::SoundStream;
use crate::util::{Scheduler, TwosComplement};
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};

#[macro_use]
mod memory_map;

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom)]
#[repr(u8)]
pub enum SweepFlag {
    Time = 0b01110000,
    IncreaseOrDecrease = 0b00001000,
    Shift = 0b00000111,
}

impl FlagMask for SweepFlag {
    fn read_mask() -> u8 {
        Self::Time as u8 | Self::IncreaseOrDecrease as u8 | Self::Shift as u8
    }

    fn write_mask() -> u8 {
        Self::Time as u8 | Self::IncreaseOrDecrease as u8 | Self::Shift as u8
    }
}

#[derive(Default, Serialize, Deserialize)]
struct SquareWave {
    duty_timer: u8,
    duty: u8,
}

impl SquareWave {
    fn tick(&mut self) {
        self.duty_timer = (self.duty_timer + 1) % 8;
        if self.duty_timer == 0 {
            self.duty = (self.duty + 1) % 4;
        }
    }

    /*
     * 0 => 0b00000001,
     * 1 => 0b10000001,
     * 2 => 0b10000111,
     * 3 => 0b01111110,
     */
}

#[derive(Default, Serialize, Deserialize)]
pub struct Sweep {
    value: GameBoyFlags<SweepFlag>,
    timer: u8,
    enabled: bool,
    frequency: u16,
}

impl Sweep {
    pub fn set_value(&mut self, value: u8) {
        self.value.set_value(value);
    }

    fn period(&self) -> u8 {
        self.value.read_flag_value(SweepFlag::Time)
    }

    fn shift(&self) -> u8 {
        self.value.read_flag_value(SweepFlag::Shift)
    }

    fn decrease(&self) -> bool {
        self.value.read_flag(SweepFlag::IncreaseOrDecrease)
    }

    fn restart(&mut self, freq: &mut Frequency) {
        self.frequency = freq.read_value();
        self.timer = self.period();
        self.enabled = self.period() != 0 || self.shift() != 0;
        self.update_frequency(freq);
    }

    fn update_frequency(&mut self, freq: &mut Frequency) {
        if self.shift() != 0 {
            let mut change = self.frequency >> self.shift();
            if self.decrease() {
                change = change.twos_complement()
            }
            self.frequency += change;
            freq.set_value(self.frequency);
        }
    }

    fn tick(&mut self, freq: &mut Frequency) {
        self.timer = self.timer.saturating_sub(1);
        if self.timer == 0 && self.enabled && self.period() != 0 {
            self.update_frequency(freq)
        }
    }
}

impl MemoryMappedHardware for Sweep {
    fn read_value(&self, address: u16) -> u8 {
        MemoryMappedHardware::read_value(&self.value, address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        MemoryMappedHardware::set_value(&mut self.value, address, value);
    }
}

#[derive(Serialize, Deserialize)]
enum ToneAndSweepEvent {
    FrequencyTick,
    SweepTick,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Channel1 {
    pub sweep: Sweep,
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    waveform: SquareWave,
    scheduler: Scheduler<ToneAndSweepEvent>,
}

impl Channel1 {
    fn sweep_tick(&mut self, now: u64, freq: &mut Frequency) {
        self.sweep.tick(freq);

        let period = default_clock_speed_hz() / 128;
        self.scheduler
            .schedule(now + period as u64, ToneAndSweepEvent::SweepTick);
    }

    fn freq_tick(&mut self, now: u64, freq: &mut Frequency) {
        self.waveform.tick();

        let freq = freq.read_value();
        let period = default_clock_speed_hz() / (2048 - freq as u32) * 4;
        self.scheduler
            .schedule(now + period as u64, ToneAndSweepEvent::FrequencyTick);
    }
}

impl Channel for Channel1 {
    fn frequency_address() -> u16 {
        0xFF13
    }

    fn restart(&mut self, freq: &mut Frequency) {
        self.sweep.restart(freq);
    }

    fn deliver_events<S: SoundStream>(
        &mut self,
        now: u64,
        _sound_stream: &mut S,
        freq: &mut Frequency,
    ) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            match event {
                ToneAndSweepEvent::FrequencyTick => self.freq_tick(time, freq),
                ToneAndSweepEvent::SweepTick => self.sweep_tick(time, freq),
            }
        }
    }
}

impl MemoryMappedHardware for Channel1 {
    fn read_value(&self, address: u16) -> u8 {
        let memory_map = channel1_memory_map!(self);
        memory_map.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let mut memory_map = channel1_memory_map_mut!(self);
        memory_map.set_memory(address, value);
    }
}
