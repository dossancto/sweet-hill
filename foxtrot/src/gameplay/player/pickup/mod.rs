//! Player pickup handling.

use bevy::prelude::*;

mod collision;
mod sound;
mod ui;
pub(crate) mod state;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((collision::plugin, sound::plugin, ui::plugin));
}
