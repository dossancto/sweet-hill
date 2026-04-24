use bevy::prelude::*;

use crate::states::{GunAimState, GunState};

pub mod states;

pub mod configuration;
pub mod firing;
pub mod gun_controller;
pub mod inputs;
pub mod models;
pub mod recoil;
pub mod reload;
pub mod replace_guns;

pub(crate) mod aims;
pub(crate) mod assets;
pub(crate) mod sound;
pub(crate) mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        gun_controller::plugin,
        ui::plugin,
        configuration::plugin,
        reload::plugin,
        firing::plugin,
        models::plugin,
        sound::plugin,
        aims::plugin,
        recoil::plugin,
        replace_guns::plugin,
    ));

    app.init_state::<GunState>();
    app.init_state::<GunAimState>();
}
