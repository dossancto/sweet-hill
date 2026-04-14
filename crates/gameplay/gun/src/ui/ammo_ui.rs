use bevy::prelude::*;
use states::inventory::active_item::ActiveItem;

use crate::{configuration::gun_components::GunAmmoText, reload::domain::GunAmmo};

pub(super) fn draw_ammo_on_screen(
    text: Single<&mut Text, With<GunAmmoText>>,
    gun: Single<&GunAmmo, With<ActiveItem>>,
) {
    let mut text = text.into_inner();
    let ammo = gun.into_inner();

    text.0 = format!("{}/{}", ammo.current_on_clip, ammo.current_on_stock);
}

pub(super) fn init_ammo_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Ammo Text"),
        GunAmmoText,
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            right: px(12),
            ..default()
        },
    ));
}
