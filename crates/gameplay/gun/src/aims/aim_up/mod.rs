use bevy::prelude::*;

mod handlers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(handlers::plugin);
}
