use bevy::prelude::*;
use states::screens::Screen;

use crate::ui::ammo_ui::{draw_ammo_on_screen, init_ammo_text};

mod ammo_ui;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), init_ammo_text);

    app.add_systems(
        Update,
        (draw_ammo_on_screen
            .run_if(in_state(Screen::Gameplay))
            .after(init_ammo_text),),
    );
}
