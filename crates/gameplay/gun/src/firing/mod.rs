use bevy::prelude::*;

pub mod states;
pub mod systems;
pub mod gun_can_shoot;

pub(crate) mod firing_types;

pub(super) fn plugin(app: &mut App) {}
