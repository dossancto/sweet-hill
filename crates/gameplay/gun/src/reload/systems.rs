use crate::{
    configuration::gun_components::{ActiveGun, Gun, GunAmmo},
    reload::domain::GunReloadTrigger,
    states::GunState,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_gun_reload(
    _on: On<Start<GunReloadTrigger>>,
    gun: Single<(&Gun, &mut GunAmmo), With<ActiveGun>>,
    gun_state: Res<State<GunState>>,
) {
    let gun_state = gun_state.get();

    let (_gun, mut ammo) = gun.into_inner();

    if gun_state != &GunState::ReadyToFire {
        return;
    }

    let left_ammo = ammo.magazine_size - ammo.current_ammo;

    ammo.current_ammo = ammo.magazine_size;
    ammo.current_stock_ammo = ammo.current_stock_ammo - left_ammo;
}
