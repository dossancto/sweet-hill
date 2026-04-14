use bevy::prelude::*;

pub mod aim_up;
pub mod zoom;
pub(crate) mod utils;

pub mod aim_configurations;

pub fn plugin(app: &mut App) {
    app.add_plugins((aim_up::plugin, zoom::plugin));
}
