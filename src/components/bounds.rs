use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

#[derive(Component, Clone)]
pub struct BoundingBoxComponent {
    center: Vector3<f32>,
    extents: Vector3<f32>,
}

pub fn bounding_box_intersects(
    at: &Transform,
    abb: &BoundingBoxComponent,
    bt: &Transform,
    bbb: &BoundingBoxComponent,
) -> bool {
    let amin = at.translation() + abb.min();
    let amax = at.translation() + abb.max();
    let bmin = bt.translation() + bbb.min();
    let bmax = bt.translation() + bbb.max();

    amin.x < bmax.x
        && amax.x > bmin.x
        && amin.y < bmax.y
        && amax.y > bmin.y
        && amin.z < bmax.z
        && amax.z > bmin.z
}

#[allow(dead_code)]
pub fn bounding_box_contains(
    transform: &Transform,
    bounds: &BoundingBoxComponent,
    point: &Vector3<f32>,
) -> bool {
    let min = transform.translation() + bounds.min();
    let max = transform.translation() + bounds.max();

    min.x < point.x
        && max.x > point.x
        && min.y < point.y
        && max.y > point.y
        && min.z < point.z
        && max.z > point.z
}

impl BoundingBoxComponent {
    pub fn new(center: Vector3<f32>, size: Vector3<f32>) -> Self {
        Self {
            center,
            extents: size * 0.5,
        }
    }

    pub fn center(&self) -> &Vector3<f32> {
        &self.center
    }

    pub fn extents(&self) -> &Vector3<f32> {
        &self.extents
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vector3<f32> {
        self.extents * 2.0
    }

    pub fn min(&self) -> Vector3<f32> {
        self.center - self.extents
    }

    pub fn max(&self) -> Vector3<f32> {
        self.center + self.extents
    }
}
