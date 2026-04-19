use bevy::prelude::*;
use states::inventory::active_item::ActiveItem;

use crate::inventory::{collect::collected::CollectedItem, switch_item::inputs::NextItemAction};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(switch_next_item);
}

fn switch_next_item(
    _on: On<NextItemAction>,
    collected_items: Query<Entity, (Without<ActiveItem>, With<CollectedItem>)>,
    active_item_q: Query<Entity, With<ActiveItem>>,
    mut commands: Commands,
) {
    let Some(next_item_entity) = collected_items.iter().next() else {
        return;
    };

    for active_item_entity in active_item_q.iter() {
        commands.entity(active_item_entity).remove::<ActiveItem>();
    }

    commands
        .entity(next_item_entity)
        .insert(ActiveItem::default());
}
