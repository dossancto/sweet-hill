use bevy::prelude::*;
use bevy_seedling::prelude::*;

use states::player::PlayerCamera;

use crate::reload::configurations::components::GunReloading;

pub(super) fn handle_gun_reload_start_sound(
    _on: On<Add, GunReloading>,
    assets: Res<AssetServer>,
    player_transform: Single<&Transform, With<PlayerCamera>>,
    mut commands: Commands,
) {
    let sound = assets.load("audio/sound_effects/guns/reload/reload_start_01.wav");

    commands.spawn((
        **player_transform,
        SamplePlayer::new(sound).with_volume(Volume::Linear(1.0)),
        SpatialPool,
    ));
}

pub(super) fn handle_gun_reload_end_sound(
    _on: On<Remove, GunReloading>,

    assets: Res<AssetServer>,
    player_transform: Single<&Transform, With<PlayerCamera>>,
    mut commands: Commands,
) {
    let sound = assets.load("audio/sound_effects/guns/reload/reload_end_01.wav");

    commands.spawn((
        **player_transform,
        SamplePlayer::new(sound).with_volume(Volume::Linear(1.0)),
        SpatialPool,
    ));
}
