use crate::{firing::configurations::components::GunFireRate, reload::domain::GunAmmo};

pub(crate) fn can_semi_auto_can_shoot(
    ammo: &GunAmmo,
    semi_auto: &GunFireRate,
    elapse_secs: f32,
) -> bool {
    if ammo.has_ammo_on_clip() == false {
        return false;
    }

    let time_since_last_shot = elapse_secs - semi_auto.last_shot_time;

    if time_since_last_shot < semi_auto.delay_after_shoot {
        return false;
    }

    true
}
