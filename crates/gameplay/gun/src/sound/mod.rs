use bevy::prelude::*;
mod events;

pub mod asset_loader;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, asset_loader::plugin));
}
