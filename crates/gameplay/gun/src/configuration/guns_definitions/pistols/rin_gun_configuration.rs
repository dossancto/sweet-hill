use bevy::prelude::*;
use states::inventory::items::PickableItem;
use utils::effects::camera_sway::SwayItem;

use crate::{
    aims::aim_configurations::components::{GunAimPosition, GunAiming},
    configuration::gun_components::Gun,
    firing::{configurations::components::GunFireRate, firing_types::domain::FireTypeBullet},
    recoil::configurations::components::{GunRecoil, GunRecoilOptions},
    reload::{configurations::components::GunReload, domain::GunAmmo},
};

#[derive(Bundle)]
pub struct RinGunConfigurationBundle {
    pub gun: Gun,
    pub name: Name,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub fire_mode: GunFireRate,
    pub recoil: GunRecoil,
    pub aim: GunAiming,
    pub fire_type: FireTypeBullet,
    pub sway_item: SwayItem,
    pub visibility: Visibility,
    pub pickable_item: PickableItem,
    pub target_aim_position: GunAimPosition,
}

impl Default for RinGunConfigurationBundle {
    fn default() -> Self {
        Self {
            gun: Gun {
                id: "Rin".to_string(),
                name: "Rin".to_string(),
                damage: 50.0,
                range: 100.0,
                damage_falloff_per_hit: 5f32,
            },
            name: Name::new("Rin"),
            ammo: GunAmmo::new(6, 10),
            reload: GunReload { reload_time: 4f32 },
            fire_mode: GunFireRate::fire_mode_auto(0.8f32),
            recoil: GunRecoil::new(GunRecoilOptions {
                vertical_recoil_range: 150.0..180.0,
                horizontal_recoil_range: -5f32..5f32,
                tension: 1.,
                friction: 5.,
                recovery_timer: Timer::from_seconds(0.5f32, TimerMode::Once),
            }),
            aim: GunAiming {
                aiming_time: 70,
                zoom_level: 60.0,
            },
            fire_type: FireTypeBullet,
            sway_item: SwayItem {
                sensitivity: 0.002,
                smoothness: 10.0,
                max_sway: 0.8,
            },
            visibility: Visibility::Hidden,
            pickable_item: PickableItem,
            target_aim_position: GunAimPosition {
                target_position: Vec3::new(0.3, -0.5, 0.5),
                target_rotation: Quat::from_euler(
                    EulerRot::XYZ,
                    0.0f32.to_radians(),
                    4.0f32.to_radians(),
                    0.0f32.to_radians(),
                ),
            },
        }
    }
}
