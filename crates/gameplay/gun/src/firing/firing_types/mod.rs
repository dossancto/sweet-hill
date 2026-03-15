use bevy::prelude::*;

use crate::firing::firing_types::bullet::on_shoot_bullets;

pub(crate) mod bullet;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_shoot_bullets);
}
