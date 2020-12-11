mod axis;
mod bounds;

pub use axis::*;
pub use bounds::*;

use crate::math::Ray;
use super::{Intersection, Triangle};

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

    // TODO move primitives/triangles into world
    pub fn get_primitive(&self, index: usize) -> &Triangle {
        &self.triangles[index]
    }

    pub fn query_bounds(&self, query: &Bounds) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut primitives = Vec::new();

        if let Some(root) = self.root {
            stack.push(root);
        }

        while let Some(index) = stack.pop() {
            match &self.nodes[index] {
                BvhNode::Branch { bounds, left, right } => {
                    if bounds.overlaps(query) {
                        stack.push(*left);
                        stack.push(*right);
                    }
                },
                BvhNode::Leaf { bounds, primitive } => {
                    if bounds.overlaps(query) {
                        primitives.push(*primitive)
                    }
                },
            }
        }

        primitives
    }

    pub fn query_bounds_iter<'a>(&'a self, query: Bounds) -> BvhIterator<'a> {
        let mut stack = Vec::new();
        if let Some(root) = self.root {
            stack.push(root);
        }
        BvhIterator::new(self, query, stack)
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

pub struct BvhIterator<'a> {
    bvh: &'a Bvh,
    query: Bounds,
    stack: Vec<usize>,
}

impl<'a> BvhIterator<'a> {
    pub fn new(bvh: &'a Bvh, query: Bounds, stack: Vec<usize>) -> Self {
        Self {
            bvh,
            query,
            stack,
        }
    }

    pub fn get_triangle(&'a self, index: usize) -> &'a Triangle {
        &self.bvh.triangles[index]
    }
}

impl<'a> Iterator for BvhIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.stack.pop() {
            match &self.bvh.nodes[index] {
                BvhNode::Branch { bounds, left, right } => {
                    if bounds.overlaps(&self.query) {
                        self.stack.push(*left);
                        self.stack.push(*right);
                    }
                },
                BvhNode::Leaf { bounds, primitive } => {
                    if bounds.overlaps(&self.query) {
                        return Some(*primitive);
                    }
                },
            }
        }

        None
    }
}
