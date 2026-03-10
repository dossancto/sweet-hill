//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;
use states::screens::LoadingScreen;

mod preload_assets;
mod shader_compilation;
mod spawn_level;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<LoadingScreen>();
    app.add_plugins((
        shader_compilation::plugin,
        preload_assets::plugin,
        spawn_level::plugin,
    ));
}
