use bevy::prelude::*;

pub(crate) mod collect;
pub mod components;
pub mod handlers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((handlers::plugin,));
}
