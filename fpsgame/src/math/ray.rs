use bevy::math::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub length: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, length: f32) -> Self {
        Self {
            origin,
            direction,
            length,
        }
    }
}
