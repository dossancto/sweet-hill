use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use states::screens::Screen;

use crate::gun_controller::{
    configure::GunsBag,
    handle_input::handle_tap_fire,
    systems::switch_to_next_gun,
    ui::{draw_ammo_on_screen, init_ammo_text},
};

mod configure;
mod domain;
mod handle_input;
mod manage_guns;
mod systems;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), init_ammo_text);

    app.add_systems(
        Update,
        (
            switch_to_next_gun.run_if(
                in_state(Screen::Gameplay)
                    .and(resource_exists::<GunsBag>)
                    .and(on_timer(Duration::from_secs(5))),
            ),
            draw_ammo_on_screen
                .run_if(in_state(Screen::Gameplay))
                .after(switch_to_next_gun),
        ),
    );

    app.init_resource::<GunsBag>();

    app.add_observer(handle_tap_fire);
}
