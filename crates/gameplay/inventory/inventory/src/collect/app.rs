use bevy::app::App;

use crate::collect::{collect_trigger::collect_item, collectable::CanBeCollect};

/// Provides the ability to register a type as a collectable item within an application.
///
pub trait AddCollectable {
    /// Registers a type `T` as a collectable item for the implementing object.
    ///
    /// # Type Parameters
    /// * `T`: The type to be registered as collectable. Must implement [`CanBeCollect`].
    ///
    /// # Returns
    /// A mutable reference to the implementor, allowing for method chaining.
    ///
    /// # Usage
    /// Use this method to extend the set of types that the application can observe and collect.
    ///
    /// # Errors
    /// This method does not return errors directly, but the underlying implementation may panic if the observer registration fails or if `self` is in an invalid state.
    fn add_collectable<T>(&mut self) -> &mut Self
    where
        T: CanBeCollect;
}

impl AddCollectable for App {
    /// Registers a type `T` as a collectable item for the application instance.
    ///
    /// # Type Parameters
    /// * `T`: The type to be registered as collectable. Must implement [`CanBeCollect`].
    ///
    /// # Returns
    /// A mutable reference to the `App` instance, enabling method chaining.
    ///
    /// # Panics
    /// Panics if observer registration fails or if the application is in an invalid state for modification.
    fn add_collectable<T>(&mut self) -> &mut Self
    where
        T: CanBeCollect,
    {
        self.add_observer(collect_item::<T>);
        self
    }
}
