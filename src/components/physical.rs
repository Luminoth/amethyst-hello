use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

#[derive(Component)]
pub struct PhysicalComponent {
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
}

impl PhysicalComponent {
    #[allow(dead_code)]
    fn new(velocity: Vector3<f32>, acceleration: Vector3<f32>) -> Self {
        Self {
            velocity,
            acceleration,
        }
    }
}

impl Default for PhysicalComponent {
    fn default() -> Self {
        Self {
            velocity: Vector3::from_element(0.0),
            acceleration: Vector3::from_element(0.0),
        }
    }
}
