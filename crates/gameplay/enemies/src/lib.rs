use bevy::prelude::*;

mod hit_detection;
pub mod states;

pub fn plugin(app: &mut App) {
    app.add_plugins((hit_detection::plugin,));
}
