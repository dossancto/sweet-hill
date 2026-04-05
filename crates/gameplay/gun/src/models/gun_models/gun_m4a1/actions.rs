use bevy::{camera::visibility::RenderLayers, prelude::*};
use states::guns::marks::GunHolderMark;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;
use utils::{effects::camera_sway::SwayItem, world::RenderLayer};

use crate::{configuration::gun_components::Gun, models::gun_models::gun_m4a1::definition::{GunM4A1, GunModelM4A1}};

pub(super) fn on_add_m4a1(
    _on: On<Add, Gun>,
    // gun_holder: Single<Entity, With<GunHolderMark>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    panic!("GunModel");
    // let gun_holder = gun_holder.into_inner();
    //
    // commands.entity(gun_holder).with_children(|parent| {
    //     parent.spawn((
    //         GunModelM4A1,
    //         SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
    //         Name::new("Gun Model M4A1"),
    //         Transform::from_scale(Vec3::splat(1.5f32))
    //             .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
    //         RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::VIEW_MODEL),
    //         SwayItem {
    //             sensitivity: 0.002,
    //             smoothness: 10.0,
    //             max_sway: 0.1,
    //         },
    //     ));
    // });
}
