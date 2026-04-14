use bevy::prelude::*;
use bevy_seedling::prelude::*;
use states::{hittable::Hit, player::PlayerCamera};

use crate::{firing::events::BulletGunFired, sound::asset_loader::GunSoundsAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_fire_hit);
    app.add_observer(on_shoot_bullets);
}

fn on_fire_hit(
    _on: On<Hit>,
    player_transform: Single<&Transform, With<PlayerCamera>>,
    sounds: Res<GunSoundsAssets>,
    mut commands: Commands,
) {
    let sound = sounds.bullet_fire_hit.clone();

    commands.spawn((
        **player_transform,
        SamplePlayer::new(sound).with_volume(Volume::Linear(3.0)),
        SpatialPool,
    ));
}

fn on_shoot_bullets(
    _on: On<BulletGunFired>,

    _assets: Res<AssetServer>,
    _player_transform: Single<&Transform, With<PlayerCamera>>,
    mut _commands: Commands,
) {
    // let sound = assets.load("audio/sound_effects/fire/fire01.wav");
    //
    // commands.spawn((
    //     **player_transform,
    //     SamplePlayer::new(sound).with_volume(Volume::Linear(3.0)),
    //     SpatialPool,
    // ));
}
