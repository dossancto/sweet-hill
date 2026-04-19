use bevy::ecs::entity::Entity;

pub trait CollectItemAction {
    type Output;

    fn get_collect_event(&self) -> Self::Output;
}
