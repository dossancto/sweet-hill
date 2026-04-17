use bevy::prelude::*;
use states::{
    guns::marks::GunHolderMark,
    inventory::{active_item::ActiveItem, items::Item},
};

use crate::{bag::components::InventoryBagConfiguration, take::components::TakeItem};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_gun_replacement);
}

pub fn handle_gun_replacement(
    on: On<TakeItem>,
    mut commands: Commands,

    gun_holder_q: Single<Entity, With<GunHolderMark>>,
    items_bag: Single<&InventoryBagConfiguration>,
    active_guns_query: Query<Entity, With<ActiveItem>>,
    guns_q: Query<Entity, With<Item>>,
) {
    let len = guns_q.iter().len();

    if len >= items_bag.max_capacity {
        let current_gun_entity = active_guns_query.iter().next().unwrap();
        commands.entity(current_gun_entity).despawn();
    }

    for active_gun in active_guns_query.iter() {
        commands.entity(active_gun).remove::<ActiveItem>();
    }

    let gun_holder = gun_holder_q.into_inner();

    // let spawned_gun = on.gun_to_spawn.spawn(&mut commands, gun_holder);
    //
    // commands.entity(spawned_gun).insert(ActiveItem::default());
}
