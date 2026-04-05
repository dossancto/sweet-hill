use crate::{
    configuration::gun_components::{ActiveGun, Gun},
    firing::configurations::components::{GunFireAuto, GunFireSemiAuto},
    inputs::GunFireTrigger,
    states::GunState,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_tap_fire(
    _on: On<Start<GunFireTrigger>>,
    gun: Single<(&Gun, &GunFireSemiAuto), With<ActiveGun>>,
    gun_state: Res<State<GunState>>,
    mut commands: Commands,
) {
    let gun_state = gun_state.get();

    let (_gun, gun_fire_semi_auto) = gun.into_inner();

    if gun_state != &GunState::ReadyToFire {
        return;
    }

    commands.run_system(gun_fire_semi_auto.system_id);
}

pub(crate) fn handle_hold_fire(
    _on: On<Ongoing<GunFireTrigger>>,
    gun: Single<(&Gun, &GunFireAuto), With<ActiveGun>>,
    gun_state: Res<State<GunState>>,
    mut commands: Commands,
) {
    let gun_state = gun_state.get();

    let (_gun, gun_fire_semi_auto) = gun.into_inner();

    if gun_state != &GunState::ReadyToFire {
        return;
    }

    commands.run_system(gun_fire_semi_auto.action);
}
