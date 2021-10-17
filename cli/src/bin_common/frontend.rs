// Copyright 2021 Remi Bernotavicius

use come_boy::rendering::Renderer;
use come_boy::sound::SoundStream;
use come_boy::storage::PersistentStorage;

pub trait Frontend: Send {
    fn run(
        self,
        renderer: &mut impl Renderer,
        sound_stream: &mut impl SoundStream,
        storage: &mut impl PersistentStorage,
    );
}
