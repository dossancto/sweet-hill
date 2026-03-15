use crate::configuration::gun_components::{GunAmmo, GunFireAuto, GunFireSemiAuto};

pub(crate) fn has_enough_ammo(ammo: &GunAmmo) -> bool {
    ammo.current_ammo > 0
}

pub(crate) fn can_semi_auto_can_shoot(
    ammo: &GunAmmo,
    semi_auto: &GunFireSemiAuto,
    elapse_secs: f32,
) -> bool {
    if has_enough_ammo(ammo) == false {
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
    if has_enough_ammo(ammo) == false {
        return false;
    }

    let time_since_last_shot = elapse_secs - semi_auto.last_shot_time;

    if time_since_last_shot < semi_auto.delay_after_shoot {
        return false;
    }

    true
}
