use bevy::prelude::*;

use crate::configuration::guns_definitions::assalt_rifles::{
    m1_gun_configuration::M1GunConfigurationBundle,
    m4a1_gun_configuration::M4A1GunConfigurationBundle,
};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmoText;

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
    pub fn spawn(&self, commands: &mut Commands, gun_holder: Entity) -> Entity {
        match self {
            GunType::M4A1 => {
                let gun = commands.spawn(M4A1GunConfigurationBundle::default()).id();

                commands.entity(gun_holder).add_child(gun);

                gun
            }
            GunType::M1 => {
                let gun = commands.spawn(M1GunConfigurationBundle::default()).id();

                commands.entity(gun_holder).add_child(gun);

                gun
            }
        }
    }
}

impl ToString for GunType {
    fn to_string(&self) -> String {
        match self {
            GunType::M4A1 => "M4A1".to_string(),
            GunType::M1 => "M1".to_string(),
        }
    }
}
