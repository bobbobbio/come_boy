// Copyright 2021 Remi Bernotavicius

use super::{Channel, Frequency};
use crate::game_boy_emulator::default_clock_speed_hz;
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryAccessor, MemoryMappedHardware,
};
use crate::util::{Scheduler, TwosComplement};
use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use serde_derive::{Deserialize, Serialize};

mod memory_map_mut;

#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, IntoEnumIterator)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Sweep {
    value: GameBoyFlags<SweepFlag>,
    timer: u8,
    enabled: bool,
    freq: Frequency,
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

    fn restart(&mut self, freq: &mut Frequency, channel_enabled: &mut bool) {
        self.freq = freq.clone();
        self.timer = self.period();
        self.enabled = self.period() != 0 || self.shift() != 0;

        if self.shift() != 0 {
            self.update_frequency(freq, channel_enabled);
        }
    }

    fn calculate_frequency_change(&self) -> u16 {
        assert!(self.shift() > 0);
        let change = self.freq.read_value() >> self.shift();
        if self.decrease() {
            change.twos_complement()
        } else {
            change
        }
    }

    fn check_overflow(&self, channel_enabled: &mut bool) {
        let change = self.calculate_frequency_change();
        if self.freq.try_add(change).is_none() {
            *channel_enabled = false;
        }
    }

    fn update_frequency(&mut self, freq: &mut Frequency, channel_enabled: &mut bool) {
        let change = self.calculate_frequency_change();
        if let Some(new_freq) = self.freq.try_add(change) {
            self.freq = new_freq.clone();
            *freq = new_freq;
        } else {
            *channel_enabled = false;
        }
    }

    fn timer_fire(&mut self, freq: &mut Frequency, channel_enabled: &mut bool) {
        if self.shift() == 0 {
            return;
        }

        self.update_frequency(freq, channel_enabled);
        self.check_overflow(channel_enabled);
    }

    fn tick(&mut self, freq: &mut Frequency, channel_enabled: &mut bool) {
        self.timer = self.timer.saturating_sub(1);
        if self.timer == 0 && self.enabled && self.period() != 0 {
            self.timer_fire(freq, channel_enabled)
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
enum Channel1Event {
    FrequencyTick,
    SweepTick,
    LengthTick,
}

#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel1 {
    pub sweep: Sweep,
    pub length_and_wave: LengthAndWave,
    pub volume_envelope: GameBoyRegister,
    scheduler: Scheduler<Channel1Event>,
    pub enabled: bool,
}

impl Channel1 {
    fn sweep_tick(&mut self, now: u64, freq: &mut Frequency) {
        self.sweep.tick(freq, &mut self.enabled);

        let period = default_clock_speed_hz() / 128;
        self.scheduler
            .schedule(now + period as u64, Channel1Event::SweepTick);
    }

    fn freq_tick(&mut self, now: u64, freq: &mut Frequency) {
        self.length_and_wave.waveform.tick();

        let freq = freq.read_value();
        let period = (2048 - freq as u64) * 4;
        assert!(period > 0);
        self.scheduler
            .schedule(now + period, Channel1Event::FrequencyTick);
    }

    fn length_tick(&mut self, now: u64, using_length: bool) {
        self.length_and_wave
            .length_tick(using_length, &mut self.enabled);

        let period = default_clock_speed_hz() / 256;
        self.scheduler
            .schedule(now + period as u64, Channel1Event::LengthTick);
    }

    pub fn waveform(&self) -> u8 {
        if self.enabled {
            self.length_and_wave.waveform.waveform()
        } else {
            0x00
        }
    }

    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now, Channel1Event::FrequencyTick);
        self.scheduler.schedule(now, Channel1Event::SweepTick);
        self.scheduler.schedule(now, Channel1Event::LengthTick);
    }
}

impl Channel for Channel1 {
    const FREQUENCY_ADDRESS: u16 = 0xFF13;

    fn restart(&mut self, freq: &mut Frequency) {
        self.enabled = true;
        self.sweep.restart(freq, &mut self.enabled);
        self.length_and_wave.restart();
    }

    fn deliver_events(&mut self, now: u64, freq: &mut Frequency, using_length: bool) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            match event {
                Channel1Event::FrequencyTick => self.freq_tick(time, freq),
                Channel1Event::SweepTick => self.sweep_tick(time, freq),
                Channel1Event::LengthTick => self.length_tick(time, using_length),
            }
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl MemoryMappedHardware for Channel1 {
    fn read_value(&self, address: u16) -> u8 {
        self.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.set_memory(address, value);
    }
}
