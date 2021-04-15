// Copyright 2021 Remi Bernotavicius

use come_boy::rendering::Renderer;
use come_boy::sound::SoundStream;

pub trait Frontend: Send {
    fn run<R: Renderer, S: SoundStream>(self, renderer: &mut R, sound_stream: &mut S);
}
