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

#[derive(Component, Debug)]
pub struct GunAimPosition {
    pub target_position: Vec3,
    pub target_rotation: Quat,
}

impl Default for GunAimPosition {
    fn default() -> Self {
        GunAimPosition {
            target_position: Vec3::new(-0.08f32, -0.3, 0.0),
            target_rotation: Quat::from_euler(
                EulerRot::XYZ,
                -5.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
        }
    }
}
