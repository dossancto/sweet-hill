use std::time::Duration;

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
    pub time_to_interact: Duration,

    /// Determines whether the interaction timer should reset when an interaction is cancelled.
    ///
    /// If `true`, cancelling an interaction will reset the timer, requiring the full interaction time to be completed again for the next attempt.
    /// If `false`, the timer will not reset, allowing the player to resume the interaction without
    /// starting over.
    pub should_reset_interaction_timer_on_cancel: bool,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            blocked: false,
            time_to_interact: Duration::from_secs(0),
            should_reset_interaction_timer_on_cancel: true,
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
    pub fn new(duration: Duration) -> Self {
        Self {
            time_to_interact: Timer::from_seconds(duration.as_secs_f32(), TimerMode::Once),
        }
    }
}
