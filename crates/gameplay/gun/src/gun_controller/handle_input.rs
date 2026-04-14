use crate::{
    configuration::gun_components::Gun,
    firing::configurations::components::GunFireRate,
    inputs::GunFireTrigger,
    states::{GunState, ProcessGunFire},
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use states::inventory::active_item::ActiveItem;

pub(crate) fn handle_tap_fire(
    _on: On<Start<GunFireTrigger>>,
    gun: Single<(&Gun, &GunFireRate), With<ActiveItem>>,
    gun_state: Res<State<GunState>>,
    mut commands: Commands,
) {
    let gun_state = gun_state.get();

    let (_gun, _gun_fire_semi_auto) = gun.into_inner();

    if gun_state != &GunState::ReadyToFire {
        return;
    }

    commands.trigger(ProcessGunFire);
}

pub(crate) fn handle_hold_fire(
    _on: On<Ongoing<GunFireTrigger>>,
    gun: Single<(&Gun, &GunFireRate), With<ActiveItem>>,
    gun_state: Res<State<GunState>>,
    mut commands: Commands,
) {
    let gun_state = gun_state.get();

    let (_gun, _gun_fire_semi_auto) = gun.into_inner();

    if gun_state != &GunState::ReadyToFire {
        return;
    }

    commands.trigger(ProcessGunFire);
}
