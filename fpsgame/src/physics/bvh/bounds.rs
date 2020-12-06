use bevy::math::*;

use crate::math::Ray;
use super::Axis;

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn centroid(&self) -> Vec3 {
        self.min + ((self.max - self.min) * 0.5)
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        todo!() // TODO implement AABB-ray intersection
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn largest_direction(&self) -> Axis {
        let extents = self.max - self.min;

        if extents.x() > extents.y() {
            if extents.x() > extents.z() {
                Axis::X
            } else {
                Axis::Z
            }
        } else {
            if extents.y() > extents.z() {
                Axis::Y
            } else {
                Axis::Z
            }
        }
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d.x() * d.y() + d.y() * d.z() + d.z() * d.x())
    }
}
