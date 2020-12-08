use bevy::math::*;

use crate::math::Ray;

use super::bvh::{Bounds, Intersection, Triangle};

#[derive(Debug)]
pub struct Capsule {
    base: Vec3,
    top: Vec3,
    radius: f32,
}

impl Capsule {
    pub fn new(base: Vec3, top: Vec3, radius: f32) -> Self {
        Self {
            base,
            top,
            radius,
        }
    }

    pub fn get_bounds(&self) -> Bounds {
        let min = self.base.min(self.top) + (Vec3::one() * self.radius);
        let max = self.base.max(self.top) + (Vec3::one() * self.radius);
        Bounds::new(min, max)
    }

    pub fn intersects_triangle(&self, triangle: &Triangle) -> Option<Intersection> {
        todo!();
    }

    pub fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!();
    }
}
