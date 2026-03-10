//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
pub mod crosshair;
pub mod flashlight;
pub mod level;
pub mod npc;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        crosshair::plugin,
        npc::plugin,
        player::plugin,
        flashlight::plugin,
        // This plugin preloads the level,
        // so make sure to add it last.
        level::plugin,
    ));
}
