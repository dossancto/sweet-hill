use bevy::{app::App, state::app::AppExtStates};

use crate::states::GunState;

pub mod states;

pub mod firing;
pub(crate) mod gun_controller;

pub mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((gun_controller::plugin, ui::plugin));

    app.init_state::<GunState>();
}
