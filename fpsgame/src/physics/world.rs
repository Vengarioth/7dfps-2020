use super::bvh::{Bvh, Intersection};
use crate::math::Ray;

pub struct World {
    bvh: Bvh,
}

impl World {
    pub fn new(bvh: Bvh) -> Self {
        Self {
            bvh,
        }
    }

    pub fn raycast(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersections = self.bvh.intersects(ray);

        if intersections.len() < 1 {
            return None;
        }

        if intersections.len() == 1 {
            return Some(intersections.remove(0));
        }

        let (index, _) = intersections.iter().enumerate().fold((0, std::f32::INFINITY), |(min_index, min), (index, intersection)| {
            if intersection.t < min {
                (index, intersection.t)
            } else {
                (min_index, min)
            }
        });

        return Some(intersections.remove(index));
    }
}
