use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Cancel, Ongoing};
use gameplay_input::inputs::InteractAction;
use interaction::{
    interaction::components::{CanInteract, Interactable, Interacting},
    inventory::collect::collect_trigger::TriggerCollect,
};
use states::{
    interaction::{InteractionSourceCamera, InterationConfiguration},
    player::Player,
};
use third_party::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(active_interaction);
    app.add_observer(cancel_interaction);

    app.add_systems(FixedUpdate, load_interacting_items);
}

fn cancel_interaction(
    _on: On<Cancel<InteractAction>>,
    interactables_q: Query<(Entity, &mut Interacting)>,
    mut commands: Commands,
) {
    for (entity, interacting) in interactables_q.iter() {
        if interacting.time_to_interact.is_finished() {
            continue;
        }
        commands.entity(entity).remove::<Interacting>();
    }
}

fn active_interaction(
    _on: On<Ongoing<InteractAction>>,
    interactables_q: Query<(&CanInteract, Option<&Interacting>, &Interactable)>,
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

    let Ok((_is_hitteable, interacting, interactable)) = interactables_q.get(hit.entity) else {
        return;
    };

    if let Some(interacting) = interacting {
        if interacting.time_to_interact.is_finished() == false {
            return;
        }

        commands.entity(hit.entity).remove::<Interacting>();
        commands.trigger(TriggerCollect { entity: hit.entity });
    } else {
        commands
            .entity(hit.entity)
            .insert_if_new(Interacting::new(interactable.time_to_interact));
    }
}

fn load_interacting_items(mut interactables_q: Query<(Entity, &mut Interacting)>, time: Res<Time>) {
    for (_entity, mut interactable) in interactables_q.iter_mut() {
        if interactable.time_to_interact.is_finished() {
            continue;
        }
        interactable.time_to_interact.tick(time.delta());
    }
}
