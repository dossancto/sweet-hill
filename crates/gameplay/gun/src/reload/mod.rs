use bevy::prelude::*;

use crate::reload::systems::handle_gun_reload;

pub mod domain;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_observer(handle_gun_reload);

}
