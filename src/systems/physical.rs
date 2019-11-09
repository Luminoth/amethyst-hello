use amethyst::core::math::Vector3;
use amethyst::core::{SystemDesc, Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

use crate::components::PhysicalComponent;

#[derive(Default, SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, PhysicalComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut physicals, mut transforms, time): Self::SystemData) {
        for (physical, transform) in (&mut physicals, &mut transforms).join() {
            transform.prepend_translation(Vector3::new(
                physical.velocity.x * time.delta_seconds(),
                physical.velocity.y * time.delta_seconds(),
                physical.velocity.z * time.delta_seconds(),
            ));

            physical.velocity += Vector3::new(
                physical.acceleration.x * time.delta_seconds(),
                physical.acceleration.y * time.delta_seconds(),
                physical.acceleration.z * time.delta_seconds(),
            );
        }
    }
}
