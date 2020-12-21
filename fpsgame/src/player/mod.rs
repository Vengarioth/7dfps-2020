use bevy::prelude::*;
use crate::command_line::CommandLineOpt;

use crate::movement::{
    Movement,
    MovementData,
    Kinematic,
    GroundedState,
};

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

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player::new(4.012901, 0.3168293),
        Transform::from_translation(Vec3::new(-3.1755996, 5.0, 2.4332705)),
        Movement(Vec3::zero()),
        MovementData::default(), // TODO
        GroundedState::default(),
        Kinematic,
    ));
}

pub fn move_player(
    opt: Res<CommandLineOpt>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Movement)>,
) {
    let mut player_move = Vec3::default();
    if keyboard_input.pressed(KeyCode::W) {
         player_move += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
         player_move += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
         player_move += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
         player_move += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Space) {
         player_move += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::LShift) {
         player_move += Vec3::new(0.0, -1.0, 0.0);
    }

    let delta = time.delta_seconds.min(0.032);

    for (mut player, mut movement) in query.iter_mut() {
        let sin = player.yaw.sin();
        let cos = player.yaw.cos();

        player_move *= delta * 10.0;
        // player_move *= Vec3::new(1.0, 2.0, 1.0);

        player_move = Vec3::new(
            player_move.x() * cos - player_move.z() * sin,
            player_move.y(),
            player_move.z() * cos + player_move.x() * sin,
        );

        // player_move += -Vec3::new(0.0, 9.81 * delta, 0.0);

        movement.0 = player_move;

        if opt.is_debug() && keyboard_input.just_pressed(KeyCode::T) {
            player.add_trauma(0.5);
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,

    /// Werether to activate the current action or not
    pub action: bool,

    /// Total height of the player
    pub height: f32,

    /// Height offset at which the camera is placed
    pub camera_height: f32,

    /// Height offset from which raycasts to the ground are made
    pub raycast_offset: f32,

    /// True if the player is standing on solid ground
    pub grounded: bool,
    /// True if the player was standing on solid ground last frame
    pub was_grounded: bool,
    /// The number of frames since player was last grounded
    pub frames_since_grounded: u32,

    /// True if the player is standing on a slope
    pub on_slope: bool,
    /// True if the player was standing on a slope last frame
    pub was_on_slope: bool,

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

            action: false,

            height: 1.6,
            camera_height: 1.5,
            raycast_offset: 1.0,

            grounded: false,
            was_grounded: false,
            frames_since_grounded: 1000, // arbitrarily high on start, pretending the player was floating in air for a while
            on_slope: false,
            was_on_slope: false,

            trauma: 0.0,
            trauma_yaw: 0.0,
            trauma_pitch: 0.0,
            trauma_roll: 0.0,
        }
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

    pub fn get_look_direction(&self) -> Vec3 {
        let combined_yaw = self.yaw + self.trauma_yaw;
        let combined_pitch = self.pitch + self.trauma_pitch;
        // let combined_roll = self.roll + self.trauma_roll;

        let direction = Vec3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(
            direction.x(),
            direction.y() * combined_pitch.cos() - direction.z() * combined_pitch.sin(),
            direction.z() * combined_pitch.cos() - direction.y() * combined_pitch.sin(),
        );

        let direction = Vec3::new(
            direction.x() * combined_yaw.cos() - direction.z() * combined_yaw.sin(),
            direction.y(),
            direction.z() * combined_yaw.cos() - direction.x() * combined_yaw.sin(),
        );

        direction.normalize()
    }
}

pub fn update_trauma(
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        if player.trauma > 0.0 {
            player.trauma = TRAUMA_MIN.max(player.trauma - TRAUMA_DECAY*time.delta_seconds);
            player.shake_camera(time.seconds_since_startup);
        }
    }
} 

pub fn shake_when_hit_ground(
    mut last_grounded: Local<bool>,
    mut player_query: Query<&mut Player>) {
    for mut player in player_query.iter_mut() {
        if !*last_grounded && player.grounded {
            player.add_trauma(0.5);
        }
        *last_grounded = player.grounded;
    }
} 
