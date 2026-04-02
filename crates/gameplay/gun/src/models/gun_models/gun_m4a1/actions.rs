use bevy::prelude::*;
use states::guns::marks::GunHolderMark;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;
use utils::effects::camera_sway::SwayItem;

use crate::models::gun_models::gun_m4a1::definition::GunModelM4A1;

pub(super) fn on_add_m4a1(
    _on: On<Add, GunHolderMark>,
    gun_holder: Single<Entity, With<GunHolderMark>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let gun_holder = gun_holder.into_inner();

    commands.entity(gun_holder).with_children(|parent| {
        parent.spawn((
            GunModelM4A1,
            SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
            Name::new("Gun Model M4A1"),
            SwayItem {
                sensitivity: 0.002,
                smoothness: 10.0,
                max_sway: 0.1,
            },
        ));
    });
}
