use bevy::prelude::*;

use crate::{configuration::gun_components::ActiveGun, replace_guns::events::TakeGunEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_gun_replacement);
}

pub fn handle_gun_replacement(on: On<TakeGunEvent>, mut commands: Commands) {
    let spawned_gun = on.gun_to_spawn.spawn(&mut commands);

    commands.entity(spawned_gun).insert(ActiveGun);
}
