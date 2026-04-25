use bevy::prelude::*;
use interaction::interaction::components::CanInteract;
use states::interaction::InteractionSourceCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_can_interact_ui);
    app.add_observer(clear_can_interact_text);

    app.add_observer(show_can_interact_text);
    app.add_observer(hide_can_interact_text);
}

#[derive(Component)]
struct CanInteractText;

fn setup_can_interact_ui(_on: On<Add, InteractionSourceCamera>, mut commands: Commands) {
    commands.spawn((
        Name::new("Can interact text"),
        Text::new("Press E to interact"),
        CanInteractText,
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn clear_can_interact_text(
    _on: On<Remove, InteractionSourceCamera>,
    can_interact_q: Query<Entity, With<CanInteractText>>,
    mut commands: Commands,
) {
    for visibility in can_interact_q.iter() {
        commands.entity(visibility).despawn();
    }
}

fn show_can_interact_text(
    _on: On<Add, CanInteract>,
    mut can_interact_q: Query<&mut Visibility, With<CanInteractText>>,
) {
    for mut visibility in can_interact_q.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn hide_can_interact_text(
    _on: On<Remove, CanInteract>,
    mut can_interact_q: Query<&mut Visibility, With<CanInteractText>>,
) {
    for mut visibility in can_interact_q.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

