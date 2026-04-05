use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Component, Debug)]
pub struct GunFireAuto {
    pub delay_after_shoot: f32, // Rounds per second
    pub last_shot_time: f32,    // Rounds per second
    pub action: SystemId,
}
impl GunFireAuto {
    pub fn new(rounds_per_sec: f32, action: SystemId) -> Self {
        Self {
            delay_after_shoot: 1000f32 / rounds_per_sec / 1000f32,
            last_shot_time: 0.0,
            action,
        }
    }
}

#[derive(Component, Debug)]
pub struct GunFireSemiAuto {
    pub delay_after_shoot: f32, // Rounds per second
    pub last_shot_time: f32,    // Rounds per second
    pub system_id: SystemId,
}
impl GunFireSemiAuto {
    pub fn new(delay_after_shoot: f32, system_id: SystemId) -> Self {
        Self {
            delay_after_shoot,
            last_shot_time: 0.0,
            system_id,
        }
    }
}

