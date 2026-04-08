use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

use crate::{
    configuration::gun_components::{ActiveGun, Gun},
    reload::configurations::components::GunReloading,
};

use crate::inputs::ToogleActiveGun;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(switch_to_next_gun);
    app.add_observer(hide_gun);
    app.add_observer(show_gun);
}

fn switch_to_next_gun(
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

fn hide_gun(
    on: On<Remove, ActiveGun>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now inactive, hiding it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Hidden);
        return;
    };

    if *visibility == Visibility::Hidden {
        return;
    }

    *visibility = Visibility::Hidden;

    return;
}

fn show_gun(
    on: On<Add, ActiveGun>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now actieve, showing it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Visible);
        return;
    };

    if *visibility == Visibility::Visible {
        return;
    }

    *visibility = Visibility::Visible;

    return;
}
