use bevy::prelude::*;

pub mod can_interact_ui;
pub mod interacting_ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((can_interact_ui::plugin, interacting_ui::plugin));
}
