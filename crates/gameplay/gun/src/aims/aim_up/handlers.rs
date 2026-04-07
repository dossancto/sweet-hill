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
}

/// Smoothly centers the gun on screen while aiming. Uses calculate_delta_value to compute per-frame
/// deltas so the transition completes in the configured aiming time.
fn center_gun_on_screen(
    _on: On<Ongoing<GunAimTrigger>>,
    gun_holder: Single<&mut Transform, With<GunHolderMark>>,
    gun: Single<&GunAiming, With<ActiveGun>>,
    time: Res<Time>,
) {
    let mut gun_transform = gun_holder.into_inner();

    // target position/rotation when aiming
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

    // translation: compute per-axis delta and apply
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

    // rotation: convert to Euler angles, compute delta on X rotation and apply
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

    gun_holder: Single<&mut Transform, With<GunHolderMark>>,
) {
    let mut gun_transform = gun_holder.into_inner();

    let player_forward = gun_transform.forward();
    let gun_offset = player_forward * 1.5 + Vec3::new(0.4, -0.5, 0.0);

    gun_transform.translation = gun_offset;

    gun_transform.rotation = Quat::from_rotation_y(7.5f32.to_radians());
}
