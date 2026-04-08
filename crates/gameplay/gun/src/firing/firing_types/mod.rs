use bevy::prelude::*;

pub(crate) mod bullet;
pub(crate) mod domain;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bullet::plugin,));
}
