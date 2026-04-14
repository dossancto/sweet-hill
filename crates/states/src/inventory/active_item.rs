use bevy::prelude::*;

/// Marker component indicating that an entity is currently the active item.
///
/// Attach this component to an entity to designate it as the active item within the game or system.
/// Systems can query for entities with `ActiveItem` to perform logic specific to the currently active item.
///
/// # Edge Cases
/// - Removing this component from an entity will cause it to no longer be recognized as active.
#[derive(Component)]
pub struct ActiveItem {
    pub can_be_swiched: bool,
}

impl ActiveItem {
    /// Determines if the item is currently eligible to be switched.
    ///
    /// # Returns
    /// `true` if the item can be switched; `false` if switching is not permitted.
    ///
    /// # Example
    /// ```rust
    /// let item = ActiveItem { can_be_swiched: false };
    /// assert!(!item.can_switch());
    /// ```
    pub fn can_switch(&self) -> bool {
        self.can_be_swiched
    }
}

impl Default for ActiveItem {
    /// Creates a new `ActiveItem` with switching enabled by default.
    ///
    /// # Returns
    /// An `ActiveItem` instance with `can_be_swiched` set to `true`.
    ///
    /// # Example
    /// ```rust
    /// let item = ActiveItem::default();
    /// assert!(item.can_switch());
    /// ```
    fn default() -> Self {
        Self {
            can_be_swiched: true,
        }
    }
}
