use bevy::prelude::*;
use states::inventory::active_item::ActiveItem;

/// Registers the `assert_a_single_active_item` observer with the application.
///
/// This function is intended to be called during application setup to ensure that the observer
/// responsible for validating that only one item is active at a time is added to the application's
/// observer list.
///
/// # Parameters
/// - `app`: A mutable reference to the [`App`] instance where the observer will be registered. Must not be `null`.
///
/// # Panics
/// Panics if `app` is in an invalid state or if adding the observer fails due to internal constraints.
///
/// # Example
/// ```rust
/// let mut app = App::new();
/// plugin(&mut app);
/// ```
pub(super) fn plugin(app: &mut App) {
    // app.add_observer(assert_a_single_active_item);
}

/// Ensures that only one entity in the world retains the `ActiveItem` component at any given time.
/// 
/// This function is typically used in ECS (Entity Component System) contexts where the `ActiveItem` component
/// should be unique among all entities. When an entity receives the `ActiveItem` component (via an insert event),
/// this function removes the `ActiveItem` component from all other entities, enforcing the invariant that only one
/// entity can be active at a time.
///
/// # Behavior
/// - The entity referenced by `on.entity` retains its `ActiveItem` component.
/// - All other entities with the `ActiveItem` component will have it removed.
///
/// # Edge Cases & Exceptions
/// - If `on.entity` is not present in the query, no error is thrown; the function simply removes `ActiveItem` from all entities except `on.entity`.
/// - If only one entity has the `ActiveItem` component, no changes are made.
/// - If `query` is empty, the function has no effect.
fn assert_a_single_active_item(
    on: On<Add, ActiveItem>,
    query: Query<(&ActiveItem, Entity)>,
    mut commands: Commands,
) {
    let entity_to_keep = on.entity;

    for (_, active_item) in query.iter() {
        if active_item == entity_to_keep {
            continue;
        }

        commands.entity(active_item).remove::<ActiveItem>();
    }
}
