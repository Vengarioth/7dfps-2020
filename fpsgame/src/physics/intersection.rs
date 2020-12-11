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

#[derive(Debug, Clone)]
pub struct PrimitiveIntersection {
    pub position: Vec3,
    pub surface_normal: Vec3,
    pub penetration_normal: Vec3,
    pub penetration_depth: f32,
}

impl PrimitiveIntersection {
    pub fn new(position: Vec3, surface_normal: Vec3, penetration_normal: Vec3, penetration_depth: f32) -> Self {
        Self {
            position,
            surface_normal,
            penetration_normal,
            penetration_depth,
        }
    }
}
