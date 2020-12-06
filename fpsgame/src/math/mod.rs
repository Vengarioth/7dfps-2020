// TASK: impl add, sub, div, mul etc for the structs
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
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

impl Mul for IVec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Div for IVec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
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

impl Mul for IVec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Div for IVec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {x: self.x / other.x, y: self.y / other.y}
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (degrees / 180f32) * std::f32::consts::PI
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    (radians / std::f32::consts::PI) * 180f32
}