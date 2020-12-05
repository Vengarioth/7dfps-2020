use bevy::math::*;

use crate::math::Ray;

#[derive(Debug)]
pub struct Bounds {
    pub position: Vec3,
    pub size: Vec3,
}

impl Bounds {
    pub fn new(position: Vec3, size: Vec3) -> Self {
        Self {
            position,
            size,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        todo!() // TODO implement AABB-ray intersection
    }
}
