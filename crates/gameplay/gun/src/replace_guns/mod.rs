use bevy::prelude::*;

pub mod system;

/// A plugin that handles gun replacement logic in the game. This plugin is responsible for
/// replacing guns with their respective replacements based on the player's current loadout and the
/// available replacements.
///
/// # Arguments
///
/// * `app` - A mutable reference to the Bevy `App` to which the plugin will be added.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(system::plugin);
}
