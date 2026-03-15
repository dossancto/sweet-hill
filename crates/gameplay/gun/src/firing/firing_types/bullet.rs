use crate::{
    configuration::gun_components::{ActiveGun, Gun, GunAmmo, GunFireSemiAuto},
    firing::gun_can_shoot::semi_auto_can_shoot,
};
use bevy::prelude::*;

pub(crate) fn shoot_bullets(
    gun: Single<(&Gun, &mut GunAmmo, &mut GunFireSemiAuto), With<ActiveGun>>,
    time: Res<Time>,
) {
    let (_gun, mut ammo, mut semi_auto) = gun.into_inner();

    let elapsed_time = time.elapsed_secs();

    if semi_auto_can_shoot(&ammo, &semi_auto, elapsed_time) == false {
        return;
    }

    ammo.current_ammo = ammo.current_ammo - 1;

    semi_auto.last_shot_time = elapsed_time;
}

pub(crate) fn shoot_explosion() {}
