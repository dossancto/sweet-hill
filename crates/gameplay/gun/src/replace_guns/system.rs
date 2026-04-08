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

pub fn print_active_guns(active_guns_q: Query<&Name, With<ActiveGun>>) {
    for gun in active_guns_q.iter() {
        warn!("Active gun: {}", gun);
    }
}

pub fn grep_gun(
    mut gun_bag: ResMut<GunBag>,
    mut commands: Commands,
    active_guns_q: Query<Entity, With<ActiveGun>>,
) {
    let auto_bullet_system = commands.register_system(shoot_auto_bullets);

    if gun_bag.guns_slot_available() {
        for gun in active_guns_q {
            commands.entity(gun).remove::<ActiveGun>();
        }

        let b = commands
            .spawn(M1GunConfigurationBundle::new(auto_bullet_system.clone()))
            .insert(Name::new("m1".to_string()))
            .id();

        gun_bag.add_gun("m1".into(), b);

        let a = commands
            .spawn(M4A1GunConfigurationBundle::new(auto_bullet_system.clone()))
            .insert((ActiveGun, Name::new("m4a1".to_string())))
            .id();

        gun_bag.add_gun("m4a1".into(), a);

        warn!("Added a new gun to Gun Bag");

        let bag_len = gun_bag.guns.len();

        warn!("Current number of guns in Gun Bag: {bag_len}");

        // TODO: Append a new gun to Gun Bag
        // panic!("No available slot for adding a new gun");
        return;
    } else {
    }

    return;
    // TODO: Replace the current gun with the new one
}
