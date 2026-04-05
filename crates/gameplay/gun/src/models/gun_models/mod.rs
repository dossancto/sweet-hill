use bevy::prelude::*;

pub(crate) mod gun_m4a1;
pub(crate) mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((gun_m4a1::plugin, spawn::plugin));
}
