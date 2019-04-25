// Copyright 2018 Remi Bernotavicius

use game_boy_emulator::{GameBoyRegister, MemoryChunk};

#[derive(Default)]
pub struct ToneAndSweep {
    pub sweep: GameBoyRegister,
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    pub frequency_low: GameBoyRegister,
    pub frequency_high: GameBoyRegister,
}

#[derive(Default)]
pub struct Tone {
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    pub frequency_low: GameBoyRegister,
    pub frequency_high: GameBoyRegister,
}

pub struct WaveOutput {
    pub enabled: GameBoyRegister,
    pub sound_length: GameBoyRegister,
    pub output_level: GameBoyRegister,
    pub frequency_low: GameBoyRegister,
    pub frequency_high: GameBoyRegister,
    pub wave_pattern: MemoryChunk,
}

impl Default for WaveOutput {
    fn default() -> Self {
        WaveOutput {
            enabled: Default::default(),
            sound_length: Default::default(),
            output_level: Default::default(),
            frequency_low: Default::default(),
            frequency_high: Default::default(),
            wave_pattern: MemoryChunk::from_range(0..0x10),
        }
    }
}

#[derive(Default)]
pub struct Noise {
    pub sound_length: GameBoyRegister,
    pub volume_envelope: GameBoyRegister,
    pub polynomial_counter: GameBoyRegister,
    pub counter: GameBoyRegister,
}

#[derive(Default)]
pub struct SoundController {
    pub channel1: ToneAndSweep,
    pub channel2: Tone,
    pub channel3: WaveOutput,
    pub channel4: Noise,
    pub channel_control: GameBoyRegister,
    pub output_terminal: GameBoyRegister,
    pub enabled: GameBoyRegister,
}

impl SoundController {
    pub fn set_state_post_bios(&mut self) {
        self.channel1.sweep.set_value(0x80);
        self.channel1.sound_length.set_value(0xBF);
        self.channel1.volume_envelope.set_value(0xF3);
        self.channel1.frequency_low.set_value(0xFF);
        self.channel1.frequency_high.set_value(0xBF);

        self.channel2.sound_length.set_value(0x3F);
        self.channel2.volume_envelope.set_value(0x00);
        self.channel2.frequency_low.set_value(0xFF);
        self.channel2.frequency_high.set_value(0xBF);

        self.channel3.enabled.set_value(0x7F);
        self.channel3.sound_length.set_value(0xFF);
        self.channel3.output_level.set_value(0x9F);
        self.channel3.frequency_low.set_value(0xFF);
        self.channel3.frequency_high.set_value(0xBF);
        self.channel3.wave_pattern.clone_from_slice(&[
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
}
