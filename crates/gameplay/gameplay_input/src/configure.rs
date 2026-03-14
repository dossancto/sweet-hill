use bevy::prelude::*;
use core_gameplay::player::Player;

use crate::player_input_actions_mapping::PlayerInputContext;

pub(super) fn setup_player_input_context(add: On<Add, Player>, mut commands: Commands) {
    commands.entity(add.entity).insert(PlayerInputContext);
}
