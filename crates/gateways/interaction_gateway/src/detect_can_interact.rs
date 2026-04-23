use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use interaction::interaction::components::{CanInteract, Interactable, Interacting};
use states::{
    interaction::{InteractionSourceCamera, InterationConfiguration},
    player::Player,
};
use third_party::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    // TODO: Add some flag to only run this system if the level is running.
    // It also should be possible to disable this system.
    app.add_systems(FixedUpdate, can_interact);
}

fn can_interact(
    interactables_q: Query<(&Interactable, Option<&Interacting>)>,
    mut can_interact_q: Query<Entity, With<CanInteract>>,
    interation_configuration: Single<&InterationConfiguration>,
    player: Single<(&GlobalTransform, Entity), With<InteractionSourceCamera>>,
    spatial_query: SpatialQuery,
    player_collider: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let (player, _) = player.into_inner();

    let camera_transform = player.compute_transform();

    let interation_configuration = interation_configuration.into_inner();

    let forward = camera_transform.forward();

    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        forward,
        interation_configuration.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Interactable)
            .with_excluded_entities([*player_collider]),
    );

    for entity in can_interact_q.iter_mut() {
        if let Some(hit) = hit {
            if hit.entity == entity {
                continue;
            }
        }
        commands.entity(entity).remove::<CanInteract>();
    }

    let Some(hit) = hit else {
        return;
    };

    let Ok((interactable_item, interacting)) = interactables_q.get(hit.entity) else {
        return;
    };

    if interactable_item.blocked {
        return;
    }

    commands.entity(hit.entity).insert_if_new(CanInteract);
    // if let Some(interacting) = interacting {
    //     if interacting.time_to_interact.is_finished() == false {
    //         return;
    //     }
    // } else {
    //     commands
    //         .entity(hit.entity)
    //         .insert_if_new(Interacting::new(interactable_item.time_to_interact));
    // }
}
