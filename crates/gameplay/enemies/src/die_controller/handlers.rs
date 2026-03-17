use bevy::prelude::*;
use states::hittable::Hittable;

use crate::die_controller::domain::{DeadEnemy, EnemyDead};

pub(super) fn on_enemy_dies(on: On<EnemyDead>, mut commands: Commands) {
    commands.entity(on.target).insert(DeadEnemy);
    commands.entity(on.target).remove::<Hittable>();

    info!("Enemy {} died!", on.target);

    // TODO: play death animation
}
