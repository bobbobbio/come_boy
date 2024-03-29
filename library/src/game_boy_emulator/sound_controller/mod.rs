// Copyright 2018 Remi Bernotavicius

use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, GameBoyRegister16, MemoryAccessor,
    MemoryMappedHardware,
};
use crate::game_boy_emulator::{default_clock_speed_hz, GameBoyEmulatorEvent, GameBoyScheduler};
use crate::sound::SoundStream;
use alloc::vec::Vec;
use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;
use core::fmt;
use num_enum::IntoPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

mod channel1;
mod channel2;
mod channel3;
mod channel4;
mod memory_map_mut;

trait Channel: MemoryMappedHardware {
    const FREQUENCY_ADDRESS: u16;

    fn restart(&mut self, freq: &mut Frequency);
    fn enabled(&self) -> bool;
    fn disable(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
enum ChannelHighByte {
    Restart = 0b10000000,
    CounterSelection = 0b01000000,
    FrequencyHigh = 0b00000111,
}

impl FlagMask for ChannelHighByte {
    fn read_mask() -> u8 {
        Self::CounterSelection as u8
    }

    fn write_mask() -> u8 {
        Self::Restart as u8 | Self::CounterSelection as u8 | Self::FrequencyHigh as u8
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct Frequency(GameBoyRegister16);

impl fmt::Debug for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Frequency {
    const MASK: u16 = (ChannelHighByte::FrequencyHigh as u16) << 8 | 0xFFu16;

    fn read_value(&self) -> u16 {
        self.0.read_value()
    }

    fn set_value(&mut self, value: u16) {
        self.0.set_value(value & Self::MASK)
    }

    fn try_add(&self, to_add: u16) -> Option<Self> {
        let existing = self.read_value();
        if let Some(res) = existing.checked_add(to_add) {
            if res <= Self::MASK {
                let mut new = Self::default();
                new.0.set_value(res);
                return Some(new);
            }
        }
        None
    }
}

impl MemoryMappedHardware for Frequency {
    fn read_value(&self, address: u16) -> u8 {
        MemoryMappedHardware::read_value(&self.0, address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let is_high_byte = address == 1;
        if is_high_byte {
            assert_eq!(value & !(ChannelHighByte::FrequencyHigh as u8), 0);
        }
        MemoryMappedHardware::set_value(&mut self.0, address, value)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ChannelController<C> {
    channel: C,
    using_length: bool,
    freq: Frequency,
}

impl<C: Channel> ChannelController<C> {
    fn enabled(&self) -> bool {
        self.channel.enabled()
    }

    fn disable(&mut self) {
        self.channel.disable()
    }

    fn write_high_byte(&mut self, value: u8) {
        let mut control = GameBoyFlags::<ChannelHighByte>::new();
        MemoryMappedHardware::set_value(&mut control, 0, value);

        self.using_length = control.read_flag(ChannelHighByte::CounterSelection);
        let freq_high = control.read_flag_value(ChannelHighByte::FrequencyHigh);
        MemoryMappedHardware::set_value(&mut self.freq, 1, freq_high);

        let restart = control.read_flag(ChannelHighByte::Restart);
        if restart {
            self.channel.restart(&mut self.freq);
        }
    }

    fn write_low_byte(&mut self, value: u8) {
        MemoryMappedHardware::set_value(&mut self.freq, 0, value);
    }

    fn read_high_byte(&self) -> u8 {
        let mut control = GameBoyFlags::<ChannelHighByte>::new();
        control.set_flag(ChannelHighByte::CounterSelection, self.using_length);
        MemoryMappedHardware::read_value(&control, 0)
    }
}

impl<C: Channel> MemoryMappedHardware for ChannelController<C> {
    fn read_value(&self, address: u16) -> u8 {
        if address == C::FREQUENCY_ADDRESS + 1 {
            self.read_high_byte()
        } else if address == C::FREQUENCY_ADDRESS {
            0xFF
        } else {
            MemoryMappedHardware::read_value(&self.channel, address)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address == C::FREQUENCY_ADDRESS + 1 {
            self.write_high_byte(value);
        } else if address == C::FREQUENCY_ADDRESS {
            self.write_low_byte(value);
        } else {
            MemoryMappedHardware::set_value(&mut self.channel, address, value)
        }
    }
}

#[derive(Serialize, Deserialize, IntoStaticStr)]
pub enum SoundControllerEvent {
    MixerTick,
    Channel1(channel1::Channel1Event),
}

impl From<channel1::Channel1Event> for GameBoyEmulatorEvent {
    fn from(e: channel1::Channel1Event) -> Self {
        SoundControllerEvent::Channel1(e).into()
    }
}

impl SoundControllerEvent {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub(crate) fn deliver(
        self,
        controller: &mut SoundController,
        sound_stream: &mut impl SoundStream,
        scheduler: &mut GameBoyScheduler,
        time: u64,
    ) {
        match self {
            SoundControllerEvent::MixerTick => controller.mixer_tick(sound_stream, scheduler, time),
            SoundControllerEvent::Channel1(e) => e.deliver(
                &mut controller.channel1.channel,
                &mut controller.channel1.freq,
                controller.channel1.using_length,
                scheduler,
                time,
            ),
        }
    }
}

#[derive(Default)]
struct MixerBuffer(Vec<f32>);

impl fmt::Debug for MixerBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MixerBuffer")
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SoundController {
    channel1: ChannelController<Channel1>,
    channel2: ChannelController<Channel2>,
    channel3: ChannelController<Channel3>,
    channel4: Channel4,
    channel_control: GameBoyRegister,
    output_terminal: GameBoyRegister,
    enabled: bool,

    #[serde(skip)]
    mixer_buffer: MixerBuffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
enum SoundEnable {
    All = 0b10000000,
    Channel4 = 0b00001000,
    Channel3 = 0b00000100,
    Channel2 = 0b00000010,
    Channel1 = 0b00000001,
}

impl FlagMask for SoundEnable {
    fn read_mask() -> u8 {
        Self::All as u8
            | Self::Channel4 as u8
            | Self::Channel3 as u8
            | Self::Channel2 as u8
            | Self::Channel1 as u8
    }

    fn write_mask() -> u8 {
        Self::All as u8
    }
}

impl MemoryMappedHardware for SoundController {
    fn read_value(&self, address: u16) -> u8 {
        if address == 0xFF26 {
            self.read_enable_value()
        } else {
            self.read_memory(address)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address == 0xFF2 {
            self.set_enable_value(value);
        } else {
            self.set_memory(address, value);
        }
    }
}

impl SoundController {
    fn read_enable_value(&self) -> u8 {
        let mut enabled = GameBoyFlags::<SoundEnable>::new();

        enabled.set_flag(SoundEnable::All, self.enabled);
        enabled.set_flag(SoundEnable::Channel1, self.channel1.enabled());
        enabled.set_flag(SoundEnable::Channel2, self.channel2.enabled());
        enabled.set_flag(SoundEnable::Channel3, self.channel3.enabled());
        enabled.set_flag(SoundEnable::Channel4, self.channel4.enabled());

        MemoryMappedHardware::read_value(&enabled, 0)
    }

    fn set_enable_value(&mut self, value: u8) {
        let mut written = GameBoyFlags::<SoundEnable>::new();
        MemoryMappedHardware::set_value(&mut written, 0, value);
        if written.read_flag(SoundEnable::All) {
            self.disable();
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.channel1.disable();
        self.channel2.disable();
        self.channel3.disable();
        self.channel4.disable();
    }

    pub fn set_state_post_bios(&mut self) {
        self.channel1.channel.sweep.set_value(0x80);
        self.channel1.channel.length_and_wave.set_value(0xBF);
        self.channel1.channel.volume_envelope.set_value(0xF3);
        self.channel1.freq.set_value(0xBFFF);
        self.channel1.channel.enabled = true;

        self.channel2.channel.sound_length.set_value(0x3F);
        self.channel2.channel.volume_envelope.set_value(0x00);
        self.channel2.freq.set_value(0xBFFF);

        self.channel3.channel.enabled.set_value(0x7F);
        self.channel3.channel.sound_length.set_value(0xFF);
        self.channel3.channel.output_level.set_value(0x9F);
        self.channel3.freq.set_value(0xBFFF);
        self.channel3.channel.wave_pattern.clone_from_slice(&[
            0x71, 0x72, 0xD5, 0x91, 0x58, 0xBB, 0x2A, 0xFA, 0xCF, 0x3C, 0x54, 0x75, 0x48, 0xCF,
            0x8F, 0xD9,
        ]);

        self.channel4.sound_length.set_value(0xFF);
        self.channel4.volume_envelope.set_value(0x00);
        self.channel4.polynomial_counter.set_value(0x00);
        self.channel4.counter.set_value(0xBF);

        self.channel_control.set_value(0x77);
        self.output_terminal.set_value(0xF3);
        self.enabled = true;
    }

    pub(crate) fn schedule_initial_events(&mut self, scheduler: &mut GameBoyScheduler, now: u64) {
        // Completely disabled sound controller, it is not functioning correctly and is causing
        // problems on picosystem
        if false {
            self.channel1
                .channel
                .schedule_initial_events(scheduler, now);
            scheduler.schedule(now, SoundControllerEvent::MixerTick);
        }
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn mixer_tick(
        &mut self,
        sound_stream: &mut impl SoundStream,
        scheduler: &mut GameBoyScheduler,
        now: u64,
    ) {
        let sample_rate_hz = sound_stream.sample_rate();
        let num_channels = sound_stream.channels() as usize;
        let freq_hz =
            default_clock_speed_hz() / ((2048 - self.channel1.freq.read_value() as u32) * 32);
        let elong = ((sample_rate_hz / freq_hz) as usize) * num_channels;

        let waveform = self.channel1.channel.waveform();
        self.mixer_buffer.0.resize(8 * elong, 0.0);
        for (i, item) in self.mixer_buffer.0.iter_mut().enumerate() {
            *item = ((waveform >> (i / elong)) & 0x1) as f32;
        }
        sound_stream.play_sample(&self.mixer_buffer.0[..]);

        let period = default_clock_speed_hz() / freq_hz;

        scheduler.schedule(now + period as u64, SoundControllerEvent::MixerTick);
    }
}

#[cfg(test)]
mod tests;
