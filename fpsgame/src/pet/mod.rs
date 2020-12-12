pub mod stats;
pub mod mood;
use stats::{
    *, PetStats
};
use mood::*;
use bevy::prelude::*;
use crate::enemy::Enemy;
use crate::movement::{
    Movement,
    MovementData,
    Kinematic,
    GroundedState,
};

pub struct Pet{
    stats: PetStats,
    // mood: PetMood,
    visible_enemies: Vec<Enemy>,
    attack_time: f32,
}

pub fn shoot_enemies (
    mut commands: Commands,
    time: Res<Time>,
    enemy_query: Query<(&Enemy, &Transform)>,
    mut pet_query: Query<(&mut Pet, &Transform)>,
){
    //use the list of enemies to find the nearest one and shoot them
    for (mut pet, pet_transform) in pet_query.iter_mut()
    {
        if pet.attack_time <= 0.0 {
            let mut closest_enemy = 0.0;
            for (enemy, enemy_transform) in enemy_query.iter() {
                let distance = enemy_transform.translation - pet_transform.translation;
                    // if distance.magnitude < pet.stats.attack_range
                    // {
                    //     //shoot projectile
                        
                    //     //play throw sound
                    // }
            }
        }
        else 
        {
            pet.attack_time -= time.delta_seconds;
        }
    }
}

pub fn move_pet (
    mut commands: Commands,
    time: Res<Time>,
    mut pet_query: Query<(&mut Pet, &Transform)>,
){

}


impl Pet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn spawn_pet(mut commands: Commands, position: Vec3) {
        commands.spawn((
            Pet::new(),
            Transform::from_translation(position),
            Movement(Vec3::zero()),
            MovementData::default(), // TODO
            GroundedState::default(),
            Kinematic, //Are pets kinematic??
        ));
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            stats: PetStats::new(),
            // mood: PetMood::new(),
            visible_enemies: Vec::new(),
            attack_time: 0.0,
        }
    }
}
