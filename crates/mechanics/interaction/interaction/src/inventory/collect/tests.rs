#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::inventory::collect::{
        app::AddCollectable,
        collect_action::Collect,
        collect_trigger::TriggerCollect,
        collectable::{CanBeCollect, Collectable},
    };

    #[derive(Component, Clone)]
    pub struct Money {
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

        let money = Money { amount: 10.0 };

        let spawned_money = app.world_mut().spawn(money.clone()).id();

        let event_to_trigger = Collect::new(money, spawned_money.clone());

        app.world_mut().trigger(event_to_trigger);

        app.update();

        let wallet = app.world().resource::<PlayerWallet>();

        assert_eq!(wallet.0, 10.0);
    }

    #[test]
    fn test_get_event_returns_money_collected_with_collectable() {
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
