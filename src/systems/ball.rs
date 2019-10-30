use std::ops::Deref;

use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::{SystemDesc, Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

use crate::components::{BallComponent, BoundingBoxComponent, PaddleComponent, PaddleSide};
use crate::{Sounds, ARENA_HEIGHT};

fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct BallMovementSystem;

impl<'s> System<'s> for BallMovementSystem {
    type SystemData = (
        ReadStorage<'s, BallComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BoundingBoxComponent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut transforms, mut bounds, time): Self::SystemData) {
        for (ball, transform, bounds) in (&balls, &mut transforms, &mut bounds).join() {
            // move the ball
            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());

            // update the ball bounds
            *bounds.center_mut() = *transform.translation();
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct BallCollisionSystem;

impl<'s> System<'s> for BallCollisionSystem {
    type SystemData = (
        WriteStorage<'s, BallComponent>,
        ReadStorage<'s, PaddleComponent>,
        ReadStorage<'s, BoundingBoxComponent>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, paddles, bounds, storage, sounds, audio_output): Self::SystemData,
    ) {
        // check for ball collisions
        for (ball, ball_bounds) in (&mut balls, &bounds).join() {
            let pos = ball_bounds.center();
            let half_height = ball_bounds.extents().y;

            // arena collision
            if (pos.y <= half_height && ball.velocity[1] < 0.0)
                || (pos.y >= ARENA_HEIGHT - half_height && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            // paddle collision
            for (paddle, paddle_bounds) in (&paddles, &bounds).join() {
                if !ball_bounds.intersects(&paddle_bounds) {
                    continue;
                }

                if (paddle.side == PaddleSide::Left && ball.velocity[0] < 0.0)
                    || (paddle.side == PaddleSide::Right && ball.velocity[0] > 0.0)
                {
                    ball.velocity[0] = -ball.velocity[0];
                    play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                }
            }
        }
    }
}
