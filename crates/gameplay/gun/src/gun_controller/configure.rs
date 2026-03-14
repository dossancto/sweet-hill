use bevy::{ecs::system::SystemId, platform::collections::HashMap, prelude::*};

use crate::{firing::firing_types::bullet::shoot_bullets, gun_controller::domain::*};

#[derive(Resource)]
pub struct GunsBag {
    pub guns: HashMap<String, (Entity, SystemId)>,
    pub max_guns: usize,
}

impl FromWorld for GunsBag {
    fn from_world(world: &mut World) -> Self {
        let mut guns_bag = GunsBag {
            guns: HashMap::new(),
            max_guns: 2,
        };

        let regular_shoot_system_id = world.register_system(shoot_bullets);

        guns_bag.guns.insert(
            "pistol".into(),
            (
                world
                    .spawn((
                        GunBundle::new(
                            Gun {
                                id: "pistol".to_string(),
                                name: "pistol".to_string(),
                                damage: 50.0,
                                range: 100.0,
                            },
                            GunAmmo {
                                magazine_size: 30,
                                stock_size: 90,
                                current_ammo: 30,
                                current_stock_ammo: 90,
                            },
                        ),
                        ActiveGun,
                    ))
                    .id(),
                regular_shoot_system_id,
            ),
        );

        guns_bag.guns.insert(
            "canon".into(),
            (
                world
                    .spawn(GunBundle::new(
                        Gun {
                            id: "canon".to_string(),
                            name: "Canon".to_string(),
                            damage: 50.0,
                            range: 100.0,
                        },
                        GunAmmo {
                            magazine_size: 5,
                            stock_size: 25,
                            current_ammo: 5,
                            current_stock_ammo: 25,
                        },
                    ))
                    .id(),
                regular_shoot_system_id,
            ),
        );

        guns_bag
    }
}
