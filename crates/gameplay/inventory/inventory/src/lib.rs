#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use inventory_core::{Collect, CollectItemAction};
    use inventory_macros::CollectTrigger;

    #[derive(CollectTrigger, Component)]
    #[collect_event(MoneyCollected)]
    pub struct Money;

    #[derive(Debug, Default)]
    pub struct MoneyCollected;

    #[derive(Resource, Default)]
    struct PlayerWallet(f64);

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<PlayerWallet>();

        app.add_observer(
            |event: On<Collect<MoneyCollected>>, mut wallet: ResMut<PlayerWallet>| {
                wallet.0 = wallet.0 + 100.0;
            },
        );

        let money = Money;

        let event = money.get_collect_event();

        let event_to_trigger = Collect::from(event);

        app.world_mut().trigger(event_to_trigger);

        app.update();

        let wallet = app.world().resource::<PlayerWallet>();

        assert_eq!(wallet.0, 100.0);
    }
}
