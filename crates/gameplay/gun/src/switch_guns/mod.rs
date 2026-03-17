use bevy::prelude::*;

use crate::switch_guns::switch_active_gun::switch_to_next_gun;

mod switch_active_gun;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(switch_to_next_gun);
}
