use bevy::prelude::*;
use utils::effects::camera_sway::SwayItem;

use crate::{
    aims::aim_configurations::components::GunAiming,
    configuration::gun_components::Gun,
    firing::{configurations::components::GunFireRate, firing_types::domain::FireTypeBullet},
    models::gun_models::gun_m4a1::definition::GunM4A1,
    recoil::configurations::components::{GunRecoil, GunRecoilOptions},
    reload::{configurations::components::GunReload, domain::GunAmmo},
};

#[derive(Bundle)]
pub struct M4A1GunConfigurationBundle {
    pub gun: Gun,
    pub name: Name,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub fire_mode: GunFireRate,
    pub recoil: GunRecoil,
    pub aim: GunAiming,
    pub model: GunM4A1,
    pub fire_type: FireTypeBullet,
    pub sway_item: SwayItem,
    pub visibility: Visibility,
}

impl Default for M4A1GunConfigurationBundle {
    fn default() -> Self {
        Self {
            gun: Gun {
                id: "m4a1".to_string(),
                name: "m4a1".to_string(),
                damage: 50.0,
                range: 100.0,
                damage_falloff_per_hit: 5f32,
            },
            name: Name::new("M4A1"),
            ammo: GunAmmo {
                max_clip_size: 30,
                max_stock_size: 90 * 10,
                current_on_clip: 30,
                current_on_stock: 90 * 10,
            },
            reload: GunReload { reload_time: 1f32 },
            fire_mode: GunFireRate::fire_mode_auto(10f32),
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
            model: GunM4A1,
            fire_type: FireTypeBullet,
            sway_item: SwayItem {
                sensitivity: 0.002,
                smoothness: 10.0,
                max_sway: 0.1,
            },
            visibility: Visibility::Hidden,
        }
    }
}
