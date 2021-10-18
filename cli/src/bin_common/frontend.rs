// Copyright 2021 Remi Bernotavicius

use come_boy::rendering::Renderer;
use come_boy::sound::SoundStream;

pub trait Frontend: Send {
    fn run(self, renderer: &mut impl Renderer, sound_stream: &mut impl SoundStream);
}
