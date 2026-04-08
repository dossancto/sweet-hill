use bevy::prelude::*;

use crate::{configuration::gun_components::ActiveGun, replace_guns::events::TakeGunEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_gun_replacement);
}

pub fn handle_gun_replacement(
    on: On<TakeGunEvent>,
    mut commands: Commands,
    active_guns_query: Query<Entity, With<ActiveGun>>,
) {
    for active_gun in active_guns_query.iter() {
        commands.entity(active_gun).remove::<ActiveGun>();
    }

    let spawned_gun = on.gun_to_spawn.spawn(&mut commands);

    commands.entity(spawned_gun).insert(ActiveGun);

    // gun_bag.add_gun(on.gun_to_spawn.to_string(), spawned_gun);

    return;

    // TODO: Add Gun replace logic
}
