use bevy::math::*;

pub fn closest_point_on_line_segment(a: Vec3, b: Vec3, point: Vec3) -> Vec3 {
    let ab = b - a;
    let t = (point - a).dot(ab) / ab.dot(ab);
    a + t.min(1.0).max(0.0) * ab
}
