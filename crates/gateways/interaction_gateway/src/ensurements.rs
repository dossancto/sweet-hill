use bevy::prelude::*;
use interaction::interaction::components::CanInteract;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(remove_can_interact_when_remove_interaction);
}

fn remove_can_interact_when_remove_interaction(
    on: On<Remove, Interaction>,
    mut commands: Commands,
) {
    commands.entity(on.entity).remove::<CanInteract>();
}
