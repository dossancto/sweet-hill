use bevy::prelude::*;

use crate::configuration::gun_components::GunType;

#[derive(Event)]
pub struct TakeGunEvent {
    pub gun_to_spawn: GunType,
}
