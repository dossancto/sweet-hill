use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;
use gameplay_input::inputs::ToogleActiveItem;
use states::inventory::{active_item::ActiveItem, items::PickableItem};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(switch_to_next_gun);
    app.add_observer(hide_gun);
    app.add_observer(show_gun);
    app.add_systems(Update, log_active_gun);
}

fn log_active_gun(active_item: Single<Entity, With<ActiveItem>>) {
    let active_item_entity = active_item.into_inner();

    info!("Active gun entity: {:?}", active_item_entity);
}

fn switch_to_next_gun(
    _on: On<Start<ToogleActiveItem>>,
    guns_q: Query<Entity, (Without<ActiveItem>, With<PickableItem>)>,
    active_gun_query: Single<(Entity, &ActiveItem)>,
    mut commands: Commands,
) {
    let (active_gun_entity, active_item) = active_gun_query.into_inner();

    if active_item.can_switch() == false {
        info!("Cannot switch active gun right now.");
        return;
    }

    let next_gun_entity = guns_q.iter().next().map(|entity| entity);

    if let Some(next_gun_entity) = next_gun_entity {
        info!(
            "Switching active gun from {:?} to {:?}.",
            active_gun_entity, next_gun_entity
        );
        commands.entity(active_gun_entity).remove::<ActiveItem>();
        commands
            .entity(next_gun_entity)
            .insert(ActiveItem::default());
    }
}

fn hide_gun(
    on: On<Remove, ActiveItem>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now inactive, hiding it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Hidden);
        return;
    };

    if *visibility == Visibility::Hidden {
        return;
    }

    *visibility = Visibility::Hidden;

    return;
}

fn show_gun(
    on: On<Add, ActiveItem>,
    mut commands: Commands,
    mut visibility_q: Query<&mut Visibility>,
) {
    info!("Gun is now actieve, showing it.");

    let Ok(mut visibility) = visibility_q.get_mut(on.entity) else {
        commands.entity(on.entity).insert(Visibility::Visible);
        return;
    };

    if *visibility == Visibility::Visible {
        return;
    }

    *visibility = Visibility::Visible;

    return;
}
