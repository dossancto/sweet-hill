use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Event)]
pub(crate) struct EnemyHit {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Event)]
pub(crate) struct EnemyDead {
    pub target: Entity,
}

#[derive(Component)]
pub(crate) struct DeadEnemy;
