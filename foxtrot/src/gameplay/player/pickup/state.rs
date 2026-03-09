use std::any::TypeId;

use avian_pickup::prop::HeldProp;
use bevy::{platform::collections::HashSet, prelude::*};

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub(crate) struct PlayerPickupState {
    pub(crate) pickup_opportunity: HashSet<TypeId>
}

#[derive(Component)]
pub(crate) struct CanBePickedUp;

pub(crate) fn is_holding_prop(q_prop: Query<&HeldProp>) -> bool {
    !q_prop.is_empty()
}

#[derive(Component)]
pub(crate) struct PickupPrompt;
