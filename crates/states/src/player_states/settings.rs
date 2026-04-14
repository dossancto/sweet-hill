use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct WorldModelFov(pub f32);

impl Default for WorldModelFov {
    fn default() -> Self {
        Self(75.0)
    }
}

