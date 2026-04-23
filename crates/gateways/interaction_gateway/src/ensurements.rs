use bevy::prelude::*;
use interaction::interaction::components::CanInteract;

use crate::interact::InteractionLocked;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(remove_can_interact_when_remove_interaction);
    app.add_observer(remove_locked_when_remove_interaction);
}

fn remove_can_interact_when_remove_interaction(
    on: On<Remove, Interaction>,
    mut commands: Commands,
) {
    commands.entity(on.entity).remove::<CanInteract>();
}

fn remove_locked_when_remove_interaction(on: On<Remove, Interaction>, mut commands: Commands) {
    commands.entity(on.entity).remove::<InteractionLocked>();
}
