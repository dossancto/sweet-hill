use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use states::screens::Screen;

use crate::{configuration::gun_bag::GunsBag, switch_guns::switch_active_gun::switch_to_next_gun};

mod switch_active_gun;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (switch_to_next_gun.run_if(
            in_state(Screen::Gameplay)
                .and(resource_exists::<GunsBag>)
                .and(on_timer(Duration::from_secs(5))),
        ),),
    );
}
