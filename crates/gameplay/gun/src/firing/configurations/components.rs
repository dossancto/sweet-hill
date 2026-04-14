use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GunFireRate {
    pub delay_after_shoot: f32,
    pub last_shot_time: f32,
}

impl GunFireRate {
    pub fn fire_mode_auto(rounds_per_sec: f32) -> Self {
        Self {
            delay_after_shoot: 1000f32 / rounds_per_sec / 1000f32,
            last_shot_time: 0.0,
        }
    }

    pub fn fire_mode_semi_auto(delay_after_shoot: f32) -> Self {
        Self {
            delay_after_shoot,
            last_shot_time: 0.0,
        }
    }
}
