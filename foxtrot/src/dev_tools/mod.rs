//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::prelude::PhysicsDebugPlugin;
use bevy::{dev_tools::{picking_debug::DebugPickingPlugin, states::log_transitions}, prelude::*};
use states::screens::LoadingScreen;

mod debug_ui;
mod input;
pub(crate) mod log_components;
mod validate_preloading;

use crate::menus::Menu;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(
        Update,
        (log_transitions::<Menu>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((
        debug_ui::plugin,
        input::plugin,
        DebugPickingPlugin::default(),
        validate_preloading::plugin,
        log_components::plugin,
    ));
}
