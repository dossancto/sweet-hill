use bevy::prelude::*;

pub mod gun_models;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((gun_models::plugin,));
}
