use bevy::prelude::*;

use crate::{
    configuration::gun_components::{ActiveGun, Gun},
    replace_guns::events::TakeGunEvent,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_gun_replacement);
}

pub fn handle_gun_replacement(
    on: On<TakeGunEvent>,
    mut commands: Commands,
    active_guns_query: Query<Entity, With<ActiveGun>>,
    guns_q: Query<Entity, With<Gun>>,
) {
    let len = guns_q.iter().len();

    if len >= 2 {
        let current_gun_entity = active_guns_query.iter().next().unwrap();
        commands.entity(current_gun_entity).despawn();
    }

    for active_gun in active_guns_query.iter() {
        commands.entity(active_gun).remove::<ActiveGun>();
    }

    let spawned_gun = on.gun_to_spawn.spawn(&mut commands);

    commands.entity(spawned_gun).insert(ActiveGun);
}
