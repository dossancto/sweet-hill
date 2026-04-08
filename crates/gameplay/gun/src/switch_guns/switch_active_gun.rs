use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

use crate::{
    configuration::gun_components::{ActiveGun, Gun},
    reload::configurations::components::GunReloading,
};

use crate::inputs::ToogleActiveGun;

pub(crate) fn switch_to_next_gun(
    _on: On<Start<ToogleActiveGun>>,
    guns_q: Query<(Entity, &Gun), (With<Gun>, Without<ActiveGun>)>,
    active_gun_query: Single<(Entity, &Gun), (With<ActiveGun>, Without<GunReloading>)>,
    mut commands: Commands,
) {
    let (active_gun_entity, _gun) = active_gun_query.into_inner();

    let next_gun_entity = guns_q.iter().next().map(|(entity, _)| entity);

    if let Some(next_gun_entity) = next_gun_entity {
        commands.entity(active_gun_entity).remove::<ActiveGun>();
        commands.entity(next_gun_entity).insert(ActiveGun);
    }
}
