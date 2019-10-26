use amethyst::core::{SystemDesc, Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

use crate::components::{BallComponent, PaddleComponent, PaddleSide};
use crate::utils::point_in_rect;
use crate::ARENA_HEIGHT;

#[derive(SystemDesc)]
pub struct BallMovementSystem;

impl<'s> System<'s> for BallMovementSystem {
    type SystemData = (
        ReadStorage<'s, BallComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        // move the balls
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}

#[derive(SystemDesc)]
pub struct BallCollisionSystem;

impl<'s> System<'s> for BallCollisionSystem {
    type SystemData = (
        WriteStorage<'s, BallComponent>,
        ReadStorage<'s, PaddleComponent>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        // check for ball collisions
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // arena collision
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            }

            // paddle collision
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                // bounding box collision
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == PaddleSide::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == PaddleSide::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}
