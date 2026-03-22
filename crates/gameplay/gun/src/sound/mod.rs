use bevy::prelude::*;
mod events;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(events::plugin);
}
