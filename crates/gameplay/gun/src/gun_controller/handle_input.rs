use crate::{
    configuration::gun_components::{ActiveGun, Gun, GunAmmo},
    firing::states::GunFireTrigger,
    states::GunState,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_tap_fire(
    _on: On<Start<GunFireTrigger>>,
    gun: Single<(&Gun, &mut GunAmmo), With<ActiveGun>>,
    gun_state: Res<State<GunState>>,
) {
    let gun_state = gun_state.get();

    if gun_state != &GunState::ReadyToFire {
        return;
    }
}

pub(crate) fn handle_hold_fire() {}

