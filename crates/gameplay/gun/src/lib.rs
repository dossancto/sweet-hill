use ::states::player::Player;
use bevy::prelude::*;
use bevy::{app::App, state::app::AppExtStates};

use crate::replace_guns::system::grep_gun;
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
pub mod switch_guns;

pub(crate) mod aims;
pub(crate) mod assets;
pub(crate) mod sound;
pub(crate) mod ui;

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
        replace_guns::plugin,
    ));

    app.init_state::<GunState>();
    app.init_state::<GunAimState>();

    app.add_observer(add_default_gun);
}

// TODO: Remove from here
fn add_default_gun(_add: On<Add, Player>, mut commands: Commands) {
    commands.run_system_cached(grep_gun);
}
