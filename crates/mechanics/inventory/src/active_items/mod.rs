use bevy::prelude::*;

pub mod single_active_item;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(single_active_item::plugin);
}
