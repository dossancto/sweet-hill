#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use inventory_core::CollectItemAction;
    use inventory_macros::Collect;

    #[derive(Collect, Component)]
    #[collect_event(MoneyCollected)]
    pub struct Money;

    #[derive(Debug, Default)]
    pub struct MoneyCollected {}

    #[test]
    fn test_get_event_returns_money_collected() {
        let money = Money;

        let event = money.get_collect_event();

        let expected = MoneyCollected::default();

        assert_eq!(format!("{:?}", event), format!("{:?}", expected));
    }
}
