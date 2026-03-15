use bevy::{app::App, state::app::AppExtStates};

use crate::states::GunState;

pub mod states;

pub mod configuration;
pub mod firing;
pub mod gun_controller;
pub mod reload;
pub mod switch_guns;

pub mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        gun_controller::plugin,
        ui::plugin,
        configuration::plugin,
        switch_guns::plugin,
        reload::plugin,
        firing::plugin,
    ));

    app.init_state::<GunState>();
}
