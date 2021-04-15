// Copyright 2018 Remi Bernotavicius

use self::memory_map::{SoundControllerMemoryMap, SoundControllerMemoryMapMut};
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, GameBoyRegister16, MemoryAccessor,
    MemoryMappedHardware,
};
use crate::sound::SoundStream;
use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;
use enum_utils::ReprFrom;
use serde_derive::{Deserialize, Serialize};

#[macro_use]
mod memory_map;

mod channel1;
mod channel2;
mod channel3;
mod channel4;

trait Channel: MemoryMappedHardware {
    fn frequency_address() -> u16;
    fn restart(&mut self, freq: &mut Frequency);
    fn deliver_events<S: SoundStream>(
        &mut self,
        now: u64,
        sound_stream: &mut S,
        freq: &mut Frequency,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, ReprFrom)]
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

type Frequency = GameBoyRegister16;

#[derive(Default, Serialize, Deserialize)]
struct ChannelController<C> {
    channel: C,
    counter_selection: bool,
    frequency: Frequency,
}

impl<C: Channel> ChannelController<C> {
    fn write_high_byte(&mut self, value: u8) {
        let mut control = GameBoyFlags::<ChannelHighByte>::new();
        MemoryMappedHardware::set_value(&mut control, 0, value);

        self.counter_selection = control.read_flag(ChannelHighByte::CounterSelection);
        let freq_high = control.read_flag_value(ChannelHighByte::FrequencyHigh);
        MemoryMappedHardware::set_value(&mut self.frequency, 1, freq_high);

        let restart = control.read_flag(ChannelHighByte::Restart);
        if restart {
            self.channel.restart(&mut self.frequency);
        }
    }

    fn write_low_byte(&mut self, value: u8) {
        MemoryMappedHardware::set_value(&mut self.frequency, 0, value);
    }

    fn read_high_byte(&self) -> u8 {
        let mut control = GameBoyFlags::<ChannelHighByte>::new();
        control.set_flag(ChannelHighByte::CounterSelection, self.counter_selection);
        MemoryMappedHardware::read_value(&control, 0)
    }

    fn deliver_events<S: SoundStream>(&mut self, now: u64, sound_stream: &mut S) {
        self.channel
            .deliver_events(now, sound_stream, &mut self.frequency);
    }
}

impl<C: Channel> MemoryMappedHardware for ChannelController<C> {
    fn read_value(&self, address: u16) -> u8 {
        if address == C::frequency_address() + 1 {
            self.read_high_byte()
        } else if address == C::frequency_address() {
            0xFF
        } else {
            MemoryMappedHardware::read_value(&self.channel, address)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address == C::frequency_address() + 1 {
            self.write_high_byte(value);
        } else if address == C::frequency_address() {
            self.write_low_byte(value);
        } else {
            MemoryMappedHardware::set_value(&mut self.channel, address, value)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SoundController {
    channel1: ChannelController<Channel1>,
    channel2: ChannelController<Channel2>,
    channel3: ChannelController<Channel3>,
    channel4: Channel4,
    channel_control: GameBoyRegister,
    output_terminal: GameBoyRegister,
    enabled: GameBoyRegister,
}

impl MemoryMappedHardware for SoundController {
    fn read_value(&self, address: u16) -> u8 {
        let memory_map = sound_controller_memory_map!(self);
        memory_map.read_memory(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address == 0xFF26 {
            if value & 0x80 == 0x80 {
                self.enable();
            } else {
                self.disable();
            }
        } else {
            let mut memory_map = sound_controller_memory_map_mut!(self);
            memory_map.set_memory(address, value);
        }
    }
}

impl SoundController {
    pub fn set_state_post_bios(&mut self) {
        self.channel1.channel.sweep.set_value(0x80);
        self.channel1.channel.sound_length.set_value(0xBF);
        self.channel1.channel.volume_envelope.set_value(0xF3);
        self.channel1.frequency.set_value(0xBFFF);

        self.channel2.channel.sound_length.set_value(0x3F);
        self.channel2.channel.volume_envelope.set_value(0x00);
        self.channel2.frequency.set_value(0xBFFF);

        self.channel3.channel.enabled.set_value(0x7F);
        self.channel3.channel.sound_length.set_value(0xFF);
        self.channel3.channel.output_level.set_value(0x9F);
        self.channel3.frequency.set_value(0xBFFF);
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
        self.enabled.set_value(0xF1);
    }

    fn enable(&mut self) {
        self.enabled.set_value(0xF0);
    }

    fn disable(&mut self) {
        self.enabled.set_value(0x70);
        self.channel_control.set_value(0x00);
        self.output_terminal.set_value(0x00);

        // Why does this happen?
        self.channel1.channel.sound_length.set_value(0x3F);
        self.channel1.channel.volume_envelope.set_value(0x00);
    }

    pub fn deliver_events<S: SoundStream>(&mut self, now: u64, sound_stream: &mut S) {
        self.channel1.deliver_events(now, sound_stream);
    }
}
