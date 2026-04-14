use bevy::prelude::*;
use states::{guns::marks::GunHolderMark, player_states::camera::WorldModelCamera};

pub(super) fn spawn_gun_holder(
    on: On<Add, WorldModelCamera>,
    q_player: Single<&Transform, With<WorldModelCamera>>,
    mut commands: Commands,
) {
    let player_transform = q_player.into_inner();

    let player_forward = player_transform.forward();
    let gun_offset = player_forward * 1.5 + Vec3::new(0.4, -0.5, 0.0);

    commands.entity(on.entity).with_children(|parent| {
        parent
            .spawn((
                Name::new("Gun Holder Mark"),
                Transform::from_translation(gun_offset),
                Visibility::Visible,
            ))
            .insert(GunHolderMark);
    });
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
