use bevy::prelude::*;
use states::world::PostPhysicsAppSystems;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Flicker {
    pub base_intensity: f32,
    pub frequency: f32,
    pub percentage: f32,
}

impl Default for Flicker {
    fn default() -> Self {
        Flicker {
            base_intensity: 1.0,
            frequency: 20.0,
            percentage: 0.1,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (flicker_light_pointlight, flicker_light_spotlight)
            .run_if(in_state(states::screens::Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::Update),
    );
}

pub fn flicker_light_pointlight(
    time: Res<Time>,
    mut query: Query<(&mut PointLight, &Flicker), With<Flicker>>,
) {
    for (mut light, flicker_config) in &mut query {
        let flickers_per_second = flicker_config.frequency;
        let flicker_percentage = flicker_config.percentage;
        let flicker = (time.elapsed_secs() * flickers_per_second).sin();
        let base_intensity = flicker_config.base_intensity;
        light.intensity = base_intensity + flicker * base_intensity * flicker_percentage;
    }
}

pub fn flicker_light_spotlight(
    time: Res<Time>,
    mut query: Query<(&mut SpotLight, &Flicker), With<Flicker>>,
) {
    for (mut light, flicker_config) in &mut query {
        let flickers_per_second = 20.0;
        let flicker_percentage = 0.9;
        let flicker = (time.elapsed_secs() * flickers_per_second).sin();
        let base_intensity = flicker_config.base_intensity;
        light.intensity = base_intensity + flicker * base_intensity * flicker_percentage;
    }
}
