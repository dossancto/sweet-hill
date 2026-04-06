use bevy::{ecs::system::SystemId, platform::collections::HashMap, prelude::*};

use crate::{
    configuration::gun_components::*,
    firing::firing_types::bullet::{shoot_auto_bullets, shoot_semi_auto_bullets},
    models::gun_models::{gun_m4a1::definition::GunM4A1, spawn::spawn_gun},
    reload::domain::GunAmmo,
};

#[derive(Resource)]
pub struct GunBag {
    pub guns: HashMap<String, (Entity)>,
    pub max_guns: usize,
}

impl GunBag {
    pub fn add_gun(&mut self, name: String, entity: Entity) {
        if self.guns.len() < self.max_guns {
            self.guns.insert(name, (entity));
        }
    }

    /// Checks if there is an available slot for adding a new gun.
    ///
    /// Returns `true` if the current number of guns is less than the maximum allowed (`max_guns`),
    /// indicating that a new gun can be added. Returns `false` otherwise.
    pub fn guns_slot_available(&self) -> bool {
        self.guns.len() < self.max_guns
    }
}

// impl FromWorld for GunsBag {
//     fn from_world(world: &mut World) -> Self {
//         let mut guns_bag = GunsBag {
//             guns: HashMap::new(),
//             max_guns: 2,
//         };
//
//         let semi_auto_bullet_system = world.register_system(shoot_semi_auto_bullets);
//         let auto_bullet_system = world.register_system(shoot_auto_bullets);
//
//         guns_bag.guns.insert(
//             "pistol".into(),
//             (
//                 world
//                     .spawn((
//                         Gun {
//                             id: "pistol".to_string(),
//                             name: "pistol".to_string(),
//                             damage: 50.0,
//                             range: 100.0,
//                             damage_falloff_per_hit: 5f32,
//                         },
//                         GunM4A1,
//                         GunAmmo {
//                             max_clip_size: 30,
//                             max_stock_size: 90 * 10,
//                             current_on_clip: 30,
//                             current_on_stock: 90 * 10,
//                         },
//                         GunReload { reload_time: 1f32 },
//                         GunFireAuto::new(10f32, auto_bullet_system),
//                         ActiveGun,
//                         GunRecoil::new(GunRecoilOptions {
//                             vertical_recoil_range: 2.0..5.0,
//                             horizontal_recoil_range: -5f32..5f32,
//                             tension: 5.,
//                             friction: 6.,
//                             recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),
//                         }),
//                         GunAiming {
//                             aiming_time: 100,
//                             zoom_level: 60.0,
//                         },
//                     ))
//                     .observe(spawn_gun)
//                     .id(),
//                 auto_bullet_system,
//             ),
//         );
//
//         guns_bag.guns.insert(
//             "canon".into(),
//             (
//                 world
//                     .spawn((
//                         Gun {
//                             id: "canon".to_string(),
//                             name: "Canon".to_string(),
//                             damage: 100.0,
//                             range: 100.0,
//                             damage_falloff_per_hit: 10. / 100.,
//                         },
//                         GunAmmo {
//                             max_clip_size: 5,
//                             max_stock_size: 25,
//                             current_on_clip: 5,
//                             current_on_stock: 25,
//                         },
//                         GunReload { reload_time: 1. },
//                         GunFireSemiAuto::new(1.2f32, semi_auto_bullet_system),
//                         GunAiming {
//                             aiming_time: 300,
//                             zoom_level: 30.0,
//                         },
//                         GunRecoil::new(GunRecoilOptions {
//                             vertical_recoil_range: 20.0..50.0,
//                             horizontal_recoil_range: -5f32..5f32,
//                             tension: 5.,
//                             friction: 6.,
//                             recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),
//                         }),
//                     ))
//                     .id(),
//                 semi_auto_bullet_system,
//             ),
//         );
//
//         guns_bag
//     }
// }
