use audio::SpatialPool;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_seedling::sample::SamplePlayer;

use crate::gameplay::player::{assets::PlayerAssets, camera::PlayerCamera};

use super::states::{FlashlightLight, PlayerFlashlightState, ToggleFlashlight};

pub(crate) fn on_toggle_flashlight(
    _on: On<Start<ToggleFlashlight>>,
    state: Res<State<PlayerFlashlightState>>,
    player_camera: Single<Entity, With<PlayerCamera>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayerFlashlightState>>,
    mut player_assets: ResMut<PlayerAssets>,
    flashlight: Query<(Entity, &ChildOf), With<FlashlightLight>>,
) {
    let current_state = state.get();

    if current_state == &PlayerFlashlightState::Disabled {
        // TODO: maybe play a "click" sound to indicate the flashlight can't be toggled
        return;
    }

    let new_state = match current_state {
        PlayerFlashlightState::Off => PlayerFlashlightState::On,
        PlayerFlashlightState::On => PlayerFlashlightState::Off,
        _ => unreachable!(),
    };

    next_state.set(new_state.clone());

    let player = player_camera.into_inner();
    let rng = &mut rand::rng();

    let flashlight_children_to_remove = flashlight.into_iter();

    for (entity, _child) in flashlight_children_to_remove {
        commands.entity(entity).despawn();
    }

    if new_state == PlayerFlashlightState::On {
        let sound = player_assets.land_sounds.pick(rng).clone();
        commands.entity(player).with_child((
            SamplePlayer::new(sound),
            SpatialPool,
            Transform::default(),
        ));
        commands.entity(player).with_children(|parent| {
            parent
                .spawn(SpotLight {
                    intensity: 10_000.0 * 20.,
                    range: 40.0,
                    inner_angle: 30f32.to_radians(),
                    outer_angle: 35f32.to_radians(),
                    color: Color::srgb(1.0, 0.95, 0.8),
                    ..default()
                })
                .insert(FlashlightLight);
        });
    } else {
        let sound = player_assets.land_sounds.pick(rng).clone();
        commands.entity(player).with_child((
            SamplePlayer::new(sound),
            SpatialPool,
            Transform::default(),
        ));
    }
}
