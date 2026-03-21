use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct SwayItem {
    pub sensitivity: f32,
    pub smoothness: f32,
    pub max_sway: f32,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, item_sway_system);
}

fn item_sway_system(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &SwayItem)>,
    time: Res<Time>,
) {
    let delta = mouse_motion.delta;

    for (mut transform, sway) in query.iter_mut() {
        let raw_x = -delta.y * sway.sensitivity;
        let raw_y = -delta.x * sway.sensitivity;

        let target_x = raw_x.clamp(-sway.max_sway, sway.max_sway);
        let target_y = raw_y.clamp(-sway.max_sway, sway.max_sway);

        let target_rotation = Quat::from_euler(EulerRot::YXZ, target_y, target_x, 0.0);

        transform.rotation = transform.rotation.slerp(
            target_rotation,
            (sway.smoothness * time.delta_secs()).min(1.0),
        );
    }
}
