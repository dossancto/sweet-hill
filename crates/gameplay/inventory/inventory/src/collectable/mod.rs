use bevy::prelude::*;

#[derive(Component)]
pub struct Collectable;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect_action::Collect;

    #[derive(Component, Clone)]
    struct Money {
        pub amount: f64,
    }

    #[derive(Resource, Default)]
    struct PlayerWallet(f64);

    #[derive(Event)]
    struct TriggerCollect {
        pub entity: Entity,
    }

    #[test]
    fn test_get_event_returns_money_collected() {
        let mut app = App::new();
        app.init_resource::<PlayerWallet>();

        app.add_observer(|on: On<Collect<Money>>, mut wallet: ResMut<PlayerWallet>| {
            wallet.0 = wallet.0 + on.value.amount;
        });

        fn collect_item(
            on: On<TriggerCollect>,
            mut commands: Commands,
            query: Query<&Collectable>,
        ) {
            let entity = on.entity;

            let Ok(collectable) = query.get(entity) else {
                panic!("Entity does not have a Collectable<Money> component");
            };

            let value = "123";

            let event_to_trigger = Collect::new(value, entity.clone());

            commands.trigger(event_to_trigger);
        }

        app.add_observer(collect_item);

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
