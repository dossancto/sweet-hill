use bevy::{ecs::system::SystemId, prelude::*};
use bevy_enhanced_input::prelude::Complete;
use gameplay_input::inputs::CollectItemInput;
use states::player::PlayerCamera;

use avian3d::prelude::*;
use third_party::avian3d::CollisionLayer;

use crate::{
    bag::components::InteractionConfiguration,
    take::components::{Collect, GunItem, PickableItem},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_collect_trigger);
    app.add_observer(handle);
}

fn handle(on: On<Collect<GunItem>>) {}

fn handle_collect_trigger(
    on: On<Complete<CollectItemInput>>,
    player: Single<(&GlobalTransform, &InteractionConfiguration), With<PlayerCamera>>,
    spatial_query: SpatialQuery,
    pickable_item_q: Query<(&PickableItem, &GunItem)>,
    mut commands: Commands,
) {
    let (player, pickup_actor) = player.into_inner();

    let camera_transform = player.compute_transform();

    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        camera_transform.forward(),
        pickup_actor.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Interactable),
    );

    let Some(hit) = hit else {
        return;
    };

    let Ok((hitted_pickable, gun_item)) = pickable_item_q.get(hit.entity) else {
        return;
    };

    commands.trigger(Collect {
        context: hit.entity,
        event: gun_item.clone(),
    });
}
