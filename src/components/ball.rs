use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;

#[derive(Component)]
pub struct BallComponent {
    pub velocity: [f32; 2],
}

impl Default for BallComponent {
    fn default() -> Self {
        Self {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        }
    }
}
