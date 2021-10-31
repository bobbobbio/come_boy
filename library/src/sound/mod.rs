// Copyright 2021 Remi Bernotavicius

#[cfg(feature = "sound")]
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

impl<T: SoundStream> SoundStream for &mut T {
    fn play_sample(&mut self, data: &[f32]) {
        (**self).play_sample(data)
    }

    fn channels(&self) -> u16 {
        (**self).channels()
    }

    fn sample_rate(&self) -> u32 {
        (**self).sample_rate()
    }
}
