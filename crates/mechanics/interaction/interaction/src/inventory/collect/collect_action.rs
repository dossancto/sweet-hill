use bevy::prelude::*;

/// Represents an event indicating that an entity has collected a value of type `E`.
///
/// This event is typically used in scenarios where an entity (such as a player or agent)
/// collects, acquires, or picks up a resource, item, or reward represented by `E`.
/// The event carries both the entity that performed the collection and the value collected.
///
/// # Type Parameters
/// * `E` - The type of value being collected (e.g., a resource, item, or currency).
#[derive(EntityEvent, Debug)]
pub struct Collect<E> {
    /// The entity that performed the collection action.
    pub entity: Entity,
    /// The value or item that was collected by the entity.
    pub value: E,
}

impl<E> Collect<E> {
    /// Creates a new `Collect` event for the specified entity and value.
    ///
    /// # Parameters
    /// * `event` - The value or item that was collected. Must be of type `E`.
    /// * `entity` - The entity that performed the collection. Must be a valid [`Entity`].
    ///
    /// # Returns
    /// A new `Collect<E>` event containing the provided entity and value.
    ///
    /// # Panics
    /// This function does not panic.
    pub(crate) fn new(event: E, entity: Entity) -> Self {
        Collect {
            value: event,
            entity,
        }
    }
}

