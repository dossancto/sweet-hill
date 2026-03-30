use bevy::{app::App, state::app::AppExtStates};

use crate::states::{GunAimState, GunState};

pub mod states;

pub mod configuration;
pub mod firing;
pub mod gun_controller;
pub mod inputs;
pub mod models;
pub mod reload;
pub mod switch_guns;
pub mod recoil;

pub(crate) mod sound;
pub(crate) mod ui;
pub(crate) mod aims;
pub(crate) mod assets;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        gun_controller::plugin,
        ui::plugin,
        configuration::plugin,
        switch_guns::plugin,
        reload::plugin,
        firing::plugin,
        models::plugin,
        sound::plugin,
        aims::plugin,
        recoil::plugin,
    ));

    app.init_state::<GunState>();
    app.init_state::<GunAimState>();
}
