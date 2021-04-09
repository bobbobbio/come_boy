// Copyright 2021 Remi Bernotavicius

use come_boy::rendering::Renderer;

pub trait Frontend: Send {
    fn run<R: Renderer>(self, renderer: &mut R);
}
