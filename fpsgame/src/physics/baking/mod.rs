use crate::physics::bvh::*;
use crate::physics::Triangle;

/// Builds a BVH from a static set of triangles
pub fn build_bvh(triangles: Vec<Triangle>) -> Bvh {

    let bounds: Vec<Bounds> = triangles.iter().map(|triangle| triangle.get_bounds()).collect();
    let indices: Vec<usize> = triangles.iter().enumerate().map(|(i, _)| i).collect();
    let mut nodes = Vec::new();

    let (root, _) = build_recursive(&triangles, &bounds, &indices, &mut nodes);

    Bvh::from_prebuilt(nodes, Some(root), triangles)
}

fn build_recursive(triangles: &[Triangle], triangle_bounds: &[Bounds], primitives: &[usize], nodes: &mut Vec<BvhNode>) -> (usize, Bounds) {
    match primitives.len() {
        0 => panic!("No primitives were provided"),
        1 => {
            // construct a leaf node
            let index = nodes.len();
            let primitive = primitives[0];
            let bounds = triangle_bounds[primitive].clone();
            nodes.push(BvhNode::Leaf {
                bounds: bounds.clone(),
                primitive,
            });

            (index, bounds)
        },
        2 => {
            // make simple split

            let (left, left_bounds) = build_recursive(triangles, triangle_bounds, &[primitives[0]], nodes);
            let (right, right_bounds) = build_recursive(triangles, triangle_bounds, &[primitives[1]], nodes);
            let bounds = left_bounds.join(&right_bounds);

            let index = nodes.len();
            nodes.push(BvhNode::Branch {
                left,
                right,
                bounds: bounds.clone()
            });

            (index, bounds)
        }
        _ => {
            // split primitives by largest axis

            // computes the total bounds of all primitives
            let node_bounds = primitives.iter()
                .fold(triangle_bounds[primitives[0]].clone(), |a, b| a.join(&triangle_bounds[*b]));

            let mut left = Vec::new();
            let mut right = Vec::new();
            match node_bounds.largest_direction() {
                Axis::X => {
                    let half = node_bounds.centroid().x();
                    for primitive in primitives {
                        if triangle_bounds[*primitive].centroid().x() < half {
                            left.push(*primitive);
                        } else {
                            right.push(*primitive);
                        }
                    }
                },
                Axis::Y => {
                    let half = node_bounds.centroid().y();
                    for primitive in primitives {
                        if triangle_bounds[*primitive].centroid().y() < half {
                            left.push(*primitive);
                        } else {
                            right.push(*primitive);
                        }
                    }
                },
                Axis::Z => {
                    let half = node_bounds.centroid().z();
                    for primitive in primitives {
                        if triangle_bounds[*primitive].centroid().z() < half {
                            left.push(*primitive);
                        } else {
                            right.push(*primitive);
                        }
                    }
                },
            }

            // TODO sometimes this sorting fails on very small triangles with similar centroids
            if left.len() == 0 {
                while right.len() >= left.len() {
                    left.push(right.remove(0));
                }
            }
            if right.len() == 0 {
                while left.len() >= right.len() {
                    right.push(left.remove(0));
                }
            }

            let (left, _) = build_recursive(triangles, triangle_bounds, &left, nodes);
            let (right, _) = build_recursive(triangles, triangle_bounds, &right, nodes);

            let index = nodes.len();
            nodes.push(BvhNode::Branch {
                left,
                right,
                bounds: node_bounds.clone()
            });

            (index, node_bounds)
        },
    }
}
