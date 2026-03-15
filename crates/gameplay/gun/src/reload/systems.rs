use crate::{
    configuration::gun_components::{ActiveGun, Gun, GunAmmo, GunReload, GunReloading},
    reload::domain::GunReloadTrigger,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(crate) fn handle_gun_reload(
    _on: On<Start<GunReloadTrigger>>,
    gun: Single<(&Gun, Entity, &mut GunAmmo), (With<ActiveGun>, Without<GunReloading>)>,
    mut commands: Commands,
) {
    let (_gun, entity, mut ammo) = gun.into_inner();

    commands.entity(entity).insert(GunReloading);

    let left_ammo = ammo.magazine_size - ammo.current_ammo;

    ammo.current_ammo = ammo.magazine_size;
    ammo.current_stock_ammo = ammo.current_stock_ammo - left_ammo;

    commands.entity(entity).remove::<GunReloading>();
}
