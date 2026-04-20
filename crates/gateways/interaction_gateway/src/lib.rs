use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use interaction::interaction::components::{CanInteract, Interactable};
use states::{
    interaction::{InteractionSourceCamera, InterationConfiguration},
    player::Player,
};
use third_party::avian3d::CollisionLayer;

pub fn plugin(app: &mut App) {
    // TODO: Add some flag to only run this system if the level is running.
    // It also should be possible to disable this system.
    app.add_systems(Update, can_interact);
    app.add_observer(on_add_can_interact);
    app.add_observer(on_remove_can_interact);
}

fn on_add_can_interact(on: On<Add, CanInteract>) {
    info!("Can interact with {:?}", on.entity);
}

fn on_remove_can_interact(on: On<Remove, CanInteract>) {
    info!("CANT {:?}", on.entity);
}

fn can_interact(
    interactables_q: Query<Entity, With<Interactable>>,
    can_interact_q: Query<Entity, With<CanInteract>>,
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

    for entity in can_interact_q.iter() {
        commands.entity(entity).remove::<CanInteract>();
    }

    let Some(hit) = hit else {
        return;
    };

    // let Ok(_is_hitteable) = interactables_q.get(hit.entity) else {
    //     return;
    // };

    commands.entity(hit.entity).insert(CanInteract);
}
