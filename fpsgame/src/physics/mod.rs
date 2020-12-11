pub mod bvh;
pub mod primitive;
mod baking;
mod world;
mod util;
mod intersection;

pub use world::*;
pub use intersection::*;

use bevy::math::*;
use gltf;

use self::primitive::Triangle;

pub fn create_bvh_from_gltf(path: &str) -> World {
    let (document, buffer, ..) = gltf::import(path).unwrap();
    let mut triangles = Vec::new();

    for scene in document.scenes() {
        for node in scene.nodes() {
            load_recursive(&node, &buffer, &mut triangles);
        }
    }

    World::new(baking::build_bvh(triangles))
}

pub fn load_recursive(node: &gltf::Node, buffers: &[gltf::buffer::Data], triangles: &mut Vec<Triangle>) {
    if let Some(mesh) = node.mesh() {
        // TODO support transform?
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let positions: Vec<Vec3> = reader.read_positions().expect("primitive must have POSITION attribute").map(|v| Vec3::new(v[0], v[1], v[2])).collect();
            let indices: Vec<u32> = reader.read_indices().expect("primitive must have indices").into_u32().collect();

            for triangle_indices in indices.chunks(3) {
                triangles.push(Triangle::new(
                    positions[triangle_indices[0] as usize],
                    positions[triangle_indices[1] as usize],
                    positions[triangle_indices[2] as usize],
                ));
            }
        }
    }

    for child in node.children() {
        load_recursive(&child, buffers, triangles);
    }
}
