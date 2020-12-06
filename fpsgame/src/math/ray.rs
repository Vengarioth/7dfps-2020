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

    // returns the point with distance `t` from the ray's origin along the ray's direction
    pub fn get_point(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }
}
