use bevy::{ecs::system::SystemId, prelude::*};

use crate::reload::domain::GunAmmo;

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
    pub damage_falloff_per_hit: f32,
    pub range: f32,
}

#[derive(Component, Debug)]
pub struct GunReload {
    pub reload_time: f32,
}

/// Component representing the aiming state of a gun. Guns that dont implement this component will
/// not be able to aim, and will always have the same zoom level.
///
/// - `aiming_time`: The time spent aiming, measured in milliseconds (ms).
/// - `zoom_level`: The current zoom level, measured in degrees.
#[derive(Component, Debug)]
pub struct GunAiming {
    /// Time spent aiming, in milliseconds (ms).
    pub aiming_time: u32,
    /// Current zoom level, in degrees.
    pub zoom_level: f32,
}

#[derive(Component, Debug)]
pub struct GunReloading {
    pub time_to_reload: Timer,
}

#[derive(Component, Debug)]
pub struct GunFireAuto {
    pub delay_after_shoot: f32, // Rounds per second
    pub last_shot_time: f32,    // Rounds per second
    pub action: SystemId,
}
impl GunFireAuto {
    pub fn new(rounds_per_sec: f32, action: SystemId) -> Self {
        Self {
            delay_after_shoot: 1000f32 / rounds_per_sec / 1000f32,
            last_shot_time: 0.0,
            action,
        }
    }
}

#[derive(Component, Debug)]
pub struct GunFireSemiAuto {
    pub delay_after_shoot: f32, // Rounds per second
    pub last_shot_time: f32,    // Rounds per second
    pub system_id: SystemId,
}
impl GunFireSemiAuto {
    pub fn new(delay_after_shoot: f32, system_id: SystemId) -> Self {
        Self {
            delay_after_shoot,
            last_shot_time: 0.0,
            system_id,
        }
    }
}

#[derive(Bundle)]
pub struct GunSemiAutoBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub config: GunFireSemiAuto,
}

impl GunSemiAutoBundle {
    pub fn new(
        gun: Gun,
        ammo: GunAmmo,
        reload: GunReload,
        semi_auto_config: GunFireSemiAuto,
    ) -> Self {
        Self {
            gun,
            ammo,
            reload,
            config: semi_auto_config,
        }
    }
}

#[derive(Bundle)]
pub struct GunAutoBundle {
    pub gun: Gun,
    pub ammo: GunAmmo,
    pub reload: GunReload,
    pub config: GunFireAuto,
}

impl GunAutoBundle {
    pub fn new(gun: Gun, ammo: GunAmmo, reload: GunReload, auto_config: GunFireAuto) -> Self {
        Self {
            gun,
            ammo,
            reload: reload,
            config: auto_config,
        }
    }
}

/// `GunRecoil` is a component that simulates the recoil effect for a gun in a game.
///
/// This component is intended to be attached to entities representing guns or weapons.
/// It tracks the current and target recoil offsets, as well as the physical properties
/// (tension and friction) that control how the recoil effect behaves over time.
///
/// # Fields
///
/// - `current_offset`: The current recoil offset applied to the gun (e.g., for visual kickback).
/// - `target_offset`: The target recoil offset the gun is moving towards (e.g., after firing).
/// - `tension`: The spring tension that pulls the gun back to its neutral position. Higher values make the gun return faster.
/// - `friction`: The damping factor that slows down the recoil movement. Higher values make the recoil settle more quickly.
///
/// Typically, a system will update `current_offset` each frame to approach `target_offset`,
/// using the `tension` and `friction` values to control the spring-like behavior.
#[derive(Component)]
pub struct GunRecoil {
    /// The current recoil offset applied to the gun (e.g., for visual kickback).
    pub current_offset: Vec3,
    /// The target recoil offset the gun is moving towards (e.g., after firing).
    pub target_offset: Vec3,
    /// The spring tension that pulls the gun back to its neutral position. Higher values make the gun return faster.
    pub tension: f32,
    /// The damping factor that slows down the recoil movement. Higher values make the recoil settle more quickly.
    pub friction: f32,
    pub recovery_timer: Timer,
    pub fov_kick: f32,
    pub current_fov_offset: f32,
}

impl Default for GunRecoil {
    fn default() -> Self {
        Self {
            current_offset: Vec3::ZERO,
            target_offset: Vec3::ZERO,
            tension: 5.0,
            friction: 6.0,
            recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),
            fov_kick: 2.0,
            current_fov_offset: 0.0,
        }
    }
}
