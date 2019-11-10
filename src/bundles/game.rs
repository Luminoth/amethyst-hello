use amethyst::audio::DjSystemDesc;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::prelude::*;

use crate::systems;
use crate::{Music, RunningState};

#[derive(Default)]
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            DjSystemDesc::new(|music: &mut Music| music.music.next()).build(world),
            "dj_system",
            &[],
        );
        builder.add(
            systems::MovementSystem::default().pausable(RunningState::Running),
            "movement_system",
            &[],
        );
        builder.add(
            systems::PaddleInputSystem::default().pausable(RunningState::Running),
            "paddle_input_system",
            &[],
        );
        builder.add(
            systems::BallCollisionSystem::default().pausable(RunningState::Running),
            "ball_collision_system",
            &["paddle_input_system", "movement_system"],
        );
        builder.add(
            systems::ScoreSystem::default().pausable(RunningState::Running),
            "score_system",
            &["movement_system"],
        );

        Ok(())
    }
}
