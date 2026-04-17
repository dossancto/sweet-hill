use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;
use gameplay_input::inputs::ToogleActiveItem;
use states::inventory::{active_item::ActiveItem, items::Item};

/// Registers gun-related event observers with the provided application instance.
///
/// This function attaches the following observers to the application's event system:
/// - `switch_to_next_gun`: Handles logic for switching to the next available gun.
/// - `hide_gun`: Responds to events that require the gun to be hidden from view.
/// - `show_gun`: Responds to events that require the gun to be displayed.
pub(super) fn plugin(app: &mut App) {
    app.add_observer(switch_to_next_item);
    app.add_observer(hide_gun);
    app.add_observer(show_gun);
}

/// Switches the currently active gun to the next available pickable gun entity.
///
/// This function is typically invoked in response to a user action or game event that requests toggling the active weapon.
/// It checks whether the currently active gun is eligible for switching, and if so, deactivates it and activates the next available pickable gun.
///
/// # Behavior
/// - If the currently active gun does not allow switching (as determined by `ActiveItem::can_switch()`), the function logs a message and takes no further action.
/// - If there is at least one pickable gun available, the function removes the `ActiveItem` component from the current gun and assigns it to the next available pickable gun entity.
///
/// # Edge Cases & Errors
/// - If no pickable gun is available, the function does nothing and no error is raised.
/// - If the `ActiveItem` component's `can_switch()` method returns `false`, switching is prevented and a log message is generated.
/// - If `active_gun_query` does not return exactly one entity, this will panic at runtime due to the use of `into_inner()`.
fn switch_to_next_item(
    _on: On<Start<ToogleActiveItem>>,
    guns_q: Query<Entity, (Without<ActiveItem>, With<Item>)>,
    active_gun_query: Single<(Entity, &ActiveItem)>,
    mut commands: Commands,
) {
    let (active_gun_entity, active_item) = active_gun_query.into_inner();

    if active_item.can_switch() == false {
        info!("Cannot switch active gun right now.");
        return;
    }

    let next_gun_entity = guns_q.iter().next().map(|entity| entity);

    let Some(next_gun_entity) = next_gun_entity else {
        return;
    };

    info!(
        "Switching active gun from {:?} to {:?}.",
        active_gun_entity, next_gun_entity
    );

    commands.entity(active_gun_entity).remove::<ActiveItem>();

    commands
        .entity(next_gun_entity)
        .insert(ActiveItem::default());
}

/// Hides the gun entity by setting its visibility to hidden when it becomes inactive.
///
/// This function is intended to be called when an `ActiveItem` component is removed from an entity,
/// indicating that the gun is no longer active and should not be visible in the game world.
///
/// # Behavior
/// If the entity referenced by `on` does not currently have a `Visibility` component, one is inserted
/// and set to [`Visibility::Hidden`]. If the entity's visibility is already hidden, no further action is taken.
/// Otherwise, the entity's visibility is set to hidden.
///
/// # Edge Cases
/// - If the entity does not exist or has been despawned, the function will attempt to insert a `Visibility::Hidden`
///   component, which may have no effect if the entity is invalid.
/// - No error is returned or logged if the entity is missing or the query fails; the function simply returns early.
///
/// # Panics
/// This function does not panic under normal circumstances.
fn hide_gun(
    on: On<Remove, ActiveItem>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now inactive, hiding it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Hidden);
        return;
    };

    if *visibility == Visibility::Hidden {
        return;
    }

    *visibility = Visibility::Hidden;

    return;
}

/// Makes the specified gun entity visible in the game world.
///
/// This function is intended to be called when a gun becomes active (e.g., equipped or selected by the player).
/// It ensures that the gun's entity is visible by updating its `Visibility` component. If the entity does not
/// already have a `Visibility` component, one is inserted and set to `Visible`.
///
/// # Behavior
/// - If the entity referenced by `on.entity` does not have a `Visibility` component, one is added and set to `Visible`.
/// - If the entity's `Visibility` is already `Visible`, no further action is taken.
/// - Otherwise, the entity's `Visibility` is set to `Visible`.
///
/// # Edge Cases & Errors
/// - If `on.entity` is invalid or has been despawned, the function will attempt to insert a `Visibility` component, which may have no effect or cause a warning depending on the ECS implementation.
/// - No error is returned or propagated; the function silently returns if the entity cannot be found in the query.
fn show_gun(
    on: On<Add, ActiveItem>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now actieve, showing it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Visible);
        return;
    };

    if *visibility == Visibility::Visible {
        return;
    }

    *visibility = Visibility::Visible;

    return;
}
