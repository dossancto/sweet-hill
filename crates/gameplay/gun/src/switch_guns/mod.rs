use bevy::prelude::*;

mod switch_active_gun;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((switch_active_gun::plugin,));
}
