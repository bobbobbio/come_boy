// Copyright 2021 Remi Bernotavicius

use super::SoundStream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use std::sync::{Arc, Mutex};

struct Sample {
    data: Vec<f32>,
    pos: usize,
}

impl Sample {
    fn new() -> Self {
        Self {
            data: vec![],
            pos: 0,
        }
    }

    fn set(&mut self, data: &[f32]) {
        self.data = data.to_owned();
        self.pos = 0;
    }

    fn fill(&mut self, data_out: &mut [f32]) {
        if self.data.is_empty() {
            return;
        }

        let mut pos = 0;

        while pos < data_out.len() {
            let source = &self.data[self.pos..];
            let sink = &mut data_out[pos..];

            let end = std::cmp::min(source.len(), sink.len());
            sink[..end].clone_from_slice(&source[..end]);

            self.pos = (self.pos + end) % self.data.len();
            pos += end;
        }
    }
}

pub struct CpalSoundStream {
    sample: Arc<Mutex<Sample>>,
    _stream: Stream,
    sample_rate: u32,
    channels: u16,
}

impl CpalSoundStream {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        let sample = Arc::new(Mutex::new(Sample::new()));
        let their_sample = sample.clone();
        let data_fn = move |data_out: &mut [f32], _: &cpal::OutputCallbackInfo| {
            their_sample.lock().unwrap().fill(data_out);
        };
        let error_fn = |err| eprintln!("audio stream error: {}", err);
        let stream = device
            .build_output_stream(&config.into(), data_fn, error_fn)
            .unwrap();
        stream.play().unwrap();

        Self {
            sample,
            _stream: stream,
            sample_rate,
            channels,
        }
    }
}

impl SoundStream for CpalSoundStream {
    fn play_sample(&mut self, data: &[f32]) {
        self.sample.lock().unwrap().set(data);
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }
}
