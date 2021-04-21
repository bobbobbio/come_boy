// Copyright 2021 Remi Bernotavicius

pub mod cpal;

pub trait SoundStream {
    fn play_sample(&mut self, data: &[f32]);
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
}

pub struct NullSoundStream;

impl SoundStream for NullSoundStream {
    fn play_sample(&mut self, _data: &[f32]) {}
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        13_1072
    }
}
