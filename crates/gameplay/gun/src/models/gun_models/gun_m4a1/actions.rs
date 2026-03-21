use bevy::prelude::*;
use states::player::PlayerCamera;
use third_party::bevy_trenchbroom::LoadTrenchbroomModel;

use crate::models::gun_models::gun_m4a1::definition::GunModelM4A1;

pub(super) fn on_add_m4a1(
    _on: On<Add, PlayerCamera>,
    player: Single<(&GlobalTransform, Entity), With<PlayerCamera>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let (player_transform, player) = player.into_inner();

    let player_forward = player_transform.forward();

    // Position the gun relative to the player's forward direction
    let gun_offset = player_forward * 1.0 + Vec3::new(0.4, -0.5, 0.0);

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            GunModelM4A1,
            SceneRoot(assets.load_trenchbroom_model::<GunModelM4A1>()),
            Name::new("Gun Model M4A1"),
            // Position the gun at the player's hand, using forward direction
            Transform::from_translation(gun_offset)
                .with_scale(Vec3::splat(1.5f32))
                .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
        ));
    });
}
