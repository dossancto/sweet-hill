use bevy::prelude::*;

pub trait CollectItemAction {
    type Output;

    fn get_collect_event(&self) -> Self::Output;
}

#[derive(Event, Debug)]
pub struct Collect<E>
{
    pub event: E,
}

impl<E> From<E> for Collect<E>
{
    fn from(event: E) -> Self {
        Collect { event }
    }
}
