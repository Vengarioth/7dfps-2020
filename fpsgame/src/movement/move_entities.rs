use bevy::prelude::*;

use crate::physics::bvh::Bounds;

#[derive(Bundle)]
pub struct MovementComponents {
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub movement_data: MovementData,
    pub grounded_state: GroundedState,
}

#[derive(Debug, Default, Clone)]
pub struct Acceleration(pub Vec3);


#[derive(Debug, Default, Clone)]
pub struct Velocity(pub Vec3);

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

pub fn move_entities(world: Res<crate::physics::World>, mut entities: Query<(&MovementData, &mut GroundedState, &mut Acceleration, &mut Velocity, &mut Transform)>) {
    for (movement_data, grounded_state, acceleration, velocity, transform) in entities.iter_mut() {

        let position = transform.translation;
        let min = position + Vec3::new(-movement_data.radius, 0.0, -movement_data.radius);
        let max = position + Vec3::new(movement_data.radius, movement_data.height, movement_data.radius);

        let current_bounds = Bounds::new(min, max);
    }
}
