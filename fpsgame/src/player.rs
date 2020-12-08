use bevy::math::*;

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
        }
    }

    pub fn get_look_direction(&self) -> Vec3 {
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(
            direction.x(),
            direction.y() * self.pitch.cos() - direction.z() * self.pitch.sin(),
            direction.z() * self.pitch.cos() - direction.y() * self.pitch.sin(),
        );

        let direction = Vec3::new(
            direction.x() * self.yaw.cos() - direction.z() * self.yaw.sin(),
            direction.y(),
            direction.z() * self.yaw.cos() - direction.x() * self.yaw.sin(),
        );

        direction.normalize()
    }
}
