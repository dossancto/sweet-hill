use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputContextAppExt;
use core_gameplay::player::states::BlocksInput;

use crate::{
    configure::setup_player_input_context,
    player_input_actions_mapping::{PlayerInputContext, update_player_input_binding},
};
mod configure;
mod player_input_actions_mapping;
pub mod inputs;

pub fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInputContext>();

    app.init_resource::<BlocksInput>();
    app.add_systems(
        PreUpdate,
        update_player_input_binding.run_if(resource_changed::<BlocksInput>),
    );

    app.add_observer(setup_player_input_context);
}
