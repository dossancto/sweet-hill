use bevy::{ecs::system::SystemId, prelude::*};

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
    pub range: f32,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmo {
    pub magazine_size: usize,
    pub stock_size: usize,
    pub current_ammo: usize,
    pub current_stock_ammo: usize,
}

#[derive(Component, Debug)]
pub struct GunFireAuto {
    pub cadence: f32, // Rounds per second
    pub action: SystemId,
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
