use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hittable {
    pub health: f32,
}

#[derive(Event)]
pub struct Hit {
    pub damage: f32,
    pub target: Entity,
}
