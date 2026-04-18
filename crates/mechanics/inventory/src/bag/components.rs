use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryBagConfiguration {
    pub max_capacity: usize,
}

impl InventoryBagConfiguration {
    pub fn new(max_capacity: usize) -> Self {
        Self { max_capacity }
    }
}

#[derive(Component)]
pub struct InteractionConfiguration {
    pub interaction_distance: f32,
}
