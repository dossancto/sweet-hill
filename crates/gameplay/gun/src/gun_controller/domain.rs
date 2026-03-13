use bevy::{ecs::system::SystemId, platform::collections::HashMap, prelude::*};
use states::world;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmoText;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ActiveGun;

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct Gun {
    pub id: String,
    pub name: String,
    pub damage: f32,
    pub range: f32,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmo {
    pub magazine_size: usize,
    pub stock_size: usize,
    pub current_ammo: usize,
    pub current_stock_ammo: usize,
}

#[derive(Resource)]
pub struct GunsBag {
    pub guns: HashMap<String, Entity>,
    pub max_guns: usize,
}

impl FromWorld for GunsBag {
    fn from_world(world: &mut World) -> Self {
        let mut guns_bag = GunsBag {
            guns: HashMap::new(),
            max_guns: 2,
        };

        guns_bag.guns.insert(
            "pistol".into(),
            world
                .spawn((
                    GunAmmo {
                        magazine_size: 30,
                        stock_size: 90,
                        current_ammo: 30,
                        current_stock_ammo: 90,
                    },
                    Gun {
                        id: "pistol".to_string(),
                        name: "Pistol".to_string(),
                        damage: 10.0,
                        range: 50.0,
                    },
                    ActiveGun,
                ))
                .id(),
        );

        guns_bag.guns.insert(
            "canon".into(),
            world
                .spawn((
                    GunAmmo {
                        magazine_size: 1,
                        stock_size: 10,
                        current_ammo: 1,
                        current_stock_ammo: 10,
                    },
                    Gun {
                        id: "canon".to_string(),
                        name: "Canon".to_string(),
                        damage: 100.0,
                        range: 50.0,
                    },
                ))
                .id(),
        );

        guns_bag
    }
}
