use bevy::prelude::*;
use rand::Rng;
use states::inventory::active_item::ActiveItem;

use crate::{firing::events::BulletGunFired, recoil::configurations::components::GunRecoil};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        process_recoil_system.run_if(in_state(states::screens::Screen::Gameplay)),
    );

    app.add_observer(apply_gun_recoil);
}

pub fn apply_gun_recoil(
    _on: On<BulletGunFired>,
    mut recoil: Single<&mut GunRecoil, With<ActiveItem>>,
) {
    let mut rng = rand::rng();

    recoil.recovery_timer.reset();

    let horizontal_kick = rng.random_range(recoil.horizontal_recoil_range.clone());
    let vertical_kick = rng.random_range(recoil.vertical_recoil_range.clone());

    recoil.target_offset.x += horizontal_kick;
    recoil.target_offset.y += vertical_kick;
}

fn process_recoil_system(
    time: Res<Time>,
    mut recoil: Single<&mut GunRecoil, With<ActiveItem>>,
    mut camera_transform: Single<&mut Transform, With<Camera3d>>,
) {
    let dt = time.delta_secs();

    recoil.recovery_timer.tick(time.delta());

    if recoil.recovery_timer.is_finished() {
        recoil.target_offset = recoil.target_offset.lerp(Vec3::ZERO, dt * 10.0);
    }

    let delta = recoil.target_offset - recoil.current_offset;
    let spring_force = delta * recoil.tension;
    let damping_force = recoil.current_offset * recoil.friction;

    recoil.current_offset += (spring_force - damping_force) * dt;

    camera_transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        recoil.current_offset.x.to_radians(),
        recoil.current_offset.y.to_radians(),
        0.0,
    );
}
