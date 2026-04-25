use bevy::prelude::*;

pub mod interaction;
pub mod inventory;

pub fn plugin(app: &mut App) {
    app.add_plugins((inventory::plugin, interaction::plugin));
}
