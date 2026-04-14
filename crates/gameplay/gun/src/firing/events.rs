use bevy::ecs::event::Event;

use crate::configuration::gun_components::Gun;

#[derive(Event)]
pub struct BulletGunFired {
    pub gun: Gun,
}
