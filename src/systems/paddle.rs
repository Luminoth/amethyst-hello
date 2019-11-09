use amethyst::core::{SystemDesc, Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{PaddleComponent, PaddleSide};
use crate::{ARENA_HEIGHT, PADDLE_HEIGHT, PADDLE_SPEED};

#[derive(Default, SystemDesc)]
pub struct PaddleInputSystem;

impl<'s> System<'s> for PaddleInputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PaddleComponent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            // read the input
            let movement = match paddle.side {
                PaddleSide::Left => input.axis_value("left_paddle"),
                PaddleSide::Right => input.axis_value("right_paddle"),
            };

            // apply the translation
            if let Some(amount) = movement {
                let speed = amount * PADDLE_SPEED * time.delta_seconds();
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + speed)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5),
                );
            }
        }
    }
}
