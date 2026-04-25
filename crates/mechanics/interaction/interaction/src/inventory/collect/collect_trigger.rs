use bevy::prelude::*;

use crate::inventory::collect::{
    collect_action::Collect,
    collectable::{CanBeCollect, Collectable},
};

/// Event signaling that a collect action should be triggered for a specific entity.
///
/// This event is typically sent when an entity (such as a player or agent) interacts with a collectible object in the game world.
/// Systems listening for this event should handle the logic for collecting or processing the specified entity.
#[derive(Event)]
pub struct TriggerCollect {
    /// * `entity` - The [`Entity`] to be collected. Must be a valid entity reference; passing an invalid or despawned entity may result in no action or errors in downstream systems.
    pub entity: Entity,
}

/// Attempts to collect a specific entity that implements the `CanBeCollect` trait when a `TriggerCollect` event occurs.
///
/// This function is intended to be used within an ECS (Entity Component System) context where entities can be marked as collectable.
/// When triggered, it checks if the specified entity is collectable and, if so, issues a `Collect` event for that entity.
///
/// # Type Parameters
/// * `T`: A component type that must implement the `CanBeCollect` trait. Represents the kind of collectable item.
///
/// # Parameters
/// * `on`: An `On<TriggerCollect>` event containing the entity to attempt to collect. The `entity` field specifies which entity is being targeted for collection.
/// * `commands`: A mutable `Commands` object used to trigger new events in the ECS system.
/// * `query`: A `Query` that retrieves references to components of type `T` for entities that also have the `Collectable` marker component.
///
/// # Behavior
/// If the targeted entity does not have the required `T` component with the `Collectable` marker, the function returns immediately and no event is triggered.
/// If the entity is collectable, a `Collect` event is created and triggered for that entity.
///
/// # Edge Cases
/// * If the entity specified in `on.entity` does not exist or does not match the query, no action is taken and no error is raised.
pub(crate) fn collect_item<T: CanBeCollect>(
    on: On<TriggerCollect>,
    mut commands: Commands,
    query: Query<&T, With<Collectable>>,
) {
    let entity = on.entity;

    let Ok(collectable) = query.get(entity) else {
        return;
    };

    let event_to_trigger = Collect::new(collectable.clone(), entity);

    commands.trigger(event_to_trigger);
}


