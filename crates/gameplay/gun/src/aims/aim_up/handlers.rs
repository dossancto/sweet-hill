use bevy::prelude::*;

use bevy_enhanced_input::prelude::{Cancel, Ongoing};
use states::guns::marks::GunHolderMark;

use crate::{
    aims::{aim_configurations::components::GunAiming, utils::calculate_delta_value},
    configuration::gun_components::ActiveGun,
    inputs::GunAimTrigger,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(center_gun_on_screen);
    app.add_observer(return_gun_to_default_position);
    app.add_systems(Update, smooth_return_system);
}

#[derive(Component)]
struct ReturningToIdle;

fn center_gun_on_screen(
    _on: On<Ongoing<GunAimTrigger>>,
    gun_holder: Single<(&mut Transform, Entity, Option<&ReturningToIdle>), With<GunHolderMark>>,
    gun: Single<&GunAiming, With<ActiveGun>>,
    time: Res<Time>,
    mut comands: Commands,
) {
    let (mut gun_transform, gun_entity, return_to_idle) = gun_holder.into_inner();

    if return_to_idle.is_some() {
        comands.entity(gun_entity).remove::<ReturningToIdle>();
    }

    let player_forward = gun_transform.forward().normalize();
    let mut target_translation = player_forward * 1. + Vec3::new(0.0, -0.3, 0.0);
    target_translation.x = -0.08f32;

    let target_rotation = Quat::from_euler(
        EulerRot::XYZ,
        -5.0f32.to_radians(),
        0.0f32.to_radians(),
        0.0f32.to_radians(),
    );

    let frame_time_ms = time.delta_secs() * 1000.0;
    let aim_duration_ms = gun.aiming_time;

    let delta_x = calculate_delta_value(
        gun_transform.translation.x,
        target_translation.x,
        aim_duration_ms,
        frame_time_ms,
    );
    let delta_y = calculate_delta_value(
        gun_transform.translation.y,
        target_translation.y,
        aim_duration_ms,
        frame_time_ms,
    );
    let delta_z = calculate_delta_value(
        gun_transform.translation.z,
        target_translation.z,
        aim_duration_ms,
        frame_time_ms,
    );

    gun_transform.translation.x += delta_x;
    gun_transform.translation.y += delta_y;
    gun_transform.translation.z += delta_z;

    let (mut cur_x, mut cur_y, mut cur_z) = gun_transform.rotation.to_euler(EulerRot::XYZ);
    let (tar_x, tar_y, tar_z) = target_rotation.to_euler(EulerRot::XYZ);

    let delta_rx = calculate_delta_value(cur_x, tar_x, aim_duration_ms, frame_time_ms);
    let delta_ry = calculate_delta_value(cur_y, tar_y, aim_duration_ms, frame_time_ms);
    let delta_rz = calculate_delta_value(cur_z, tar_z, aim_duration_ms, frame_time_ms);

    cur_x += delta_rx;
    cur_y += delta_ry;
    cur_z += delta_rz;

    gun_transform.rotation = Quat::from_euler(EulerRot::XYZ, cur_x, cur_y, cur_z);
}

fn return_gun_to_default_position(
    _on: On<Cancel<GunAimTrigger>>,
    mut commands: Commands,
    holder_q: Single<Entity, With<GunHolderMark>>,
) {
    let holder_entity = holder_q.into_inner();

    commands.entity(holder_entity).insert(ReturningToIdle);
}

fn smooth_return_system(
    mut commands: Commands,
    holders: Single<(Entity, &mut Transform), (With<GunHolderMark>, With<ReturningToIdle>)>,
    gun_q: Single<&GunAiming, With<ActiveGun>>,
    time: Res<Time>,
) {
    let (holder_entity, mut transform) = holders.into_inner();
    let aim_duration_ms = gun_q.into_inner().aiming_time * 2;

    let player_forward = transform.forward();
    let target_translation = player_forward * 1.5 + Vec3::new(0.4, -0.5, 0.0);
    let target_rotation = Quat::from_rotation_y(7.5f32.to_radians());

    let frame_time_ms = time.delta_secs() * 1000.0;

    let dx = calculate_delta_value(
        transform.translation.x,
        target_translation.x,
        aim_duration_ms,
        frame_time_ms,
    );
    let dy = calculate_delta_value(
        transform.translation.y,
        target_translation.y,
        aim_duration_ms,
        frame_time_ms,
    );
    let dz = calculate_delta_value(
        transform.translation.z,
        target_translation.z,
        aim_duration_ms,
        frame_time_ms,
    );

    transform.translation.x += dx;
    transform.translation.y += dy;
    transform.translation.z += dz;

    let (mut cx, mut cy, mut cz) = transform.rotation.to_euler(EulerRot::XYZ);
    let (tx, ty, tz) = target_rotation.to_euler(EulerRot::XYZ);
    let drx = calculate_delta_value(cx, tx, aim_duration_ms, frame_time_ms);
    let dry = calculate_delta_value(cy, ty, aim_duration_ms, frame_time_ms);
    let drz = calculate_delta_value(cz, tz, aim_duration_ms, frame_time_ms);
    cx += drx;
    cy += dry;
    cz += drz;
    transform.rotation = Quat::from_euler(EulerRot::XYZ, cx, cy, cz);

    let close_translation = (transform.translation - target_translation).length() < 0.01;
    let (cx2, cy2, cz2) = transform.rotation.to_euler(EulerRot::XYZ);
    let close_rotation =
        (cx2 - tx).abs() < 0.01 && (cy2 - ty).abs() < 0.01 && (cz2 - tz).abs() < 0.01;

    if close_translation && close_rotation {
        commands.entity(holder_entity).remove::<ReturningToIdle>();
    }
}
