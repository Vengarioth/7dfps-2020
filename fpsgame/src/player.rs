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
}
