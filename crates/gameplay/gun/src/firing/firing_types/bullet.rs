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
use states::{
    hittable::{Hit, Hittable},
    player::{Player, PlayerCamera},
};
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
    player_collider: Single<Entity, With<Player>>,
    hittable_query: Query<&Hittable>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
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
        &SpatialQueryFilter::from_mask(CollisionLayer::Hittable)
            .with_excluded_entities([*player_collider]),
    );

    let initial_damage = gun.damage;

    for (i, hit) in hits.iter().enumerate() {
        let damage_falloff = gun.damage_falloff_per_hit.powi(i as i32);

        let damage = initial_damage * damage_falloff;

        let node = hittable_query.get(hit.entity).ok();

        if node.is_none() {
            continue;
        }

        commands.trigger(Hit {
            target: hit.entity,
            damage,
        });
    }
}
