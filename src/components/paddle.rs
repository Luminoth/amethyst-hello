use amethyst::ecs::{Component, DenseVecStorage};

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_SPEED: f32 = 50.0;

#[derive(PartialEq, Eq)]
pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct PaddleComponent {
    pub side: PaddleSide,
    pub width: f32,
    pub height: f32,
}

impl PaddleComponent {
    pub fn new(side: PaddleSide) -> Self {
        Self {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}
