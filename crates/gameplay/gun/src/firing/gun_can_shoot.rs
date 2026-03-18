use crate::{
    configuration::gun_components::{GunFireAuto, GunFireSemiAuto},
    reload::domain::GunAmmo,
};

pub(crate) fn can_semi_auto_can_shoot(
    ammo: &GunAmmo,
    semi_auto: &GunFireSemiAuto,
    elapse_secs: f32,
) -> bool {
    if ammo.has_ammo() == false {
        return false;
    }

    let time_since_last_shot = elapse_secs - semi_auto.last_shot_time;

    if time_since_last_shot < semi_auto.delay_after_shoot {
        return false;
    }

    true
}

pub(crate) fn can_auto_can_shoot(
    ammo: &GunAmmo,
    semi_auto: &GunFireAuto,
    elapse_secs: f32,
) -> bool {
    if ammo.has_ammo() == false {
        return false;
    }

    let time_since_last_shot = elapse_secs - semi_auto.last_shot_time;

    if time_since_last_shot < semi_auto.delay_after_shoot {
        return false;
    }

    true
}
