#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
}

impl Intersection {
    pub fn new(t: f32) -> Self {
        Self {
            t,
        }
    }
}
