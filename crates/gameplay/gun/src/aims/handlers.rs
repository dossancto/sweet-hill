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
    camera: Single<&mut Projection, (With<Camera>, With<ChildOf>)>,
) {
    match *camera.into_inner() {
        Projection::Perspective(ref mut perspective) => {
            let delta_zoom = -0.05;

            // TODO: Add a Gun Component to tweak the gun zoom level
            perspective.fov = (perspective.fov + delta_zoom).clamp(60f32.to_radians(), PI / 2.0);
        }
        _ => (),
    }
}
