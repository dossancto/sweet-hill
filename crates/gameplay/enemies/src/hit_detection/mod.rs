use crate::hit_detection::events_handlers::process_enemy_damage;
use bevy::prelude::*;

mod events_handlers;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(process_enemy_damage);
}
