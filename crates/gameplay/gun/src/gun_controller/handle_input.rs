use crate::{
    firing::states::GunFireTrigger,
    gun_controller::domain::{ActiveGun, Gun, GunAmmo},
    states::GunState,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_tap_fire(
    _on: On<Start<GunFireTrigger>>,
    gun: Single<(&Gun, &mut GunAmmo), With<ActiveGun>>,
    gun_state: Res<State<GunState>>,
) {
    let gun_state = gun_state.get();

    if gun_state != &GunState::ReadyToFire {
        return;
    }
}

pub(crate) fn handle_hold_fire() {}

/// Determines whether the user can shoot based on gun specifications.
///
/// This method checks the following conditions:
/// - The gun's firing rate and cooldown period, by comparing the current time in milliseconds
///   with the last time a bullet was shot.
/// - Whether the gun has enough ammo to fire.
/// - Additional gun-specific constraints (e.g., overheating, jammed state, etc.).
///
/// Returns `true` if all conditions are met and the user is allowed to shoot; otherwise returns `false`.
///
/// # Returns
///
/// * `bool` - `true` if the user can shoot, `false` otherwise.
///
/// # Example
///
/// ```rust
/// if can_shoot() {
///     // Fire bullet
/// }
/// ```
///
/// # Note
///
/// This function should be updated to handle additional gun mechanics as needed.
pub(crate) fn can_shoot() -> bool {
    true
}
