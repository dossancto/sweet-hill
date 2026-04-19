# Inventory Module

This crate provides a small, event-driven collectable system for Bevy ECS. It defines:

- A `Collectable` marker component for entities that can be picked up.
- A `CanBeCollect` trait to tag components that represent collectable values.
- A `Collect<E>` event that carries the collected value and the entity.
- A `TriggerCollect` event and helpers to emit `Collect<E>` for any registered type.

The core idea: you tag entities with both a collectable component (like `Money`) and the `Collectable` marker, then trigger a `TriggerCollect` event. The crate looks up the first matching collectable type and fires `Collect<E>` with the component value.

## Basic usage

Register collectable types on the `App`, then listen for `Collect<E>` events.

```rust
use bevy::prelude::*;
use inventory::{
    collect::{AddCollectable, TriggerCollect},
    collect_action::Collect,
    collectable::{CanBeCollect, Collectable},
};

#[derive(Component, Clone)]
struct Money {
    amount: f64,
}

#[derive(Resource, Default)]
struct PlayerWallet(f64);

impl CanBeCollect for Money {}

fn main() {
    let mut app = App::new();
    app.init_resource::<PlayerWallet>();

    // Handle collect events.
    app.add_observer(|on: On<Collect<Money>>, mut wallet: ResMut<PlayerWallet>| {
        wallet.0 += on.value.amount;
    });

    // Register Money as a collectable type.
    app.add_collectable::<Money>();

    let spawned_money = app.world_mut().spawn((Money { amount: 10.0 }, Collectable)).id();

    // Ask the system to collect the entity.
    app.world_mut().trigger(TriggerCollect {
        entity: spawned_money,
    });

    app.update();
}
```

## Multiple collectable types

You can register as many collectable types as you need. Each type gets its own `Collect<E>` event stream.

```rust
use bevy::prelude::*;
use inventory::{
    collect::{AddCollectable, TriggerCollect},
    collect_action::Collect,
    collectable::{CanBeCollect, Collectable},
};

#[derive(Component, Clone)]
struct Money {
    amount: f64,
}

#[derive(Component, Clone)]
struct Gun {
    name: String,
}

impl CanBeCollect for Money {}
impl CanBeCollect for Gun {}

fn main() {
    let mut app = App::new();

    app.add_observer(|on: On<Collect<Money>>| {
        let _amount = on.value.amount;
        let _entity = on.entity;
    });

    app.add_observer(|on: On<Collect<Gun>>| {
        let _name = &on.value.name;
        let _entity = on.entity;
    });

    app.add_collectable::<Money>();
    app.add_collectable::<Gun>();

    let money_entity = app.world_mut().spawn((Money { amount: 5.0 }, Collectable)).id();
    let gun_entity = app
        .world_mut()
        .spawn((Gun { name: "Blaster".to_string() }, Collectable))
        .id();

    app.world_mut().trigger(TriggerCollect { entity: money_entity });
    app.world_mut().trigger(TriggerCollect { entity: gun_entity });

    app.update();
}
```

## Notes

- `Collect<E>` is an `EntityEvent` and includes the collecting entity id and value.
- The collect flow only triggers for entities with both `Collectable` and a registered `CanBeCollect` component.
- Use `AddCollectable` to register new collectable types on the app.
