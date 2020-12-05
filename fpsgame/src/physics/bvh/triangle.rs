use bevy::math::*;
use crate::math::Ray;
use super::Intersection;

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {

        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -std::f32::EPSILON && a < std::f32::EPSILON {
            return None; // the ray is parallel to this triangle
        }

        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t > std::f32::EPSILON {
            return Some(Intersection::new(t));
        }
        
        return None;
    }
}
