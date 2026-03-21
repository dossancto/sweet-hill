use bevy::prelude::*;

pub mod camera_sway;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera_sway::plugin);
}
