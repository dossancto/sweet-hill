use bevy::prelude::*;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;

use crate::models::gun_models::gun_m4a1::definition::{GunM4A1, GunModelM4A1};

pub(super) fn spawn_m4a1_model(
    _on: On<Insert, GunM4A1>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let my_gun_component = (
        SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
        Transform::from_scale(Vec3::splat(1.5f32)),
    );

    commands.entity(_on.entity).insert(my_gun_component);
}
