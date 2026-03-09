//! Player pickup UI interactions.
//! In particular, change the crosshair when the player is looking at a prop and hide it when the player is holding a prop.

use std::any::Any as _;

use avian_pickup::{actor::AvianPickupActor, prop::HeldProp};
use avian3d::prelude::*;
use bevy::{prelude::*, sprite::Text2dShadow};

use crate::{
    PostPhysicsAppSystems,
    gameplay::{
        crosshair::CrosshairState,
        player::{
            camera::PlayerCamera,
            pickup::state::{CanBePickedUp, PickupPrompt, PlayerPickupState},
        },
    },
    screens::Screen,
    third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_for_pickup_opportunity
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
    app.add_observer(hide_crosshair_when_picking_up);
    app.add_observer(show_crosshair_when_not_picking_up);
    app.add_observer(how_point_at_pickup_opportunity);
    app.add_observer(hide_point_at_pickup_opportunity);
}

fn how_point_at_pickup_opportunity(on: On<Add, CanBePickedUp>, mut commands: Commands) {
    let text_justification = Justify::Center;

    // commands.entity(on.entity).insert();

    commands.spawn((
        Text2d::new("pickup"),
        // text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        TextBackgroundColor(Color::BLACK.with_alpha(0.5)),
        Text2dShadow::default(),
        PickupPrompt,
    ));
}

fn hide_point_at_pickup_opportunity(
    on: On<Remove, CanBePickedUp>,
    mut commands: Commands,
    q_point: Query<Entity, (With<Text2d>, With<PickupPrompt>)>,
) {
    for entity in q_point.iter() {
        commands.entity(entity).despawn();
    }

    commands.entity(on.entity).remove::<PointLight>();
}

fn check_for_pickup_opportunity(
    player: Single<(&GlobalTransform, &AvianPickupActor), With<PlayerCamera>>,
    spatial_query: SpatialQuery,
    can_be_picked_up: Query<Entity, With<CanBePickedUp>>,
    mut commands: Commands,
    mut crosshair: Single<&mut CrosshairState>,
    mut pickup_state: Single<&mut PlayerPickupState>,
) {
    let (player, pickup_actor) = player.into_inner();
    let camera_transform = player.compute_transform();
    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        camera_transform.forward(),
        pickup_actor.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Prop),
    );

    let system_id = check_for_pickup_opportunity.type_id();

    for entity in can_be_picked_up.iter() {
        commands.entity(entity).remove::<CanBePickedUp>();
    }

    let Some(hit) = hit else {
        crosshair.wants_square.remove(&system_id);
        pickup_state.pickup_opportunity.remove(&system_id);

        return;
    };

    commands.entity(hit.entity).insert(CanBePickedUp);

    crosshair.wants_square.insert(system_id);
    pickup_state.pickup_opportunity.insert(system_id);
}

fn hide_crosshair_when_picking_up(
    _on: On<Add, HeldProp>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    crosshair
        .wants_invisible
        .insert(hide_crosshair_when_picking_up.type_id());
}

fn show_crosshair_when_not_picking_up(
    _on: On<Remove, HeldProp>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    crosshair
        .wants_invisible
        .remove(&hide_crosshair_when_picking_up.type_id());
}
