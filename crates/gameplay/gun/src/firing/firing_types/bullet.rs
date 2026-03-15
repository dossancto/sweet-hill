use crate::{
    configuration::gun_components::{
        ActiveGun, Gun, GunAmmo, GunFireAuto, GunFireSemiAuto, GunReloading,
    },
    firing::{
        events::BulletGunFired,
        gun_can_shoot::{can_auto_can_shoot, can_semi_auto_can_shoot},
    },
};
use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use states::player::PlayerCamera;
use third_party::avian3d::CollisionLayer;

pub(crate) fn shoot_semi_auto_bullets(
    gun: Single<(&Gun, &mut GunAmmo, &mut GunFireSemiAuto), With<ActiveGun>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (gun, mut ammo, mut semi_auto) = gun.into_inner();

    let elapsed_time = time.elapsed_secs();

    if can_semi_auto_can_shoot(&ammo, &semi_auto, elapsed_time) == false {
        return;
    }

    commands.trigger(BulletGunFired { gun: gun.clone() });

    ammo.current_ammo = ammo.current_ammo - 1;

    semi_auto.last_shot_time = elapsed_time;
}

pub(crate) fn shoot_auto_bullets(
    gun: Single<(&Gun, &mut GunAmmo, &mut GunFireAuto), With<ActiveGun>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (gun, mut ammo, mut auto) = gun.into_inner();

    let elapsed_time = time.elapsed_secs_wrapped();

    if can_auto_can_shoot(&ammo, &auto, elapsed_time) == false {
        return;
    }

    commands.trigger(BulletGunFired { gun: gun.clone() });

    ammo.current_ammo = ammo.current_ammo - 1;

    auto.last_shot_time = elapsed_time;
}

pub fn on_shoot_bullets(
    on: On<BulletGunFired>,
    player: Single<(&GlobalTransform, Entity), With<PlayerCamera>>,
    spatial_query: SpatialQuery,
) {
    let (player, _) = player.into_inner();

    let gun = &on.gun;

    let camera_transform = player.compute_transform();

    let hits = spatial_query.ray_hits(
        camera_transform.translation,
        camera_transform.forward(),
        gun.range,
        10,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Hittable),
    );

    for hit in hits {
        println!("Hit entity: {:?}", hit.entity);
    }
}
