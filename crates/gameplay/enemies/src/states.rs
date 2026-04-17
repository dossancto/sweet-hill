use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy {
    pub name: String,
    pub health: f32,
    pub damage: f32,
}

impl Enemy {
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_dead(&self) -> bool {
        self.is_alive() == false
    }

    pub fn apply_damage(&mut self, damage: f32) {
        self.health -= damage;

        if self.health < 0.0 {
            self.health = 0.0;
        }
    }
}
