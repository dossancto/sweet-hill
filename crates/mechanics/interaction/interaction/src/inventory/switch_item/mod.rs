use bevy::prelude::*;

pub(crate) mod handler;
pub mod inputs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((handler::plugin,));
}
