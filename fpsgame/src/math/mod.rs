use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec3 {
    fn unit_x() -> Self {
        Self { x: 1, y: 0, z: 0 }
    }

    fn unit_y() -> Self {
        Self { x: 0, y: 1, z: 0 }
    }

    fn unit_z() -> Self {
        Self { x: 0, y: 0, z: 1 }
    }

    fn splat(num: i32) -> Self {
        Self { x: num, y: num, z: num }
    }

    fn cross(self, other: Self) -> Self {
        Self { 
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
         }
    }

    fn truncate(self) -> IVec2 {
        IVec2 { x: self.x, y: self.y }
    }
}

impl IVec2 {
    fn unit_x() -> Self {
        Self { x: 1, y: 0 }
    }

    fn unit_y() -> Self {
        Self { x: 0, y: 1 }
    }

    fn splat(num: i32) -> Self {
        Self { x: num, y: num }
    }

    fn perp_dot(self, other: Self) -> i32 {
        (self.x * other.y) - (self.y * other.x)
    }

    fn extend(self, z: i32) -> IVec3 {
        IVec3 { x: self.x, y: self.y, z }
    }

}

impl Add for IVec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for IVec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<i32> for IVec3 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Div<i32> for IVec3 {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {x: self.x * other, y: self.y * other}
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {x: self.x / other, y: self.y / other}
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (degrees / 180f32) * std::f32::consts::PI
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    (radians / std::f32::consts::PI) * 180f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetics() {
        let vec2_input1 = IVec2 {
            x: 2,
            y: 2,
        };
        let vec2_input2 = IVec2 {
            x: 2,
            y: 2,
        };
        let vec3_input1 = IVec3 {
            x: 2,
            y: 2,
            z: 2,
        };
        let vec3_input2 = IVec3 {
            x: 2,
            y: 2,
            z: 2,
        };
        assert_eq!(vec2_input1+vec2_input2, IVec2 { x: 4, y: 4 });
        assert_eq!(vec2_input1-vec2_input2, IVec2 { x: 0, y: 0 });
        assert_eq!(vec2_input1*2, IVec2 { x: 4, y: 4 });
        assert_eq!(vec2_input1/2, IVec2 { x: 1, y: 1 });

        assert_eq!(vec3_input1+vec3_input2, IVec3 { x: 4, y: 4, z: 4 });
        assert_eq!(vec3_input1-vec3_input2, IVec3 { x: 0, y: 0, z: 0 });
        assert_eq!(vec3_input1*2, IVec3 { x: 4, y: 4, z: 4 });
        assert_eq!(vec3_input1/2, IVec3 { x: 1, y: 1, z: 1 });
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero_should_panic() {
        let _exp = IVec2 { x: 1, y: 1 }/0;
        let _exp = IVec3 { x: 1, y: 1, z: 1 }/0;
    }

    #[test]
    fn test_degrees_to_radians() {
        assert_eq!(degrees_to_radians(90f32), std::f32::consts::PI/2f32);
        assert_eq!(degrees_to_radians(180f32), std::f32::consts::PI);
        assert_eq!(degrees_to_radians(360f32), std::f32::consts::PI*2f32);
    }
    
    #[test]
    fn test_radians_to_degrees() {
        assert_eq!(radians_to_degrees(std::f32::consts::PI/2f32), 90f32);
        assert_eq!(radians_to_degrees(std::f32::consts::PI), 180f32);
        assert_eq!(radians_to_degrees(std::f32::consts::PI*2f32), 360f32);
    }
}