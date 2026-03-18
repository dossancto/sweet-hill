use crate::{
    configuration::gun_components::{ActiveGun, GunReloading},
    inputs::GunReloadTrigger,
    reload::domain::GunAmmo,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_gun_reload(
    _on: On<Start<GunReloadTrigger>>,
    gun_entity: Single<(Entity, &GunAmmo), (With<ActiveGun>, Without<GunReloading>)>,
    mut commands: Commands,
) {
    let (entity, gun_ammo) = gun_entity.into_inner();

    if gun_ammo.full_clip() {
        return;
    }

    commands.entity(entity).insert(GunReloading {
        time_to_reload: Timer::from_seconds(1.5, TimerMode::Once),
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

        info!(
            "reloading time, {}",
            reloading.time_to_reload.remaining_secs()
        );

        if reloading.time_to_reload.is_finished() == false {
            continue;
        }

        ammo.reload();

        commands.entity(entity).remove::<GunReloading>();
    }
}
