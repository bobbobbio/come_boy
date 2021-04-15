// Copyright 2021 Remi Bernotavicius

use super::SoundStream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CpalSoundStream {
    sender: Sender<Vec<f32>>,
}

impl CpalSoundStream {
    pub fn new() -> Self {
        let (sender, receiver): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = channel();
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if let Ok(d) = receiver.try_recv() {
                data.clone_from_slice(&d[..]);
            } else {
                println!("no audio sample");
            }
        };
        let error_fn = |err| eprintln!("audio stream error: {}", err);
        let stream = device
            .build_output_stream(&config.into(), data_fn, error_fn)
            .unwrap();
        stream.play().unwrap();

        Self { sender }
    }
}

impl SoundStream for CpalSoundStream {
    fn play_sample(&mut self, data: &[f32]) {
        self.sender.send(data.to_owned()).unwrap();
    }
}
