use bevy::prelude::*;

use crate::configuration::gun_bag::GunsBag;

pub mod gun_bag;
pub mod gun_components;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<GunsBag>(GunsBag {
        guns: default(),
        max_guns: 2,
    });
}
