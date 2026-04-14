use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GunState {
    #[default]
    ReadyToFire,
    Reloading,
    Firing,
    Switching,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GunAimState {
    #[default]
    Idle,
    Aiming,
}

#[derive(Event)]
pub struct ProcessGunFire;
