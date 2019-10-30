use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Component;

#[derive(Component, Clone)]
pub struct BoundingBoxComponent {
    center: Vector3<f32>,
    extents: Vector3<f32>,
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

    pub fn center_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.center
    }

    pub fn extents(&self) -> &Vector3<f32> {
        &self.extents
    }

    pub fn size(&self) -> Vector3<f32> {
        self.extents * 2.0
    }

    pub fn min(&self) -> Vector3<f32> {
        self.center - self.extents
    }

    pub fn max(&self) -> Vector3<f32> {
        self.center + self.extents
    }

    pub fn contains(&self, point: &Vector3<f32>) -> bool {
        let min = self.min();
        let max = self.max();

        min.x < point.x
            && max.x > point.x
            && min.y < point.y
            && max.y > point.y
            && min.z < point.z
            && max.z > point.z
    }

    pub fn intersects(&self, other: &BoundingBoxComponent) -> bool {
        let min = self.min();
        let max = self.max();
        let omin = other.min();
        let omax = other.max();

        min.x < omax.x
            && max.x > omin.x
            && min.y < omax.y
            && max.y > omin.y
            && min.z < omax.z
            && max.z > omin.z
    }
}
