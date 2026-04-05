use bevy::prelude::*;

use crate::reload::systems::{handle_gun_reload, process_reloading_guns};

pub mod domain;
mod sounds;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_observer(handle_gun_reload);
    app.add_systems(Update, process_reloading_guns);
    app.add_plugins(sounds::plugin);
}
