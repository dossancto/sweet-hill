use bevy::prelude::*;
use third_party::bevy_trenchbroom::GetTrenchbroomModelPath;
use utils::asset_tracking::LoadResource;

use crate::models::gun_models::rin::{actions::spawn_rin_model, definition::GunModelRin};

mod actions;
pub mod definition;

pub(super) fn plugin(app: &mut App) {
    app.load_asset::<Gltf>(GunModelRin::model_path());

    app.add_observer(spawn_rin_model);
}
