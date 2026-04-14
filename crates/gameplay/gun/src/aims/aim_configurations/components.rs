use bevy::prelude::*;

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
