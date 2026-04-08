use bevy::{ecs::system::SystemId, prelude::*};

use crate::{
    aims::aim_configurations::components::GunAiming,
    configuration::gun_components::Gun,
    firing::configurations::components::GunFireAuto,
    models::gun_models::gun_m4a1::definition::GunM4A1,
    recoil::configurations::components::{GunRecoil, GunRecoilOptions},
    reload::{configurations::components::GunReload, domain::GunAmmo},
};

#[derive(Bundle)]
pub struct M1GunConfigurationBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub fire_mode: GunFireAuto,
    pub recoil: GunRecoil,
    pub aim: GunAiming,
    // pub model: GunM4A1,
}

impl M1GunConfigurationBundle {
    pub fn new(system_id: SystemId) -> Self {
        Self {
            gun: Gun {
                id: "M1".to_string(),
                name: "M1".to_string(),
                damage: 150.0,
                range: 100.0,
                damage_falloff_per_hit: 5f32,
            },
            ammo: GunAmmo::new(10, 4),
            reload: GunReload { reload_time: 1f32 },
            fire_mode: GunFireAuto::new(10f32, system_id),
            recoil: GunRecoil::new(GunRecoilOptions {
                vertical_recoil_range: 4.0..5.0,
                horizontal_recoil_range: -5f32..5f32,
                tension: 1.,
                friction: 5.,
                recovery_timer: Timer::from_seconds(0.2, TimerMode::Once),
            }),
            aim: GunAiming {
                aiming_time: 50,
                zoom_level: 60.0,
            },
            // model: GunM4A1,
        }
    }
}
