use bevy::{prelude::*, sprite::Text2dShadow};

use crate::{
    configuration::gun_components::{ActiveGun, GunAmmoText},
    reload::domain::GunAmmo,
};

pub(super) fn draw_ammo_on_screen(
    text: Single<&mut Text2d, With<GunAmmoText>>,
    gun: Single<&GunAmmo, With<ActiveGun>>,
) {
    let mut text = text.into_inner();
    let ammo = gun.into_inner();

    text.0 = format!("{}/{}", ammo.current_on_clip, ammo.current_on_stock);
}

pub(super) fn init_ammo_text(mut commands: Commands) {
    let text_justification = Justify::Left;
    commands.spawn((
        Text2d::new("0/0"),
        TextLayout::new_with_justify(text_justification),
        TextBackgroundColor(Color::BLACK.with_alpha(0.5)),
        Text2dShadow::default(),
        GunAmmoText,
    ));
}
