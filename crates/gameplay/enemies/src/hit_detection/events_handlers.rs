use bevy::prelude::*;
use states::hittable::Hit;

use crate::states::Enemy;

pub fn process_enemy_damage(on: On<Hit>, mut query: Query<&mut Enemy>, mut commands: Commands) {
    let enemy = query.get_mut(on.target);

    if enemy.is_err() {
        return;
    }

    let mut enemy = enemy.unwrap();

    if enemy.health <= 0. {
        info!("Enemy already dead, ignoring hit.");
        return;
    }

    let new_health = enemy.health - on.damage;

    if new_health <= 0. {
        enemy.health = 0.;
        // TODO: Trigger event
        info!("Enemy killed!");
    } else {
        enemy.health = new_health;
        info!("Enemy hit! Remaining health: {}", new_health);
    }
}
