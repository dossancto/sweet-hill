use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{Character, prelude::*};

use bevy_trenchbroom::prelude::*;
use enemies::states::Enemy;
use interaction::{
    interaction::components::Interactable, inventory::collect::collectable::Collectable,
};
use states::hittable::Hittable;
use utils::asset_tracking::LoadResource;

use crate::third_party::{
    avian3d::CollisionLayer,
    bevy_trenchbroom::{GetTrenchbroomModelPath as _, LoadTrenchbroomModel as _},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_crate_small);
    app.add_observer(setup_crate_big);
    app.load_asset::<Gltf>(CrateBig::model_path())
        .load_asset::<Gltf>(CrateSmall::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate01_big.gltf")
)]
// #[component(on_add = setup_prop::<CrateBig>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct CrateBig;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate01_small.gltf")
)]
pub(crate) struct CrateSmall;

fn setup_crate_big(
    add: On<Add, CrateBig>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let model = asset_server.load_trenchbroom_model::<CrateBig>();
    commands.entity(add.entity).insert((
        Hittable,
        CollisionLayers::new(CollisionLayer::Hittable, LayerMask::ALL),
        ColliderDensity(1_000.0),
        Collider::cuboid(1.5f32, 1.5f32, 1.5f32),
        Enemy {
            name: "Crate BIg".to_string(),
            health: 500000.0,
            damage: 0.0,
        },
    ));
    commands.entity(add.entity).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius: 0.5 },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });
    commands.entity(add.entity).insert((
        // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
        SceneRoot(model),
        // The prop should be held upright.
        PreferredPickupRotation(Quat::IDENTITY),
        // Holding a big crate right in your face looks bad, so
        // let's move the pickup distance a bit further away.
        RigidBody::Dynamic,
    ));
}

fn setup_crate_small(
    add: On<Add, CrateSmall>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let model = asset_server.load_trenchbroom_model::<CrateSmall>();
    commands.entity(add.entity).insert((
        Hittable,
        CollisionLayers::new(
            [CollisionLayer::Interactable, CollisionLayer::Hittable],
            LayerMask::ALL,
        ),
        ColliderDensity(1_000.0),
        Collider::cuboid(1f32, 1f32, 1f32),

        // This makes the raycast colides with other things, resulting in the player not being able
        // to pick up the crate.
        // ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
        //     .with_default_layers(CollisionLayers::new(
        //         CollisionLayer::Hittable,
        //         LayerMask::ALL,
        //     ))
        //     .with_default_density(1_000.0),
        Collectable,
        Interactable::default(),
        Enemy {
            name: "Crate Small".to_string(),
            health: 500.0,
            damage: 0.0,
        },
    ));
    commands.entity(add.entity).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius: 0.5 },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });
    commands.entity(add.entity).insert((
        // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
        SceneRoot(model),
        // The prop should be held upright.
        PreferredPickupRotation(Quat::IDENTITY),
        // Holding a big crate right in your face looks bad, so
        // let's move the pickup distance a bit further away.
        RigidBody::Dynamic,
    ));
}
