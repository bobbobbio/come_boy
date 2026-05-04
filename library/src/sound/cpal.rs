// Copyright 2021 Remi Bernotavicius

use super::SoundStream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use ringbuf::{
    traits::{Consumer as _, Producer as _, Split},
    HeapRb,
};

pub struct CpalSoundStream {
    producer: <HeapRb<f32> as Split>::Prod,
    _stream: Stream,
    sample_rate: u32,
    channels: u16,
}

impl Default for CpalSoundStream {
    fn default() -> Self {
        Self::new()
    }
}

impl CpalSoundStream {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        // buffer size: enough for ~100ms of audio to absorb timing jitter
        let buf_size = (sample_rate as usize * channels as usize) / 10;
        let rb = HeapRb::<f32>::new(buf_size);
        let (producer, mut consumer) = rb.split();

        let data_fn = move |data_out: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data_out.iter_mut() {
                *sample = consumer.try_pop().unwrap_or(0.0);
            }
        };
        let error_fn = |err| log::error!("audio stream error: {}", err);
        let stream = device
            .build_output_stream(&config.into(), data_fn, error_fn, None)
            .unwrap();
        stream.play().unwrap();

        Self {
            producer,
            _stream: stream,
            sample_rate,
            channels,
        }
    }
}

impl SoundStream for CpalSoundStream {
    fn play_sample(&mut self, data: &[f32]) {
        for &sample in data {
            let _ = self.producer.try_push(sample);
        }
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }
}
