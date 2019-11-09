use std::ops::Deref;

use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

use crate::components::{
    bounding_box_intersects, BallComponent, BoundingBoxComponent, PaddleComponent, PaddleSide,
    PhysicalComponent,
};
use crate::{Sounds, ARENA_HEIGHT};

fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct BallCollisionSystem;

impl<'s> System<'s> for BallCollisionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, PhysicalComponent>,
        ReadStorage<'s, BallComponent>,
        ReadStorage<'s, PaddleComponent>,
        ReadStorage<'s, BoundingBoxComponent>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (transforms, mut physicals, balls, paddles, bounds, storage, sounds, audio_output): Self::SystemData,
    ) {
        for (_ball, ball_transform, ball_physical, ball_bounds) in
            (&balls, &transforms, &mut physicals, &bounds).join()
        {
            let bounds_center = ball_transform.translation() + ball_bounds.center();
            let half_height = ball_bounds.extents().y;

            // arena collision
            if (bounds_center.y <= half_height && ball_physical.velocity.y < 0.0)
                || (bounds_center.y >= ARENA_HEIGHT - half_height && ball_physical.velocity.y > 0.0)
            {
                ball_physical.velocity.y = -ball_physical.velocity.y;
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            // paddle collision
            for (paddle, paddle_transform, paddle_bounds) in (&paddles, &transforms, &bounds).join()
            {
                if !bounding_box_intersects(
                    ball_transform,
                    ball_bounds,
                    paddle_transform,
                    paddle_bounds,
                ) {
                    continue;
                }

                if (paddle.side == PaddleSide::Left && ball_physical.velocity.x < 0.0)
                    || (paddle.side == PaddleSide::Right && ball_physical.velocity.x > 0.0)
                {
                    ball_physical.velocity.x = -ball_physical.velocity.x;
                    play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                }
            }
        }
    }
}
