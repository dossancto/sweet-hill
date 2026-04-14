use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy {
    pub name: String,
    pub health: f32,
    pub damage: f32,
}
