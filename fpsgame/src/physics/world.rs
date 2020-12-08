use super::{PrimitiveIntersection, Sphere, bvh::{Bvh, Intersection}};
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

    pub fn collide_sphere(&self, sphere: &Sphere) -> Option<PrimitiveIntersection> {
        let bounds = sphere.get_bounds();
        let mut max_penetration = std::f32::NEG_INFINITY;
        let mut best_intersection = None;
        for index in self.bvh.query_bounds(&bounds) {
            if let Some(intersection) = sphere.intersects_triangle(self.bvh.get_primitive(index)) {
                if intersection.penetration_depth > max_penetration {
                    max_penetration = intersection.penetration_depth;
                    best_intersection = Some(intersection);
                }
            }
        }

        best_intersection
    }
}
