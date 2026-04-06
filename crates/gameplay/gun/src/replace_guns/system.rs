use bevy::prelude::*;

use crate::{
    configuration::{
        gun_bag::GunBag, gun_components::ActiveGun,
        guns_definitions::assalt_rifles::m4a1_gun_configuration::M4A1GunConfigurationBundle,
    },
    firing::firing_types::bullet::shoot_auto_bullets,
};

pub(super) fn plugin(_app: &mut App) {}

pub fn grep_gun(mut gun_bag: ResMut<GunBag>, mut commands: Commands) {
    let auto_bullet_system = commands.register_system(shoot_auto_bullets);

    if gun_bag.guns_slot_available() {
        let a = commands
            .spawn(M4A1GunConfigurationBundle::new(auto_bullet_system))
            .insert((ActiveGun, Name::new("m4a1".to_string())))
            .id();

        gun_bag.add_gun("m4a1".into(), a);

        // TODO: Append a new gun to Gun Bag
        // panic!("No available slot for adding a new gun");
        return;
    }

    return;
    // TODO: Replace the current gun with the new one
}
