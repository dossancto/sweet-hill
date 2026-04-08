use bevy::prelude::*;
use states::guns::marks::GunHolderMark;

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
    gun_holder_q: Single<Entity, With<GunHolderMark>>,
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

    let gun_holder = gun_holder_q.into_inner();

    let spawned_gun = on.gun_to_spawn.spawn(&mut commands, gun_holder);

    commands.entity(spawned_gun).insert(ActiveGun);
}
