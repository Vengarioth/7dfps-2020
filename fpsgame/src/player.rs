use bevy::prelude::*;
use noise::*;

use crate::math::{*, Clamp};

const TRAUMA_MIN: f32 = 0.0;
const TRAUMA_MAX: f32 = 1.0;
const TRAUMA_POWER: f32 = 2.0;
const TRAUMA_DECAY: f32 = 2.0;
const PERLIN_SAMPLE_SIZE: f64 = 10.0;
const MAX_YAW_IN_RAD: f32 = 0.2; // maximum amount of yaw rotation when shaking 
const MAX_PITCH_IN_RAD: f32 = 0.1; // maximum amount of pitch rotation when shaking 
const MAX_ROLL_IN_RAD: f32 = 0.1; // maximum amount of roll rotation when shaking 

#[derive(Debug)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,

    /// Total height of the player
    pub height: f32,

    /// Height offset at which the camera is placed
    pub camera_height: f32,

    /// Height offset from which raycasts to the ground are made
    pub raycast_offset: f32,

    /// True if the player is standing on solid ground
    pub grounded: bool,

    // trauma the player experienced spanning from [TRAUMA_MIN - TRAUMA_MAX]
    pub trauma: f32, 
    pub trauma_yaw: f32,
    pub trauma_pitch: f32,
    pub trauma_roll: f32,
}

impl Player {
    pub fn new(yaw: f32, pitch: f32) -> Self { // <- Self is `Player` because we are in `impl Player`
        Self { // <- implicit return because no semicolon
            yaw, // <- this is the short form of `yaw: yaw,`
            pitch,

            height: 1.6,
            camera_height: 1.5,
            raycast_offset: 1.0,

            grounded: false,

            trauma: 0.0,
            trauma_yaw: 0.0,
            trauma_pitch: 0.0,
            trauma_roll: 0.0,
        }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
    }

    pub fn add_trauma(&mut self, amount: f32) {
        if amount > TRAUMA_MAX { println!("Trauma amount added was greater than {} and was limited to {}.", TRAUMA_MAX, TRAUMA_MAX) };
        self.trauma += min(TRAUMA_MAX, self.trauma+amount);
        self.trauma = self.trauma.clamp_value(0.0, 1.0);
    }

    pub fn shake_camera(&mut self, secs_since_startup: f64) {
        let shake = self.trauma.powf(TRAUMA_POWER); // the amount of shake depending on the amount of trauma 
        let perlin = Perlin::new();
        let perlin_noise_yaw = perlin.get([1.0, secs_since_startup*PERLIN_SAMPLE_SIZE]) as f32;
        let perlin_noise_pitch = perlin.get([2.0, secs_since_startup*PERLIN_SAMPLE_SIZE]) as f32;
        let perlin_noise_roll = perlin.get([3.0, secs_since_startup*PERLIN_SAMPLE_SIZE]) as f32;
        self.trauma_yaw = MAX_YAW_IN_RAD * shake * perlin_noise_yaw;
        self.trauma_pitch = MAX_PITCH_IN_RAD * shake * perlin_noise_pitch;
        self.trauma_roll = MAX_ROLL_IN_RAD * shake * perlin_noise_roll;
        self.trauma_yaw = self.trauma_yaw.clamp_value(-MAX_YAW_IN_RAD, MAX_YAW_IN_RAD);
        self.trauma_pitch = self.trauma_pitch.clamp_value(-MAX_PITCH_IN_RAD, MAX_PITCH_IN_RAD);

        println!("seconds {}, perlin {}, trauma: {}, degrees: {}", secs_since_startup, perlin_noise_yaw, self.trauma, radians_to_degrees(self.trauma_yaw));
    }
}

pub fn update_trauma(
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        if player.trauma > 0.0 {
            player.trauma = max(TRAUMA_MIN, player.trauma - TRAUMA_DECAY*time.delta_seconds);
            player.shake_camera(time.seconds_since_startup);
        }
    }
}