#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use inventory_core::CollectItemAction;
    use inventory_macros::Collect;

    #[derive(Collect, Component)]
    #[collect_event(MoneyCollected)]
    pub struct A;

    #[derive(Debug)]
    pub struct MoneyCollected {}

    impl Default for MoneyCollected {
        fn default() -> Self {
            MoneyCollected {}
        }
    }

    #[test]
    fn test_get_event_returns_money_collected() {
        let a = A;
        let event = a.get_event();
        // Assuming get_event returns a type that can be compared to MoneyCollected
        // If get_event returns a reference or a trait object, adjust accordingly
        let expected = MoneyCollected::default();
        // If PartialEq is not implemented, you may need to implement it or use pattern matching
        assert_eq!(format!("{:?}", event), format!("{:?}", expected));
    }
}
