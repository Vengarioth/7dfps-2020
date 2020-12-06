mod axis;
mod bounds;
mod triangle;
mod intersection;

pub use axis::*;
pub use bounds::*;
pub use triangle::*;
pub use intersection::*;

use crate::math::Ray;

#[derive(Debug)]
pub enum BvhNode {
    Branch {
        bounds: Bounds,
        left: usize,
        right: usize,
    },
    Leaf {
        bounds: Bounds,
        primitive: usize,
    },
}

#[derive(Debug)]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    root: Option<usize>,
    triangles: Vec<Triangle>,
}

impl Bvh {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            triangles: Vec::new(),
        }
    }

    pub fn from_prebuilt(nodes: Vec<BvhNode>, root: Option<usize>, triangles: Vec<Triangle>) -> Self {
        Self {
            nodes,
            root,
            triangles,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        if let Some(root) = self.root {
            self.intersect_recursive(root, ray, &mut intersections);
        }
        intersections
    }

    pub fn calculate_cost(&self) -> f32 {
        let mut stack = Vec::new();
        if let Some(root) = self.root {
            stack.push(root);
        }

        let mut cost = 0.0;
        while let Some(index) = stack.pop() {
            match &self.nodes[index] {
                BvhNode::Branch { bounds, left, right} => {
                    cost += bounds.surface_area();
                    stack.push(*left);
                    stack.push(*right);
                },
                BvhNode::Leaf { .. } => { /* do nothing */ }
            }
        }

        cost
    }

    pub fn intersect_recursive(&self, index: usize, ray: &Ray, intersections: &mut Vec<Intersection>) {
        match &self.nodes[index] {
            BvhNode::Branch { bounds, left, right } => {
                if bounds.intersects(ray) {
                    self.intersect_recursive(*left, ray, intersections);
                    self.intersect_recursive(*right, ray, intersections);
                }
            },
            BvhNode::Leaf { bounds, primitive } => {
                if bounds.intersects(ray) {
                    if let Some(intersection) = self.triangles[*primitive].intersects(ray) {
                        intersections.push(intersection);
                    }
                }
            },
        }
    }
}
