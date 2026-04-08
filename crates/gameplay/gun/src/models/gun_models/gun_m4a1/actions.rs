use bevy::prelude::*;
use states::guns::marks::GunHolderMark;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;

use crate::models::gun_models::gun_m4a1::definition::{GunM4A1, GunModelM4A1};

pub(super) fn spawn_m4a1_model(
    _on: On<Insert, GunM4A1>,
    gun_holder_q: Single<Entity, With<GunHolderMark>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let gun_holder = gun_holder_q.into_inner();

    let my_gun_component = (
        SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
        Transform::from_scale(Vec3::splat(1.5f32)),
    );

    commands.entity(gun_holder).with_child(my_gun_component);
}
