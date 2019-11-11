use amethyst::assets::PrefabLoaderSystemDesc;
use amethyst::audio::DjSystemDesc;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::prelude::*;

use crate::prefabs::{BallPrefab, PaddlePrefab};
use crate::systems::{BallCollisionSystem, MovementSystem, PaddleInputSystem, ScoreSystem};
use crate::{Music, RunningState};

#[derive(Default)]
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // prefab loaders
        builder.add(
            PrefabLoaderSystemDesc::<BallPrefab>::default().build(world),
            "",
            &[],
        );
        builder.add(
            PrefabLoaderSystemDesc::<PaddlePrefab>::default().build(world),
            "",
            &[],
        );

        builder.add(
            DjSystemDesc::new(|music: &mut Music| music.music.next()).build(world),
            "",
            &[],
        );

        builder.add(
            MovementSystem::default().pausable(RunningState::Running),
            "movement_system",
            &[],
        );
        builder.add(
            PaddleInputSystem::default().pausable(RunningState::Running),
            "paddle_input_system",
            &[],
        );
        builder.add(
            BallCollisionSystem::default().pausable(RunningState::Running),
            "ball_collision_system",
            &["paddle_input_system", "movement_system"],
        );
        builder.add(
            ScoreSystem::default().pausable(RunningState::Running),
            "score_system",
            &["movement_system"],
        );

        Ok(())
    }
}
