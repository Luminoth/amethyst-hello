use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::Error;
use serde::{Deserialize, Serialize};

use crate::components::{BallComponent, BoundingBoxComponent, PhysicalComponent};

#[derive(PrefabData, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BallPrefab {
    transform: Transform,
    physical: PhysicalComponent,
    bounds: BoundingBoxComponent,
    ball: BallComponent,
}
