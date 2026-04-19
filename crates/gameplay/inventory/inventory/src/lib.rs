use bevy::prelude::*;

pub mod collect;

pub fn plugin(app: &mut App) {
    app.add_plugins((collect::plugin,));
}
