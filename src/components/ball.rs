use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BallComponent;
