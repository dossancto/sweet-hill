use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use states::screens::Screen;

use crate::die_controller::{handlers::on_enemy_dies, systems::despawn_offscreen_corpses};

pub mod domain;
pub mod handlers;
pub mod systems;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_enemy_dies);

    app.add_systems(
        PostUpdate,
        (despawn_offscreen_corpses
            .run_if(in_state(Screen::Gameplay).and(on_timer(Duration::from_secs(1)))),),
    );
}
