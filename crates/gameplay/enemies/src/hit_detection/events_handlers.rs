use bevy::prelude::*;
use states::hittable::Hit;

use crate::{
    die_controller::domain::{EnemyDead, EnemyHit},
    states::Enemy,
};

pub fn process_enemy_damage(on: On<Hit>, mut query: Query<&mut Enemy>, mut commands: Commands) {
    let enemy = query.get_mut(on.target);

    let Ok(mut enemy) = enemy else {
        return;
    };

    if enemy.is_dead() {
        return;
    }

    commands.trigger(EnemyHit {
        target: on.target.clone(),
        damage: on.damage,
    });

    enemy.apply_damage(on.damage);

    if enemy.is_dead() {
        commands.trigger(EnemyDead {
            target: on.target.clone(),
        });
    }
}
