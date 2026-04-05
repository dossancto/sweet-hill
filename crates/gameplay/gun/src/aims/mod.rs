use bevy::prelude::*;

mod handlers;

pub mod aim_configurations;

pub fn plugin(app: &mut App) {
    app.add_plugins(handlers::plugin);
}
