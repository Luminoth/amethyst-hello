use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

pub const PADDLE_SPEED: f32 = 50.0;

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
