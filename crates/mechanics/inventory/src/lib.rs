use bevy::prelude::*;

pub mod switch_items;

pub fn plugin(app: &mut App) {
    app.add_plugins((switch_items::plugin,));
}
