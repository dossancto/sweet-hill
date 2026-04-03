use bevy::prelude::*;
use states::guns::marks::GunHolderMark;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;
use utils::effects::camera_sway::SwayItem;

use crate::models::gun_models::gun_m4a1::definition::GunModelM4A1;

pub(super) fn on_add_m4a1(
    on: On<Add, GunHolderMark>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.entity(on.entity).with_children(|parent| {
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
