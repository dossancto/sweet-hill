use bevy::prelude::*;

use crate::configuration::{gun_bag::GunBag, gun_components::ActiveGun};

pub mod gun_bag;
pub mod gun_components;
pub mod guns_definitions;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<GunBag>(GunBag {
        guns: default(),
        max_guns: 2,
    });

    app.add_plugins(guns_definitions::plugin);

    app.add_observer(assert_unique_active_gun);
}

fn assert_unique_active_gun(
    _add: On<Insert, ActiveGun>,
    active_guns: Query<Entity, With<ActiveGun>>,
    mut commands: Commands,
) {
    for active_gun in active_guns.iter().skip(1) {
        commands.entity(active_gun).remove::<ActiveGun>();
    }
}
