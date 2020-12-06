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
        #[inline(always)]
        fn min_max(min: f32, max: f32, origin: f32, direction: f32) -> (f32, f32) {
            let t_min = (min - origin) / direction;
            let t_max = (max - origin) / direction;

            if t_min > t_max {
                (t_max, t_min)
            } else {
                (t_min, t_max)
            }
        }

        let (tmin, tmax) = min_max(self.min.x(), self.max.x(), ray.origin.x(), ray.direction.x());
        let (tymin, tymax) = min_max(self.min.y(), self.max.y(), ray.origin.y(), ray.direction.y());

        if tmin > tymax || tymin > tmax {
            return false;
        }

        let tmin = tmin.max(tymin);
        let tmax = tmax.min(tymax);

        let (tzmin, tzmax) = min_max(self.min.z(), self.max.z(), ray.origin.z(), ray.direction.z());

        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        return true;
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
