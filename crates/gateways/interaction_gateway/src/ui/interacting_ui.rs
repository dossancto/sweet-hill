use bevy::{color::palettes::tailwind, prelude::*};
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

#[derive(Component)]
struct ProgressBar;

fn setup_interacting_ui(_on: On<Add, InteractionSourceCamera>, mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(300.0),
            height: Val::Px(10.0),
            bottom: Val::Px(0.),
            left: Val::Percent(50.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(2.0),
            ..default()
        },
        PercentageText,
        Visibility::Hidden,
        children![(
            Node {
                width: Val::Percent(20.),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            Visibility::Inherited,
            ProgressBar,
        )],
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
    mut percentage_text_q: Query<&mut Node, With<ProgressBar>>,
    mut root_percentage_text_q: Query<&mut Visibility, With<PercentageText>>,
) {
    let (interactable, interacting) = interactables_q.into_inner();

    let total_time = interactable.time_to_interact.as_secs_f32();

    let elapsed_time = interacting.time_to_interact.remaining_secs();

    let resulting_percentage = 100.0 - (elapsed_time / total_time * 100.0);

    for mut visibility in root_percentage_text_q.iter_mut() {
        *visibility = Visibility::Visible;
    }

    for mut percentage_node in percentage_text_q.iter_mut() {
        percentage_node.width = Val::Percent(resulting_percentage);
    }
}
