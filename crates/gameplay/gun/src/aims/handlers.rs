use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Cancel, Complete, Start};

use crate::{inputs::GunAimTrigger, states::GunAimState};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_aiming_start);
    app.add_observer(on_aiming_end);
}

fn on_aiming_start(_on: On<Start<GunAimTrigger>>, mut next_state: ResMut<NextState<GunAimState>>) {
    next_state.set(GunAimState::Aiming);
    info!("Start aiming");
}

fn on_aiming_end(_on: On<Cancel<GunAimTrigger>>, mut next_state: ResMut<NextState<GunAimState>>) {
    next_state.set(GunAimState::Idle);

    info!("Stopped aiming");
}
