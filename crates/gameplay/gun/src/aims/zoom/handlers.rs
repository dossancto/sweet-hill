use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Cancel, Ongoing};
use states::player_states::settings::WorldModelFov;

use crate::{
    aims::{aim_configurations::components::GunAiming, utils::calculate_delta_value},
    configuration::gun_components::ActiveGun,
    inputs::GunAimTrigger,
    states::GunAimState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_aiming_end);
    app.add_observer(on_aiming_ongoing);
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
    gun: Single<&GunAiming, With<ActiveGun>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GunAimState>>,
) {
    match *camera.into_inner() {
        Projection::Perspective(ref mut perspective) => {
            let target_fov = gun.zoom_level.to_radians();
            let aim_duration_ms = gun.aiming_time;

            let max_fov = PI / 2.0;
            let frame_time_ms = time.delta_secs() * 1000.0;

            let is_fully_zoomed = (perspective.fov - target_fov).abs() < 0.01;

            if is_fully_zoomed {
                return;
            }

            let delta_zoom = calculate_delta_value(
                default_fov.0.to_radians(),
                target_fov,
                aim_duration_ms,
                frame_time_ms,
            );

            let new_fov = perspective.fov + delta_zoom;

            let is_fully_zooming_in = (new_fov - target_fov).abs() < 0.01;

            if is_fully_zooming_in {
                next_state.set(GunAimState::Aiming);
                info!("Fully zoomed in");
            }

            perspective.fov = new_fov.clamp(target_fov, max_fov);
        }
        _ => (),
    }
}
