// Copyright 2021 Remi Bernotavicius

pub mod cpal;

pub trait SoundStream {
    fn play_sample(&mut self, data: &[f32]);
}

pub struct NullSoundStream;

impl SoundStream for NullSoundStream {
    fn play_sample(&mut self, _data: &[f32]) {}
}
