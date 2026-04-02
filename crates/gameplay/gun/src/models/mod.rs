use bevy::prelude::*;

use crate::models::gun_mark_spawn::spawn_gun_holder;

mod gun_mark_spawn;
pub mod gun_models;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_gun_holder);
    app.add_plugins((gun_models::plugin,));
}
