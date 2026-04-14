/// Calculates the required delta_value per frame to reach the target_fov in the given duration (in milliseconds).
///
/// # Arguments
/// * `default_fov` - The starting field of view (in radians)
/// * `target_fov` - The desired field of view to reach (in radians)
/// * `duration_ms` - The duration over which to reach the target_fov (in milliseconds)
/// * `frame_time_ms` - The time per frame (in milliseconds)
///
/// # Returns
/// The delta_value value to apply per frame.
pub(crate) fn calculate_delta_value(
    default_value: f32,
    target_value: f32,
    duration_ms: u32,
    frame_time_ms: f32,
) -> f32 {
    let total_frames = (duration_ms as f32 / frame_time_ms).max(1.0);
    (target_value - default_value) / total_frames
}
