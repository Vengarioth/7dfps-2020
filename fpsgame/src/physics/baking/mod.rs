use crate::physics::bvh::*;

pub fn bake_bvh(triangles: Vec<Triangle>) -> Bvh {
    let mut bvh = Bvh::new();

    for triangle in triangles {
        bvh.insert(triangle);
    }

    bvh
}
