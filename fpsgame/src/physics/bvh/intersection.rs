use bevy::math::*;

#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

impl Intersection {
    pub fn new(t: f32, position: Vec3, normal: Vec3) -> Self {
        Self {
            t,
            position,
            normal,
        }
    }
}
