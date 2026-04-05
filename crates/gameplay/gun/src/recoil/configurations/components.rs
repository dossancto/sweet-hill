use std::ops::Range;

use bevy::prelude::*;

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
/// Example usage:
///
/// ```rust
/// let gun_recoil = GunRecoil::new(GunRecoilOptions {
///     vertical_recoil_range: 2.0..5.0,,
///     horizontal_recoil_range: -5f32..5f32,,
///     tension: 5.,
///     friction: 6.,
///     recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),
/// });
/// ```
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

    pub vertical_recoil_range: Range<f32>,

    pub horizontal_recoil_range: Range<f32>,
}

impl Default for GunRecoil {
    fn default() -> Self {
        Self {
            current_offset: Vec3::ZERO,
            target_offset: Vec3::ZERO,
            tension: 5.0,
            friction: 6.0,
            recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),
            vertical_recoil_range: 5.0f32..10.0f32,
            horizontal_recoil_range: -3.0f32..3.0f32,
        }
    }
}

impl GunRecoil {
    pub fn new(options: GunRecoilOptions) -> Self {
        Self {
            current_offset: Vec3::ZERO,
            target_offset: Vec3::ZERO,
            tension: options.tension,
            friction: options.friction,
            recovery_timer: options.recovery_timer,
            vertical_recoil_range: options.vertical_recoil_range,
            horizontal_recoil_range: options.horizontal_recoil_range,
        }
    }
}

pub struct GunRecoilOptions {
    pub tension: f32,
    pub friction: f32,
    pub vertical_recoil_range: Range<f32>,
    pub horizontal_recoil_range: Range<f32>,
    pub recovery_timer: Timer,
}
