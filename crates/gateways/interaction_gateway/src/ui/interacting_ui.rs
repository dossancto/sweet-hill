use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Cancel, Ongoing};
use gameplay_input::inputs::InteractAction;
use interaction::interaction::components::{CanInteract, Interactable, Interacting};
use states::interaction::InteractionSourceCamera;

use crate::interact::InteractionLocked;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_interacting_ui);
    app.add_observer(clear_interacting_ui_text);

    app.add_observer(hide_interacting_ui_text_on_cancel);
    app.add_observer(hide_interacting_ui_text_on_cant_interact);
    app.add_observer(load_interacting_items);
}

#[derive(Component)]
struct PercentageText;

fn setup_interacting_ui(_on: On<Add, InteractionSourceCamera>, mut commands: Commands) {
    commands.spawn((
        Name::new("Percentage"),
        Text::new(""),
        Visibility::Hidden,
        PercentageText,
        Node {
            position_type: PositionType::Absolute,
            bottom: px(36),
            left: px(12),
            ..default()
        },
    ));
}

fn hide_interacting_ui_text_on_cancel(
    _on: On<Cancel<InteractAction>>,
    mut percentage_text_q: Query<&mut Visibility, With<PercentageText>>,
) {
    for mut visibility in percentage_text_q.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn hide_interacting_ui_text_on_cant_interact(
    _on: On<Remove, CanInteract>,
    mut percentage_text_q: Query<&mut Visibility, With<PercentageText>>,
) {
    for mut visibility in percentage_text_q.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn clear_interacting_ui_text(
    _on: On<Remove, InteractionSourceCamera>,
    interacting_ui_q: Query<Entity, With<PercentageText>>,
    mut commands: Commands,
) {
    for visibility in interacting_ui_q.iter() {
        commands.entity(visibility).despawn();
    }
}

fn load_interacting_items(
    _on: On<Ongoing<InteractAction>>,
    interactables_q: Single<
        (&Interactable, &Interacting),
        (Without<InteractionLocked>, With<CanInteract>),
    >,
    percentage_text_q: Single<(&mut Text, &mut Visibility), With<PercentageText>>,
) {
    let (interactable, interacting) = interactables_q.into_inner();

    let total_time = interactable.time_to_interact.as_secs_f32();

    let elapsed_time = interacting.time_to_interact.remaining_secs();

    let resulting_percentage = 100.0 - (elapsed_time / total_time * 100.0);

    let (mut percentage_text, mut visibility) = percentage_text_q.into_inner();

    if interactable.time_to_interact.is_zero() {
        *visibility = Visibility::Hidden;
        return;
    }

    percentage_text.0 = format!("{resulting_percentage:.0}%");
    *visibility = Visibility::Visible;
}
