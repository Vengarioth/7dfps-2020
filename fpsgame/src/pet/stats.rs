
pub struct PetStats {
    damage: f32,
    range: f32,
    attack_speed: f32,
}

impl PetStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for PetStats {
    fn default() -> Self {
        Self {
            damage: 1.0,
            range: 5.0,
            attack_speed: 1.0,
        }
    }
}
