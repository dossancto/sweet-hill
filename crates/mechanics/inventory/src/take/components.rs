use bevy::prelude::*;

#[derive(Event)]
pub struct ItemTaken {}

#[derive(Component)]
pub struct PickableItem {}

pub trait PickableItemAction {}

#[derive(EntityEvent)]
pub struct Collect<I: PickableItemAction> {
    #[event_target]
    pub context: Entity,
    pub event: I,
}

#[derive(Component, Clone)]
pub struct GunItem;

// pub fn trigger<'a>(&mut self, event: impl Event<Trigger<'a>: Default>) {
//     self.queue(command::trigger(event));
// }
