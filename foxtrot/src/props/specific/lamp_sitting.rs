use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::{
    app::Propagate,
    asset::io::embedded::GetAssetServer as _,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    light::NotShadowCaster,
    prelude::*,
};

use bevy_trenchbroom::prelude::*;
use interaction::{
    interaction::components::Interactable,
    inventory::collect::{
        app::AddCollectable,
        collect_action::Collect,
        collectable::{CanBeCollect, Collectable},
    },
};
use third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel};
use utils::asset_tracking::LoadResource;

use crate::{
    props::setup::quake_bundle, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

pub(super) fn plugin(app: &mut App) {
    app.load_asset::<Gltf>(LampSitting::model_path());
    app.add_observer(on_collect_lamp_sitting);
    app.add_collectable::<LampSitting>();
}

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
    )
)]
#[derive(Clone)]
#[component(on_add = setup_lamp_sitting)]
pub(crate) struct LampSitting;

impl CanBeCollect for LampSitting {}

fn on_collect_lamp_sitting(
    on: On<Collect<LampSitting>>,
    mut commands: Commands,
) {
    commands.entity(on.entity).despawn();
}

fn setup_lamp_sitting(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let asset_server = world.get_asset_server().clone();

        let model = asset_server.load_trenchbroom_model::<LampSitting>();

        let bundle = (
            ColliderDensity(1_000.0),
            Collectable::default(),
            Collider::cuboid(0.34, 0.35, 0.65),
            CollisionLayers::new(
                [CollisionLayer::Interactable, CollisionLayer::Hittable],
                LayerMask::ALL,
            ),
            RigidBody::Dynamic,
            Interactable::default(),
            SceneRoot(model),
        );

        world
            .entity_mut(ctx.entity)
            // The prop should be held upright.
            .insert((
                bundle,
                NotShadowCaster,
                Propagate(NotShadowCaster),
                PreferredPickupRotation(Quat::IDENTITY),
            ))
            // The lamp's origin is at the bottom of the lamp, so we need to offset the light a bit.
            .with_child((
                Transform::from_xyz(0.0, 0.0, 0.0),
                PointLight {
                    color: Color::srgb(1.0, 0.7, 0.4),
                    intensity: 40_000.0,
                    radius: 0.05,
                    shadows_enabled: true,
                    ..default()
                },
            ));
    });
}
