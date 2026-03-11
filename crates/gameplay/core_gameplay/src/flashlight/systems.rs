use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::player::camera::PlayerCamera;

use super::states::{FlashlightLight, PlayerFlashlightState, ToggleFlashlight};

pub(crate) fn on_toggle_flashlight(
    _on: On<Start<ToggleFlashlight>>,
    state: Res<State<PlayerFlashlightState>>,
    player_camera: Single<Entity, With<PlayerCamera>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayerFlashlightState>>,
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

    let flashlight_children_to_remove = flashlight.into_iter();

    for (entity, _child) in flashlight_children_to_remove {
        commands.entity(entity).despawn();
    }

    if new_state == PlayerFlashlightState::On {
        // TODO: Send event to play flashlight toggle sound
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
        // TODO: Send event to play flashlight toggle sound
    }
}

#[cfg(test)]
mod tests {
    use bevy::{input::InputPlugin, state::app::StatesPlugin};

    use crate::flashlight;

    use super::*;

    #[test]
    fn did_toggle_flashlight_from_of_to_on() {
        const KEY: KeyCode = KeyCode::KeyF;

        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            InputPlugin,
            EnhancedInputPlugin,
            StatesPlugin,
        ))
        .add_plugins(flashlight::plugin)
        .add_input_context::<TestContext>()
        .finish();

        app.world_mut().spawn((
            TestContext,
            PlayerCamera,
            actions!(
                TestContext[
                (
                    Action::<ToggleFlashlight>::new(),
                    ActionSettings {
                        consume_input: true,
                        ..Default::default()
                    },
                    bindings![KEY],
                ),
                ]
            ),
        ));

        app.update();

        let flash_light_state = app.world().resource::<State<PlayerFlashlightState>>().get();
        assert_eq!(flash_light_state, &PlayerFlashlightState::Off);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KEY);

        app.update();

        let updated_flash_light_state =
            app.world().resource::<State<PlayerFlashlightState>>().get();
        assert_eq!(updated_flash_light_state, &PlayerFlashlightState::On);
    }

    #[derive(Component, Clone, Copy)]
    struct TestContext;
}
