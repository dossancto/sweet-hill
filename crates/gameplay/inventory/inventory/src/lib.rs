use bevy::prelude::*;

pub mod collect;
pub mod switch_item;

pub fn plugin(app: &mut App) {
    app.add_plugins((collect::plugin, switch_item::plugin));
}
