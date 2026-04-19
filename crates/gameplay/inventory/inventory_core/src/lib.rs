use bevy::ecs::entity::Entity;

pub trait CollectItemAction {
    type Output;

    fn get_event(&self) -> Self::Output;
}
