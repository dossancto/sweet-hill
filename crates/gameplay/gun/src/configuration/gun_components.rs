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
    pub config: GunFireSemiAuto,
}

impl GunSemiAutoBundle {
    pub fn new(gun: Gun, ammo: GunAmmo, semi_auto_config: GunFireSemiAuto) -> Self {
        Self {
            gun,
            ammo,
            config: semi_auto_config,
        }
    }
}

#[derive(Bundle)]
pub struct GunAutoBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub config: GunFireAuto,
}

impl GunAutoBundle {
    pub fn new(gun: Gun, ammo: GunAmmo, auto_config: GunFireAuto) -> Self {
        Self {
            gun,
            ammo,
            config: auto_config,
        }
    }
}
