use bevy::prelude::*;

use bevy_enhanced_input::prelude::{Cancel, Ongoing};
use states::guns::marks::GunHolderMark;

use crate::inputs::GunAimTrigger;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(center_gun_on_screen);
    app.add_observer(return_gun_to_default_position);
}

fn center_gun_on_screen(
    _on: On<Ongoing<GunAimTrigger>>,
    gun_holder: Single<&mut Transform, With<GunHolderMark>>,
) {
    let mut gun_transform = gun_holder.into_inner();

    let player_forward = gun_transform.forward().normalize();
    let mut gun_offset = player_forward * 1. + Vec3::new(0.0, -0.3, 0.0);
    gun_offset.x = -0.08f32;
    gun_transform.translation = gun_offset;
    gun_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        -5.0f32.to_radians(),
        0.0f32.to_radians(),
        0.0f32.to_radians(),
    );
}

fn return_gun_to_default_position(
    _on: On<Cancel<GunAimTrigger>>,

    gun_holder: Single<&mut Transform, With<GunHolderMark>>,
) {
    let mut gun_transform = gun_holder.into_inner();

    let player_forward = gun_transform.forward();
    let gun_offset = player_forward * 1.5 + Vec3::new(0.4, -0.5, 0.0);

    gun_transform.translation = gun_offset;

    gun_transform.rotation = Quat::from_rotation_y(7.5f32.to_radians());
}
