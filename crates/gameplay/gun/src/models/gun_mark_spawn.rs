use bevy::prelude::*;
use states::{guns::marks::GunHolderMark, player::PlayerCamera};

pub(super) fn spawn_gun_holder(
    _on: On<Add, PlayerCamera>,
    player: Single<(&GlobalTransform, Entity), With<PlayerCamera>>,
    mut commands: Commands,
) {
    let (player_transform, player) = player.into_inner();

    let player_forward = player_transform.forward();

    let gun_offset = player_forward * 1.0 + Vec3::new(0.4, -0.5, 0.0);

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            GunHolderMark,
            Name::new("Gun Holder Mark"),
            Transform::from_translation(gun_offset)
                .with_scale(Vec3::splat(1.5f32))
                .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
        ));
    });
}
