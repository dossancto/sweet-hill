use bevy::{app::App, state::app::AppExtStates};

use states::PlayerFlashlightState;

use crate::flashlight::systems::on_toggle_flashlight;

pub(crate) mod feature;
pub(crate) mod states;
pub(crate) mod systems;

/// Registers this plugin's systems and resources with the given Bevy app.
///
/// # Arguments
///
/// * `app` - A mutable reference to the Bevy [`App`] to which this plugin will add its functionality.
///
/// # Example
///
/// ```rust
/// let mut app = App::new();
/// plugin(&mut app);
/// ```
pub(super) fn plugin(app: &mut App) {
    app.insert_state(PlayerFlashlightState::Off);

    app.add_observer(on_toggle_flashlight);
}
