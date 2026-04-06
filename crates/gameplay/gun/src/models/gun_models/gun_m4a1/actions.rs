use bevy::{camera::visibility::RenderLayers, prelude::*};
use states::guns::marks::GunHolderMark;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;
use utils::{effects::camera_sway::SwayItem, world::RenderLayer};

use crate::{
    configuration::gun_components::Gun,
    models::gun_models::gun_m4a1::definition::{GunM4A1, GunModelM4A1},
};

pub(super) fn on_add_m4a1(
    _on: On<Insert, Gun>,
    gun_holder: Query<Entity, With<GunHolderMark>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    for holder in gun_holder.iter() {
        commands.entity(holder).with_children(|parent| {
            parent.spawn((
                GunModelM4A1,
                SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
                Name::new("Gun Model M4A1"),
                Transform::from_scale(Vec3::splat(1.5f32))
                    .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
                SwayItem {
                    sensitivity: 0.002,
                    smoothness: 10.0,
                    max_sway: 0.1,
                },
            ));
        });
    }
    // if let Some(gun_holder) = gun_holder.iter().next() {
    //     commands.entity(gun_holder).with_children(|parent| {
    //         parent.spawn((
    //             GunModelM4A1,
    //             SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
    //             Name::new("Gun Model M4A1"),
    //             Transform::from_scale(Vec3::splat(1.5f32))
    //                 .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
    //             SwayItem {
    //                 sensitivity: 0.002,
    //                 smoothness: 10.0,
    //                 max_sway: 0.1,
    //             },
    //         ));
    //     });
    // }
}
