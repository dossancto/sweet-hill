use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

#[point_class(base(Transform, Visibility), model("models/guns/rin/rin.gltf"))]
pub struct GunModelRin;

#[derive(Component, Debug, Reflect)]
pub struct GunRin;
