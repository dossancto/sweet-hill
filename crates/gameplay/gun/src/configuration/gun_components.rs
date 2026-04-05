use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmoText;

/// Marker component indicating that an entity currently has an active gun.
///
/// This component can be used to identify which entity is currently wielding or using a gun.
/// It does not contain any data, and is typically used in systems to filter or query entities
/// with an active gun state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ActiveGun;

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct Gun {
    pub id: String,
    pub name: String,
    pub damage: f32,
    pub damage_falloff_per_hit: f32,
    pub range: f32,
}
