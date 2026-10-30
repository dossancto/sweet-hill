use bevy::prelude::*;

pub mod assalt_rifles;
pub mod pistols;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(assalt_rifles::plugin);
}
