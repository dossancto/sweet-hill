use bevy::prelude::*;

pub mod active_items;
pub mod switch_items;

pub fn plugin(app: &mut App) {
    app.add_plugins((active_items::plugin, switch_items::plugin));
}
