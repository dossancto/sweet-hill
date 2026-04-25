use bevy::prelude::*;

pub mod app;
pub mod collect_action;
pub mod collect_trigger;
pub mod collectable;
pub mod collected;

#[cfg(test)]
mod tests;

pub(super) fn plugin(_app: &mut App) {}
