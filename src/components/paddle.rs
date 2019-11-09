use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

#[derive(PartialEq, Eq)]
pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct PaddleComponent {
    pub side: PaddleSide,
}

impl PaddleComponent {
    pub fn new(side: PaddleSide) -> Self {
        Self { side }
    }
}
