use bevy::prelude::*;

#[derive(Component)]
pub struct Collectable;

pub trait CanBeCollect: Component + Clone {}
