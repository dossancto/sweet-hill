use bevy::prelude::*;

use crate::configuration::gun_bag::GunsBag;

pub mod gun_bag;
pub mod gun_components;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GunsBag>();
}
