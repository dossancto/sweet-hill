use bevy::{pbr, prelude::*};

use crate::gun_controller::{
    configure::GunsBag,
    domain::{ActiveGun, Gun},
};

pub(crate) fn switch_to_prev_gun(
    guns_bag: Res<GunsBag>,
    active_gun_query: Single<(Entity, &Gun), With<ActiveGun>>,
    mut commands: Commands,
) {
    let (active_gun_entity, gun) = active_gun_query.into_inner();

    let dont_have_enough_guns_to_toggle = guns_bag.guns.len() <= 1;

    if dont_have_enough_guns_to_toggle {
        return;
    }

    let prev_gun_index = get_new_indice(&guns_bag, &gun.id, false);

    let prev_gun_entity = guns_bag
        .guns
        .iter()
        .nth(prev_gun_index)
        .map(|(_, &entity)| entity);

    let Some(prev_gun_entity) = prev_gun_entity else {
        return;
    };

    let (prev_gun_entity, _) = prev_gun_entity;

    commands.entity(active_gun_entity).remove::<ActiveGun>();

    commands.entity(prev_gun_entity).insert(ActiveGun);
}

pub(crate) fn switch_to_next_gun(
    guns_bag: Res<GunsBag>,
    active_gun_query: Single<(Entity, &Gun), With<ActiveGun>>,
    mut commands: Commands,
) {
    let (active_gun_entity, gun) = active_gun_query.into_inner();

    let dont_have_enough_guns_to_toggle = guns_bag.guns.len() <= 1;

    if dont_have_enough_guns_to_toggle {
        return;
    }

    let prev_gun_index = get_new_indice(&guns_bag, &gun.id, true);

    let prev_gun_entity = guns_bag
        .guns
        .iter()
        .nth(prev_gun_index)
        .map(|(_, &entity)| entity);

    let Some(prev_gun_entity) = prev_gun_entity else {
        return;
    };

    let (prev_gun_entity, _) = prev_gun_entity;

    commands.entity(active_gun_entity).remove::<ActiveGun>();

    commands.entity(prev_gun_entity).insert(ActiveGun);
}

fn get_new_indice(guns_bag: &GunsBag, current_gun_id: &str, foward: bool) -> usize {
    let current_gun_index = guns_bag
        .guns
        .iter()
        .position(|(id, _)| *id == current_gun_id)
        .unwrap_or(0) as i8;

    let direction: i8 = if foward { 1 } else { -1 };

    let size = guns_bag.guns.len() as i8;

    let prev_gun_index = (current_gun_index + size + direction) % size;

    prev_gun_index as usize
}
