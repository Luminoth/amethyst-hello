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

    amin.x <= bmax.x
        && amax.x >= bmin.x
        && amin.y <= bmax.y
        && amax.y >= bmin.y
        && amin.z <= bmax.z
        && amax.z >= bmin.z
}

#[allow(dead_code)]
pub fn bounding_box_contains(
    transform: &Transform,
    bounds: &BoundingBoxComponent,
    point: &Vector3<f32>,
) -> bool {
    let min = transform.translation() + bounds.min();
    let max = transform.translation() + bounds.max();

    min.x <= point.x
        && max.x >= point.x
        && min.y <= point.y
        && max.y >= point.y
        && min.z <= point.z
        && max.z >= point.z
}

impl Default for BoundingBoxComponent {
    fn default() -> Self {
        Self::new(Vector3::from_element(0.0), Vector3::new(1.0, 1.0, 1.0))
    }
}

impl BoundingBoxComponent {
    pub fn new(center: Vector3<f32>, size: Vector3<f32>) -> Self {
        assert!(size.x >= 0.0);
        assert!(size.y >= 0.0);
        assert!(size.z >= 0.0);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        let a = Transform::default();
        let abb = BoundingBoxComponent::default();

        let b = Transform::default();
        let bbb = BoundingBoxComponent::default();

        assert_eq!(bounding_box_intersects(&a, &abb, &b, &bbb), true);
    }
}
