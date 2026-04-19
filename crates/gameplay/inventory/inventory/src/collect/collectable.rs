use bevy::prelude::*;

#[derive(Component)]
pub struct Collectable;

pub trait CanBeCollect: Component + Clone {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{AddCollectable, TriggerCollect, collect_action::Collect};

    #[derive(Component, Clone)]
    struct Money {
        pub amount: f64,
    }

    #[derive(Component, Clone)]
    struct Gun {
        pub _name: String,
    }

    #[derive(Resource, Default)]
    struct PlayerWallet(f64);
    impl CanBeCollect for Money {}
    impl CanBeCollect for Gun {}

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<PlayerWallet>();

        app.add_observer(|on: On<Collect<Money>>, mut wallet: ResMut<PlayerWallet>| {
            wallet.0 = wallet.0 + on.value.amount;
        });

        app.add_collectable::<Money>();
        app.add_collectable::<Gun>();

        let spawned_money = {
            let money = Money { amount: 10.0 };

            app.world_mut().spawn((money.clone(), Collectable)).id()
        };

        app.world_mut().trigger(TriggerCollect {
            entity: spawned_money.clone(),
        });

        app.update();

        let wallet = app.world().resource::<PlayerWallet>();

        assert_eq!(wallet.0, 10.0);
    }
}
