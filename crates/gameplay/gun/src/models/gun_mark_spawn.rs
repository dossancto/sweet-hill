use bevy::prelude::*;
use states::{
    guns::marks::GunHolderMark, player::PlayerCamera, player_states::camera::WorldModelCamera,
};

pub(super) fn spawn_gun_holder(
    on: On<Add, WorldModelCamera>,
    q_player: Query<(&GlobalTransform, Entity), With<WorldModelCamera>>,
    mut commands: Commands,
) {
    if let Ok((player_transform, _player)) = q_player.get(on.entity) {
        let player_forward = player_transform.forward();
        let gun_offset = player_forward * 1.0 + Vec3::new(0.4, -0.5, 0.0);

        commands.entity(on.entity).with_children(|parent| {
            parent
                .spawn((
                    Name::new("Gun Holder Mark"),
                    Transform::from_translation(gun_offset), // .with_scale(Vec3::splat(1.5f32)) .with_rotation(Quat::from_rotation_y(7.5f32.to_radians())),
                ))
                .insert(GunHolderMark);
        });
    } else {
        panic!("WorldModelCamera entity not found for spawning GunHolderMark");
    }
}

pub(super) fn ensures_single_gun_holder(
    _on: On<Insert, GunHolderMark>,
    q_gun_holder: Query<Entity, With<GunHolderMark>>,
    mut commands: Commands,
) {
    for entity in q_gun_holder.iter().skip(1) {
        commands.entity(entity).despawn();
    }
}
