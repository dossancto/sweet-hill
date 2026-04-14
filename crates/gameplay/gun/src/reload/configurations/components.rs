use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GunReload {
    pub reload_time: f32,
}

#[derive(Component, Debug)]
pub struct GunReloading {
    pub time_to_reload: Timer,
}
