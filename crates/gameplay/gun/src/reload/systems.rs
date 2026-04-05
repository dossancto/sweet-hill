use crate::{
    configuration::gun_components::ActiveGun,
    inputs::GunReloadTrigger,
    reload::{
        configurations::components::{GunReload, GunReloading},
        domain::GunAmmo,
    },
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_gun_reload(
    _on: On<Start<GunReloadTrigger>>,
    gun_entity: Single<(Entity, &GunAmmo, &GunReload), (With<ActiveGun>, Without<GunReloading>)>,
    mut commands: Commands,
) {
    let (entity, gun_ammo, gun_reload) = gun_entity.into_inner();

    if gun_ammo.full_clip() {
        return;
    }

    commands.entity(entity).insert(GunReloading {
        time_to_reload: Timer::from_seconds(gun_reload.reload_time, TimerMode::Once),
    });
}

pub(crate) fn process_reloading_guns(
    mut gun: Query<
        (Entity, &mut GunReloading, &mut GunAmmo),
        (With<ActiveGun>, With<GunReloading>),
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut reloading, mut ammo) in gun.iter_mut() {
        reloading.time_to_reload.tick(time.delta());

        if reloading.time_to_reload.is_finished() == false {
            continue;
        }

        ammo.reload();

        commands.entity(entity).remove::<GunReloading>();
    }
}
