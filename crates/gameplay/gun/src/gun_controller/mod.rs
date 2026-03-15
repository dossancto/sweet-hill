use bevy::prelude::*;

use crate::gun_controller::handle_input::handle_tap_fire;

mod gun_can_shoot;
mod handle_input;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_tap_fire);
}
