use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct WorldModelCamera;
