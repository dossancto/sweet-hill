use bevy::prelude::*;

/// The parent entity of the player's cameras.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct PlayerCamera;
