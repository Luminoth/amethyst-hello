use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::Error;
use serde::{Deserialize, Serialize};

use crate::components::{BoundingBoxComponent, PaddleComponent};

#[derive(PrefabData, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PaddlePrefab {
    transform: Transform,
    bounds: BoundingBoxComponent,
    paddle: PaddleComponent,
}
