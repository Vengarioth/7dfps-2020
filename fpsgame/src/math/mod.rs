// TASK: impl add, sub, div, mul etc for the structs

mod ray;
pub use ray::*;

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
