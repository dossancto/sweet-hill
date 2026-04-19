#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use inventory_core::{Collect, CollectItemAction};
    use inventory_macros::CollectTrigger;

    #[derive(CollectTrigger, Component, Clone)]
    #[collect_event(MoneyCollected)]
    pub struct Money {
        pub amount: f64,
    }

    #[derive(Debug, Default)]
    pub struct MoneyCollected;

    #[derive(Resource, Default)]
    struct PlayerWallet(f64);

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<PlayerWallet>();

        app.add_observer(
            |on: On<Collect<MoneyCollected>>,
             mut wallet: ResMut<PlayerWallet>,
             money_q: Query<&Money>| {
                let Ok(money) = money_q.get(on.entity) else {
                    panic!("Money entity not found");
                };
                wallet.0 = wallet.0 + money.amount;
            },
        );

        let money = Money { amount: 10.0 };

        let spawned_money = app.world_mut().spawn(money.clone()).id();

        // Money may be get from Query<>
        let event = money.get_collect_event();

        let event_to_trigger = Collect::new(event, spawned_money.clone());

        app.world_mut().trigger(event_to_trigger);

        app.update();

        let wallet = app.world().resource::<PlayerWallet>();

        assert_eq!(wallet.0, 10.0);
    }
}
