use bevy::prelude::*;
use bevy_seedling::prelude::*;
use states::{hittable::Hit, player::PlayerCamera};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_fire_hit);
}

fn on_fire_hit(
    _on: On<Hit>,
    assets: Res<AssetServer>,
    player_transform: Single<&Transform, With<PlayerCamera>>,
    mut commands: Commands,
) {
    let sound = assets.load("audio/sound_effects/hit/hit01.wav");

    commands.spawn((
        **player_transform,
        SamplePlayer::new(sound).with_volume(Volume::Linear(3.0)),
        SpatialPool,
    ));
}
