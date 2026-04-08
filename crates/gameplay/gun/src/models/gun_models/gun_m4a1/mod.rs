use bevy::prelude::*;
use third_party::bevy_trenchbroom::GetTrenchbroomModelPath;
use utils::asset_tracking::LoadResource;

use crate::models::gun_models::gun_m4a1::{
    actions::{on_add_m4a1, spawn_m4a1_model},
    definition::GunModelM4A1,
};

mod actions;
pub mod definition;

pub(super) fn plugin(app: &mut App) {
    app.load_asset::<Gltf>(GunModelM4A1::model_path());

    app.add_observer(spawn_m4a1_model);
}
