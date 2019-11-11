use amethyst::assets::PrefabData;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;
use amethyst::Error;
use serde::{Deserialize, Serialize};

#[derive(Component, PrefabData, Default, Debug, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct BallComponent;
