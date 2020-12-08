use bevy::prelude::*;

use crate::physics::Sphere;

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

pub fn integrate_acceleration_velocity(time: Res<Time>, mut entities: Query<(&mut Movement, &mut Acceleration, &mut Velocity)>) {
    for (mut movement, mut acceleration, mut velocity) in entities.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds;
        movement.0 += velocity.0 * time.delta_seconds;
        acceleration.0 *= 0.9;
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

                let p = intersection.position;
                let n = intersection.surface_normal;
                crate::util::draw_primitives::draw_line_for((p, p + n), 60);

                transform.translation += intersection.surface_normal * (intersection.penetration_depth);

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
