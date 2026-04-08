use bevy::prelude::*;

use crate::{
    configuration::guns_definitions::assalt_rifles::m4a1_gun_configuration::M4A1GunConfigurationBundle,
    firing::firing_types::bullet::shoot_auto_bullets,
};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmoText;

/// Marker component indicating that an entity currently has an active gun.
///
/// This component can be used to identify which entity is currently wielding or using a gun.
/// It does not contain any data, and is typically used in systems to filter or query entities
/// with an active gun state.
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

pub enum GunType {
    M4A1,
    M1,
}

impl GunType {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        let auto_bullet_system = commands.register_system(shoot_auto_bullets);

        match self {
            GunType::M4A1 => commands
                .spawn(M4A1GunConfigurationBundle::new(auto_bullet_system))
                .id(),
            GunType::M1 => commands
                .spawn(M4A1GunConfigurationBundle::new(auto_bullet_system))
                .id(),
        }
    }
}
