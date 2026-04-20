use bevy::prelude::*;

/// Marker component indicating that an entity serves as the camera source for interaction events.
///
/// Attach this component to a camera entity to enable it as the origin for user interaction detection,
/// such as raycasting or picking. Only entities with this marker will be considered when determining
/// the camera context for interactive systems.
#[derive(Component)]
pub struct InteractionSourceCamera;

#[derive(Component)]
pub struct InterationConfiguration {
    pub interaction_distance: f32,
}
