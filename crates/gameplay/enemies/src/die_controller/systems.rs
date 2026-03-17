use bevy::prelude::*;
use states::player::PlayerCamera;

use crate::die_controller::domain::DeadEnemy;

// Define the "Visible Angle" based on FOV (roughly 0.5 for a standard 90-degree FOV)
// A dot product of 0.0 means the target is 90 degrees to the side.
// A dot product of 1.0 means the target is dead center.
const VISIBILITY_THRESHOLD: f32 = 0.3;

const DISTANCE_THRESHOLD: f32 = 50.0;

/// Despawns all enemy entities that have been marked as dead and are not visible.
///
/// # Arguments
///
/// * `dead_enemies` - A query that retrieves all entities with the `DeadEnemey` component and their `ViewVisibility`, indicating they are dead and their visibility status.
/// * `commands` - The Bevy `Commands` resource used to issue entity despawn commands.
///
/// # Behavior
///
/// This function iterates over all entities returned by the `dead_enemies` query. If an entity is not visible (`ViewVisibility.get()` returns false), it is despawned using the Bevy ECS system. Visible dead enemies are skipped.
/// After calling this function, all dead and invisible enemies will be removed from the world.
///
/// # Usage
///
/// Typically called in a system after updating enemy states, to clean up dead and invisible enemies.
pub(super) fn despawn_offscreen_corpses(
    mut commands: Commands,
    camera_q: Single<&GlobalTransform, With<PlayerCamera>>,
    mut enemy_q: Query<(Entity, &GlobalTransform), With<DeadEnemy>>,
) {
    let camera_transform = camera_q;

    // Get the forward vector of the camera
    let camera_forward = camera_transform.forward();
    let camera_pos = camera_transform.translation();

    for (entity, enemy_transform) in enemy_q.iter_mut() {
        let enemy_pos = enemy_transform.translation();

        let distance = camera_pos.distance(enemy_pos);

        if distance > DISTANCE_THRESHOLD {
            commands.entity(entity).despawn();
            continue;
        }

        let dir_to_enemy = (enemy_pos - camera_pos).normalize();

        let dot = camera_forward.dot(dir_to_enemy);

        let visible = dot > VISIBILITY_THRESHOLD;

        if visible {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
