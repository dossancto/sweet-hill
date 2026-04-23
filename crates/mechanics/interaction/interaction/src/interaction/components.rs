use bevy::prelude::*;

/// Represents an entity that can be interacted with in the game world.
///
/// Attach this component to entities that should respond to player or system interactions.
#[derive(Component)]
pub struct Interactable {
    /// Indicates whether interaction with this entity is currently prevented.
    ///
    /// When `true`, attempts to interact with the entity will be ignored or denied.
    /// When `false`, the entity is available for interaction.
    pub blocked: bool,

    /// The minimum amount of time, in Seconds, required to complete an interaction with this entity.
    ///
    /// Use this value to implement timed interactions, such as holding a button or waiting for an action to finish.
    /// Must be a non-negative floating-point value. Negative values may result in undefined behavior.
    pub time_to_interact: f32,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            blocked: false,
            time_to_interact: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CanInteract;

#[derive(Component)]
pub struct Interacting {
    pub time_to_interact: Timer,
}

impl Interacting {
    pub fn new(seconds_to_complete: f32) -> Self {
        Self {
            time_to_interact: Timer::from_seconds(seconds_to_complete, TimerMode::Once),
        }
    }
}
