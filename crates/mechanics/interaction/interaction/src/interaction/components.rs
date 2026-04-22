use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable {
    pub blocked: bool,
}

impl Default for Interactable {
    fn default() -> Self {
        Self { blocked: false }
    }
}

#[derive(Component)]
pub struct CanInteract;
