use bevy::prelude::*;

pub trait CollectItemAction {
    type Output;

    fn get_collect_event(&self) -> Self::Output;
}

#[derive(EntityEvent, Debug)]
pub struct Collect<E> {
    pub entity: Entity,
    pub event: E,
}

impl<E> Collect<E> {
    pub fn new(event: E, entity: Entity) -> Self {
        Collect { event, entity }
    }
}
