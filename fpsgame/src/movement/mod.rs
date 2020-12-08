use bevy::prelude::*;

mod move_entities;

pub use move_entities::*;

#[derive(Bundle, Default)]
pub struct MovementComponents {
    pub movement: Movement,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub movement_data: MovementData,
    pub grounded_state: GroundedState,
}
