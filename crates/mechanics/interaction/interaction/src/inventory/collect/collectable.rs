use bevy::prelude::*;

#[derive(Component)]
pub struct Collectable;

impl Default for Collectable {
    fn default() -> Self {
        Self
    }
}

pub trait CanBeCollect: Component + Clone {}
