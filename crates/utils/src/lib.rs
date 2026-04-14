use bevy::prelude::*;

pub mod asset_tracking;

pub mod animation;
pub mod asset_processing;
pub mod calculations;
pub mod hdr;
pub mod light_utils;
pub mod post_process;
pub mod shader_compilation;
pub mod ui_camera;
pub mod world;

pub mod effects;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        asset_tracking::plugin,
        asset_processing::plugin,
        effects::plugin,
        hdr::plugin,
        ui_camera::plugin,
        light_utils::plugin,
    ));
}
