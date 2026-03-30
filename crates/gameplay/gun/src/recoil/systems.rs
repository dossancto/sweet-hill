use bevy::prelude::*;

use crate::{
    configuration::gun_components::{ActiveGun, GunRecoil},
    firing::events::BulletGunFired,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        process_recoil_system.run_if(in_state(states::screens::Screen::Gameplay)),
    );

    app.add_observer(apply_gun_recoil);
}

pub fn apply_gun_recoil(
    _on: On<BulletGunFired>,
    mut recoil: Single<&mut GunRecoil, With<ActiveGun>>,
) {
    recoil.target_offset.y += 10.0;
    recoil.target_offset.x += rand::random::<f32>() * 2.0 - 1.0;
}

fn process_recoil_system(
    time: Res<Time>,
    mut recoil: Single<&mut GunRecoil, With<ActiveGun>>,
    mut visual_transform: Single<&mut Transform, With<Camera3d>>,
) {
    let delta = recoil.target_offset - recoil.current_offset;
    let spring_force = delta * recoil.tension;

    let damping_force = recoil.current_offset * recoil.friction;
    let velocity = (spring_force - damping_force) * time.delta_secs();

    recoil.current_offset += velocity;

    visual_transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        0.0,
        recoil.current_offset.y.to_radians(),
        recoil.current_offset.x.to_radians(),
    );

    recoil.target_offset = recoil
        .target_offset
        .lerp(Vec2::ZERO, time.delta_secs() * 10.0);
}
