use amethyst::assets::Loader;
use amethyst::audio::{OggFormat, SourceHandle};
use amethyst::ecs::prelude::*;

pub fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}
