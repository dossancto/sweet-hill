use bevy::prelude::*;

use crate::reload::sounds::reload_soud::{handle_gun_reload_end_sound, handle_gun_reload_start_sound};

mod reload_soud;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_gun_reload_start_sound);
    app.add_observer(handle_gun_reload_end_sound);
}
