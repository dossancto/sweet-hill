use bevy::prelude::*;

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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunFireAuto {
    pub cadence: f32, // Rounds per second
    pub enabled: bool,
}

impl Default for GunFireAuto {
    fn default() -> Self {
        Self {
            cadence: 0.2,
            enabled: false,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunFireSemiAuto {
    pub delay_after_shoot: f32, // Rounds per second
    pub enabled: bool,
}

impl Default for GunFireSemiAuto {
    fn default() -> Self {
        Self {
            delay_after_shoot: 0.5,
            enabled: false,
        }
    }
}

#[derive(Bundle)]
pub struct GunBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
}

impl GunBundle {
    pub fn new(gun: Gun, ammo: GunAmmo) -> Self {
        Self { gun, ammo }
    }
}

