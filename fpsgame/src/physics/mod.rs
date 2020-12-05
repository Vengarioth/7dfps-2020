use crate::math::IVec3;

// TASK impl convenience functions for AABB

#[derive(Debug)]
pub struct AABB {
    position: IVec3,
    size: IVec3,
}

impl AABB {
    pub fn new(position: IVec3, size: IVec3) -> Self {
        Self {
            position,
            size,
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(other.position.x + other.size.x <= self.position.x ||
        other.position.x >= self.position.x + self.size.x ||
        other.position.y + other.size.y <= self.position.y ||
        other.position.y >= self.position.y + self.size.y ||
        other.position.z + other.size.z <= self.position.z ||
        other.position.z >= self.position.z + self.size.z)
    }
}
