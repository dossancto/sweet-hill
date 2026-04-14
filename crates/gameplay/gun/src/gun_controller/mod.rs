use bevy::prelude::*;

use crate::gun_controller::handle_input::{handle_hold_fire, handle_tap_fire};

mod handle_input;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_tap_fire);
    app.add_observer(handle_hold_fire);
}
