#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use inventory_core::{Collect, CollectItemAction};
    use inventory_macros::CollectTrigger;

    #[derive(CollectTrigger, Component)]
    #[collect_event(MoneyCollected)]
    pub struct Money;

    #[derive(Debug, Default)]
    pub struct MoneyCollected {}

    #[derive(Resource, Default)]
    struct ObserverRan(bool);

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<ObserverRan>();

        app.add_observer(
            |event: On<Collect<MoneyCollected>>, mut ran: ResMut<ObserverRan>| {
                ran.0 = true;
            },
        );

        let money = Money;

        let event = money.get_collect_event();

        let event_to_trigger = Collect::from(event);

        app.world_mut().trigger(event_to_trigger);

        app.update();

        let ran = app.world().resource::<ObserverRan>();

        assert!(
            ran.0,
            "Observer did not run, expected to receive MoneyCollected event"
        );
    }
}
