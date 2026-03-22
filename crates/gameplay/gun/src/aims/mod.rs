use bevy::prelude::*;

mod handlers;

pub fn plugin(app: &mut App) {
    app.add_plugins(handlers::plugin);
}
