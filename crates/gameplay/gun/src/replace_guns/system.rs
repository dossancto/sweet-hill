use bevy::prelude::*;

use crate::{
    configuration::{
        gun_bag::GunBag,
        gun_components::ActiveGun,
        guns_definitions::assalt_rifles::{
            m1_gun_configuration::M1GunConfigurationBundle,
            m4a1_gun_configuration::M4A1GunConfigurationBundle,
        },
    },
    firing::firing_types::bullet::shoot_auto_bullets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, print_active_guns);
}

pub fn print_active_guns(
    active_guns_q: Query<&Name, With<ActiveGun>>,
    gun_holder_q: Query<&Name, With<ActiveGun>>,
) {
    warn!("Active gun count: {}", active_guns_q.iter().len());
    warn!("Active gun holders count: {}", gun_holder_q.iter().len());
}
