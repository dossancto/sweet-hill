use bevy::prelude::*;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;

use crate::models::gun_models::rin::definition::{GunModelRin, GunRin};

pub(super) fn spawn_rin_model(
    _on: On<Insert, GunRin>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let my_gun_component = (
        SceneRoot(assets.load_trenchbroom_model::<GunModelRin>()),
        Transform::from_scale(Vec3::splat(0.2f32)),
    );

    commands.entity(_on.entity).insert(my_gun_component);
}
