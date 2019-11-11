use amethyst::assets::PrefabData;
use amethyst::core::math::Vector3;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;
use amethyst::Error;
use serde::{Deserialize, Serialize};

#[derive(Component, PrefabData, Debug, Clone, Serialize, Deserialize)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
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
