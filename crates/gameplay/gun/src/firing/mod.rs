use bevy::prelude::*;

pub mod events;
pub mod gun_can_shoot;
pub mod configurations;

pub(crate) mod firing_types;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(firing_types::plugin);
}
