use bevy::prelude::*;

pub mod bag;
pub mod switch_items;
pub mod take;

pub fn plugin(app: &mut App) {
    app.add_plugins((switch_items::plugin, bag::plugin, take::plugin));
}
