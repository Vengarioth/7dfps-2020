use bevy::prelude::*;

use crate::physics::{PrimitiveIntersection, primitive::Sphere};

#[derive(Debug, Default, Clone)]
pub struct Kinematic;

#[derive(Debug, Default, Clone)]
pub struct Dynamic;

#[derive(Debug, Default, Clone)]
pub struct Acceleration(pub Vec3);

#[derive(Debug, Default, Clone)]
pub struct Velocity(pub Vec3);

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

pub fn apply_gravity(mut entities: Query<(&Gravity, &mut RigidBody)>) {
    for (gravity, mut rb) in entities.iter_mut() {
        rb.force += gravity.0;
    }
}

pub fn update_velocity(mut entities: Query<&mut RigidBody>) {
    for mut rb in entities.iter_mut() {
        let inverse_mass = 1.0 / rb.mass;
        let acceleration = rb.force * inverse_mass;
        rb.velocity += acceleration * FIXED_UPDATE;
    }
}

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
    
                let cor = 0.1;
                let j = (-(1.0 + cor) * relative_velocity.dot(relative_normal)) / (1.0 / rb.mass);
    
                let impulse = relative_normal * j * -1.0;
                crate::util::draw_primitives::draw_line_for((rb.position, rb.position + (impulse * 10.0)), 3);

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

pub fn update_rigid_bodies(mut entities: Query<&mut RigidBody>) {
    for mut rb in entities.iter_mut() {
        let velocity = rb.velocity;
        rb.position += velocity * FIXED_UPDATE;
        rb.force = Vec3::zero();
    }
}

pub fn update_rigid_body_transforms(mut entities: Query<(&RigidBody, &mut Transform)>) {
    for (rb, mut transform) in entities.iter_mut() {
        transform.translation = rb.position;
    }
}

/*
pub fn integrate_acceleration_velocity(time: Res<Time>, mut entities: Query<(&mut Movement, &mut Acceleration, &mut Velocity)>) {
    for (mut movement, mut acceleration, mut velocity) in entities.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds;
        movement.0 += velocity.0 * time.delta_seconds;
        acceleration.0 *= 0.1;
        acceleration.0 += Vec3::new(0.0, -9.81, 0.0);
    }
}

pub fn move_entities(world: Res<crate::physics::World>, mut entities: Query<(&MovementData, &mut Movement, &mut GroundedState, &mut Transform, &mut Acceleration, &mut Velocity)>) {
    for (movement_data, mut movement, grounded_state, mut transform, mut acceleration, mut velocity) in entities.iter_mut() {

        let mut length = movement.0.length();
        let mut direction = movement.0.normalize();
        movement.0 = Vec3::zero();

        while length > 0.0 {
            let mag = length.min(0.1);
            let to_move = direction * mag;

            let new_position = transform.translation + to_move;
            transform.translation = new_position;

            let query = Sphere::new(new_position, 0.2);
            if let Some(intersection) = world.collide_sphere(&query) {
                transform.translation += intersection.penetration_normal * (intersection.penetration_depth + std::f32::EPSILON);

                fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
                    vector - 2.0 * vector.dot(normal) * normal
                }

                acceleration.0 = reflect(acceleration.0, intersection.surface_normal);
                velocity.0 = reflect(velocity.0, intersection.surface_normal);
                direction = reflect(direction, intersection.surface_normal);
            }

            length -= mag;
        }
    }
}
*/

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
