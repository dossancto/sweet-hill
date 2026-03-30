use bevy::{ecs::system::SystemId, prelude::*};

use crate::reload::domain::GunAmmo;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmoText;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ActiveGun;

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct Gun {
    pub id: String,
    pub name: String,
    pub damage: f32,
    pub damage_falloff_per_hit: f32,
    pub range: f32,
}

#[derive(Component, Debug)]
pub struct GunReload {
    pub reload_time: f32,
}

/// Component representing the aiming state of a gun. Guns that dont implement this component will
/// not be able to aim, and will always have the same zoom level.
///
/// - `aiming_time`: The time spent aiming, measured in milliseconds (ms).
/// - `zoom_level`: The current zoom level, measured in degrees.
#[derive(Component, Debug)]
pub struct GunAiming {
    /// Time spent aiming, in milliseconds (ms).
    pub aiming_time: u32,
    /// Current zoom level, in degrees.
    pub zoom_level: f32,
}

#[derive(Component, Debug)]
pub struct GunReloading {
    pub time_to_reload: Timer,
}

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

#[derive(Bundle)]
pub struct GunSemiAutoBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub config: GunFireSemiAuto,
}

impl GunSemiAutoBundle {
    pub fn new(
        gun: Gun,
        ammo: GunAmmo,
        reload: GunReload,
        semi_auto_config: GunFireSemiAuto,
    ) -> Self {
        Self {
            gun,
            ammo,
            reload,
            config: semi_auto_config,
        }
    }
}

#[derive(Bundle)]
pub struct GunAutoBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub config: GunFireAuto,
}

impl GunAutoBundle {
    pub fn new(gun: Gun, ammo: GunAmmo, reload: GunReload, auto_config: GunFireAuto) -> Self {
        Self {
            gun,
            ammo,
            reload: reload,
            config: auto_config,
        }
    }
}
