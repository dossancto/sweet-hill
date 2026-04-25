use bevy::prelude::*;

/// Marker component indicating that an entity has been collected by the player or another system.
///
/// Attach this component to entities to represent that they have been collected. Systems can query
/// for entities with `CollectedItem` to perform logic specific to collected items, such as updating
/// the player's inventory, triggering events, or removing the entity from the game world.
#[derive(Component)]
pub struct CollectedItem;
