#[derive(Debug)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
}

impl Player {
    pub fn new(yaw: f32, pitch: f32) -> Self { // <- Self is `Player` because we are in `impl Player`
        Self { // <- implicit return because no semicolon
            yaw, // <- this is the short form of `yaw: yaw,`
            pitch,
        }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
    }
}
