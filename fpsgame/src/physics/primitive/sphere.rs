use bevy::math::*;

use crate::physics::{PrimitiveIntersection, bvh::{Bounds, HasBounds}, util::closest_point_on_line_segment};
use super::Triangle;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }

    pub fn get_bounds(&self) -> Bounds {
        let radius = Vec3::splat(self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Bounds::new(min, max)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.center - other.center).length() < (self.radius + other.radius)
    }

    pub fn intersects_triangle(&self, other: &Triangle) -> Option<PrimitiveIntersection> {
        let n = other.get_normal();
        let distance = (self.center - other.a).dot(n);

        if distance < -self.radius || distance > self.radius {
            return None;
        }

        let point0 = self.center - (n * distance);

        let c0 = (point0 - other.a).cross(other.b - other.a);
        let c1 = (point0 - other.b).cross(other.c - other.b);
        let c2 = (point0 - other.c).cross(other.a - other.c);
        let inside = c0.dot(n) <= 0.0 && c1.dot(n) <= 0.0 && c2.dot(n) <= 0.0;

        let radius_squared = self.radius * self.radius;

        let point1 = closest_point_on_line_segment(other.a, other.b, self.center);
        let v1 = self.center - point1;
        let dist_squared1 = v1.dot(v1);
        let intersects1 = dist_squared1 < radius_squared;

        let point2 = closest_point_on_line_segment(other.b, other.c, self.center);
        let v2 = self.center - point2;
        let dist_squared2 = v2.dot(v2);
        let intersects2 = dist_squared2 < radius_squared;

        let point3 = closest_point_on_line_segment(other.c, other.a, self.center);
        let v3 = self.center - point3;
        let dist_squared3 = v3.dot(v3);
        let intersects3 = dist_squared3 < radius_squared;

        let intersects = intersects1 || intersects2 || intersects3;

        if inside || intersects {
            let mut best_point = point0;
            let mut intersection_vec = Vec3::zero();

            if inside {
                intersection_vec = self.center - point0;
            } else {
                let d = self.center - point1;
                let mut best_distance_squared = d.dot(d);
                best_point = point1;
                intersection_vec = d;

                let d = self.center - point2;
                let distance_squared = d.dot(d);

                if distance_squared < best_distance_squared {
                    best_distance_squared = distance_squared;
                    best_point = point2;
                    intersection_vec = d;
                }

                let d = self.center - point3;
                let distance_squared = d.dot(d);

                if distance_squared < best_distance_squared {
                    best_distance_squared = distance_squared;
                    best_point = point3;
                    intersection_vec = d;
                }
            }

            let len = intersection_vec.length();
            let penetration_normal = intersection_vec / len;
            let penetration_depth = self.radius - len;

            return Some(PrimitiveIntersection::new(best_point, n, penetration_normal, penetration_depth));
        }

        return None;
    }
}

impl HasBounds for Sphere {
    fn get_bounds(&self) -> Bounds {
        self.get_bounds()
    }
}
