use bevy::app::App;

pub(crate) mod gun_controller;
pub(crate) mod guns_features;

pub fn plugin(app: &mut App) {
    app.add_plugins(gun_controller::plugin);
}
