use amethyst::ecs::{Component, DenseVecStorage};

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(Component)]
pub struct BallComponent {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Default for BallComponent {
    fn default() -> Self {
        Self {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            radius: BALL_RADIUS,
        }
    }
}
