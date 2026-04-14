use bevy::prelude::*;

/// Marker component indicating that an entity can be picked up by the player or other game systems.
///
/// Attach this component to any entity that should be interactable as a pickable item within the game world.
/// Systems responsible for item pickup or inventory management will typically query for entities with this marker.
#[derive(Component)]
pub struct PickableItem;
