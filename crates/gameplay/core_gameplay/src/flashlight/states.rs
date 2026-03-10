use bevy::{ecs::component::Component, state::state::States};
use bevy_enhanced_input::prelude::InputAction;

/// Represents the possible states of the player's flashlight.
///
/// This enum is used as a Bevy state resource to track the flashlight's status.
///
/// Variants:
/// - `Disabled`: The flashlight is not available or cannot be used.
/// - `Off`: The flashlight is available but currently turned off.
/// - `On`: The flashlight is currently turned on.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerFlashlightState {
    #[default]
    Disabled,
    Off,
    On,
}

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct ToggleFlashlight;

#[derive(Component)]
pub(crate) struct FlashlightLight;
