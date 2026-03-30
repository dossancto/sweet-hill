use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Cancel, Ongoing, Start};
use states::{player::PlayerCamera, player_states::settings::WorldModelFov};

use crate::{inputs::GunAimTrigger, states::GunAimState};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_aiming_start);
    app.add_observer(on_aiming_end);
    app.add_observer(on_aiming_ongoing);
}

fn on_aiming_start(_on: On<Start<GunAimTrigger>>, mut next_state: ResMut<NextState<GunAimState>>) {
    next_state.set(GunAimState::Aiming);
    info!("Start aiming");
}

fn on_aiming_end(
    _on: On<Cancel<GunAimTrigger>>,
    mut next_state: ResMut<NextState<GunAimState>>,
    camera: Single<&mut Projection, (With<Camera>, With<ChildOf>)>,
    defualt_fov: Res<WorldModelFov>,
) {
    next_state.set(GunAimState::Idle);

    match *camera.into_inner() {
        Projection::Perspective(ref mut perspective) => {
            perspective.fov = defualt_fov.0.to_radians();
        }
        _ => (),
    }

    info!("Stopped aiming");
}

fn on_aiming_ongoing(
    _on: On<Ongoing<GunAimTrigger>>,
    default_fov: Res<WorldModelFov>,
    camera: Single<&mut Projection, (With<Camera>, With<ChildOf>)>,
    time: Res<Time>,
) {
    match *camera.into_inner() {
        Projection::Perspective(ref mut perspective) => {
            // TODO: Add a Gun Component to tweak the gun zoom level, and time to zoom in
            let target_fov = 60f32.to_radians();
            let aim_duration_ms = 50u32;

            let max_fov = PI / 2.0;
            let frame_time_ms = time.delta_secs() * 1000.0;

            let delta_zoom = calculate_delta_zoom(
                default_fov.0.to_radians(),
                target_fov,
                aim_duration_ms,
                frame_time_ms,
            );

            perspective.fov = (perspective.fov + delta_zoom).clamp(target_fov, max_fov);
        }
        _ => (),
    }
}

/// Calculates the required delta_zoom per frame to reach the target_fov in the given duration (in milliseconds).
///
/// # Arguments
/// * `default_fov` - The starting field of view (in radians)
/// * `target_fov` - The desired field of view to reach (in radians)
/// * `duration_ms` - The duration over which to reach the target_fov (in milliseconds)
/// * `frame_time_ms` - The time per frame (in milliseconds)
///
/// # Returns
/// The delta_zoom value to apply per frame.
fn calculate_delta_zoom(
    default_fov: f32,
    target_fov: f32,
    duration_ms: u32,
    frame_time_ms: f32,
) -> f32 {
    let total_frames = (duration_ms as f32 / frame_time_ms).max(1.0);
    (target_fov - default_fov) / total_frames
}
