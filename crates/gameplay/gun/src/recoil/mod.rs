use bevy::prelude::*;

mod systems;

pub mod configurations;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((systems::plugin,));
}
