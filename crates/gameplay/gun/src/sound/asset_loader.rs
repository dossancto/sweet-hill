use bevy::prelude::*;
use bevy_seedling::sample::AudioSample;
use utils::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GunSoundsAssets>();
}

#[derive(Resource, Asset, Reflect, Clone)]
pub(crate) struct GunSoundsAssets {
    #[dependency]
    pub bullet_fire_hit: Handle<AudioSample>,
}

impl GunSoundsAssets {
    pub(crate) const BULLET_FIRE_HIT: &'static str = "audio/sound_effects/hit/hit01.wav";
}

impl FromWorld for GunSoundsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bullet_fire_hit: assets.load(Self::BULLET_FIRE_HIT),
        }
    }
}
