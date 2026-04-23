use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Complete;
use gameplay_input::inputs::InteractAction;
use interaction::{
    interaction::components::CanInteract, inventory::collect::collect_trigger::TriggerCollect,
};
use states::{
    interaction::{InteractionSourceCamera, InterationConfiguration},
    player::Player,
};
use third_party::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(active_interaction);
}

fn active_interaction(
    _on: On<Complete<InteractAction>>,
    interactables_q: Query<&CanInteract>,
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

    let Some(hit) = hit else {
        return;
    };

    let Ok(_is_hitteable) = interactables_q.get(hit.entity) else {
        return;
    };

    commands.trigger(TriggerCollect { entity: hit.entity });
}
