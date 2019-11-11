use amethyst::assets::PrefabData;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;
use amethyst::Error;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub enum PaddleSide {
    #[derivative(Default)]
    Left,
    Right,
}

#[derive(Component, PrefabData, Default, Debug, Clone, Serialize, Deserialize)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PaddleComponent {
    pub side: PaddleSide,
}

impl PaddleComponent {
    pub fn new(side: PaddleSide) -> Self {
        Self { side }
    }
}
