use bevy::math::*;

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
