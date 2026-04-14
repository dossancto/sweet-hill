use bevy::prelude::*;
use states::inventory::active_item::ActiveItem;

pub mod gun_components;
pub mod guns_definitions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(guns_definitions::plugin);

    app.add_observer(assert_unique_active_gun);
}

fn assert_unique_active_gun(
    _add: On<Insert, ActiveItem>,
    active_guns: Query<Entity, With<ActiveItem>>,
    mut commands: Commands,
) {
    for active_gun in active_guns.iter().skip(1) {
        commands.entity(active_gun).remove::<ActiveItem>();
    }
}
