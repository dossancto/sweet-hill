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
    pub fn can_switch(&self) -> bool {
        self.can_be_swiched
    }
}

impl Default for ActiveItem {
    fn default() -> Self {
        Self {
            can_be_swiched: true,
        }
    }
}
