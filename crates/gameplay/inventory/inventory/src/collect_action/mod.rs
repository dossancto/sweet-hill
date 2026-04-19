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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, Clone)]
    pub struct Money {
        pub amount: f64,
    }

    #[derive(Resource, Default)]
    struct PlayerWallet(f64);

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<PlayerWallet>();

        app.add_observer(|on: On<Collect<Money>>, mut wallet: ResMut<PlayerWallet>| {
            wallet.0 = wallet.0 + on.value.amount;
        });

        let money = Money { amount: 10.0 };

        let spawned_money = app.world_mut().spawn(money.clone()).id();

        let event_to_trigger = Collect::new(money, spawned_money.clone());

        app.world_mut().trigger(event_to_trigger);

        app.update();

        let wallet = app.world().resource::<PlayerWallet>();

        assert_eq!(wallet.0, 10.0);
    }
}

