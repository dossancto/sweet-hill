use bevy::{ecs::system::SystemId, platform::collections::HashMap, prelude::*};

use crate::{
    configuration::gun_components::*,
    firing::firing_types::bullet::{shoot_auto_bullets, shoot_semi_auto_bullets},
    reload::domain::GunAmmo,
};

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

        let semi_auto_bullet_system = world.register_system(shoot_semi_auto_bullets);
        let auto_bullet_system = world.register_system(shoot_auto_bullets);

        guns_bag.guns.insert(
            "pistol".into(),
            (
                world
                    .spawn((
                        GunAutoBundle::new(
                            Gun {
                                id: "pistol".to_string(),
                                name: "pistol".to_string(),
                                damage: 50.0,
                                range: 100.0,
                                damage_falloff_per_hit: 5f32,
                            },
                            GunAmmo {
                                max_clip_size: 30,
                                max_stock_size: 90 * 10,
                                current_on_clip: 30,
                                current_on_stock: 90 * 10,
                            },
                            GunReload { reload_time: 0.5 },
                            GunFireAuto::new(10f32, auto_bullet_system),
                        ),
                        ActiveGun,
                        GunRecoil::default(),
                        GunAiming {
                            aiming_time: 100,
                            zoom_level: 60.0,
                        },
                    ))
                    .id(),
                auto_bullet_system,
            ),
        );

        guns_bag.guns.insert(
            "canon".into(),
            (
                world
                    .spawn(GunSemiAutoBundle::new(
                        Gun {
                            id: "canon".to_string(),
                            name: "Canon".to_string(),
                            damage: 100.0,
                            range: 100.0,
                            damage_falloff_per_hit: 10. / 100.,
                        },
                        GunAmmo {
                            max_clip_size: 5,
                            max_stock_size: 25,
                            current_on_clip: 5,
                            current_on_stock: 25,
                        },
                        GunReload { reload_time: 1. },
                        GunFireSemiAuto::new(1.2f32, semi_auto_bullet_system),
                    ))
                    .id(),
                semi_auto_bullet_system,
            ),
        );

        guns_bag
    }
}
