use bevy::prelude::*;

use crate::physics::{PrimitiveIntersection, primitive::Sphere};

#[derive(Debug, Default, Clone)]
pub struct Kinematic;

#[derive(Debug, Default, Clone)]
pub struct Movement(pub Vec3);

#[derive(Debug, Default, Clone)]
pub struct MovementData {
    pub height: f32,
    pub radius: f32,
    pub raycast_offset: f32,
}

#[derive(Debug, Default, Clone)]
pub struct GroundedState {
    pub is_grounded: bool,
    pub was_grounded: bool,
    pub is_on_slope: bool,
    pub was_on_slope: bool,
    pub frames_since_grounded: u32,
}

#[derive(Debug, Clone)]
pub struct Gravity(pub Vec3);

#[derive(Debug)]
pub struct RigidBody {
    pub mass: f32,
    pub cor: f32,
    pub force: Vec3,
    pub velocity: Vec3,
    pub position: Vec3,
}

#[derive(Debug)]
pub struct Collider {
    pub sphere: Sphere,
}

const FIXED_UPDATE: f32 = 0.016;
const ITERATIONS: usize = 4;

// --- All force modifies
// --- All systems that modify force must run before velocity update ---

pub fn apply_gravity(mut entities: Query<(&Gravity, &mut RigidBody)>) {
    for (gravity, mut rb) in entities.iter_mut() {
        rb.force += gravity.0;
    }
}

// --- velocity update must happen before conllisions are resolved ---

pub fn update_velocity(mut entities: Query<&mut RigidBody>) {
    for mut rb in entities.iter_mut() {
        let inverse_mass = 1.0 / rb.mass;
        let acceleration = rb.force * inverse_mass;
        rb.velocity += acceleration * FIXED_UPDATE;
    }
}

// --- All velocity constraints
// --- collisions must be resolved before the rigid body update ---

pub fn resolve_collisions(world: Res<crate::physics::World>, mut entities: Query<(&Collider, &mut RigidBody)>) {
    for (collider, mut rb) in entities.iter_mut() {
        let query = Sphere::new(rb.position + collider.sphere.center, collider.sphere.radius);
        let intersections: Vec<PrimitiveIntersection> = world.collide_sphere_all(&query).collect();

        for _ in 0..ITERATIONS {
            for intersection in &intersections {
                let relative_velocity = -rb.velocity;
                let relative_normal = -intersection.surface_normal;

                if relative_velocity.dot(relative_normal) > 0.0 {
                    continue;
                }
    
                let j = (-(1.0 + rb.cor) * relative_velocity.dot(relative_normal)) / (1.0 / rb.mass);
    
                let impulse = relative_normal * j * -1.0;

                rb.velocity += impulse;
            }
        }

        for intersection in &intersections {
            let relative_velocity = -rb.velocity;
            let relative_normal = -intersection.surface_normal;
            if relative_velocity.dot(relative_normal) < 0.0 {
                dbg!("omg!");
            }
        }
    }
}

// --- Rigid body update must happen before the transform is modified ---
pub fn update_rigid_bodies(mut entities: Query<&mut RigidBody>) {
    for mut rb in entities.iter_mut() {
        let velocity = rb.velocity;
        rb.position += velocity * FIXED_UPDATE;
        rb.force = Vec3::zero();
    }
}

// --- Transform is modified after all physics systems ---
pub fn update_rigid_body_transforms(mut entities: Query<(&RigidBody, &mut Transform)>) {
    for (rb, mut transform) in entities.iter_mut() {
        transform.translation = rb.position;
    }
}

pub fn move_kinematic_entities(world: Res<crate::physics::World>, mut entities: Query<(&Kinematic, &MovementData, &mut Movement, &mut GroundedState, &mut Transform)>) {
    for (_, movement_data, mut movement, grounded_state, mut transform) in entities.iter_mut() {
        move_entity(movement.0, &world, &mut transform);
        movement.0 = Vec3::zero();
    }
}

fn move_entity(movement: Vec3, world: &crate::physics::World, transform: &mut Transform) {
    let mut distance_to_move = movement.length();

    while distance_to_move > std::f32::EPSILON {
        let to_move = distance_to_move.min(0.25); // move at maximum 0.25 at once to avoid tunneling

        transform.translation += movement.normalize() * to_move;
        distance_to_move -= to_move;

        for _ in 0..4 {
            if let Some(intersection) = world.collide_sphere(&Sphere::new(transform.translation, 1.0)) {
                transform.translation += intersection.penetration_normal * (intersection.penetration_depth + std::f32::EPSILON);
            }
        }

        for intersection in world.collide_sphere_all(&Sphere::new(transform.translation, 1.0)) {
            crate::util::draw_primitives::draw_line_for((intersection.position, intersection.position + (intersection.penetration_normal * intersection.penetration_depth)), 1);
        }
    }
}
