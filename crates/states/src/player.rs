use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

/// The parent entity of the player's cameras.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct PlayerCamera;

#[point_class(
    base(Transform, Visibility),
    model("models/view_model/view_model.gltf")
)]
pub struct Player;
