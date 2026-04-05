use bevy::prelude::*;

use crate::configuration::gun_components::Gun;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_gun);
}

pub fn spawn_gun(_on: On<Insert, Gun>) {
    panic!("Not implemented yet");
}
